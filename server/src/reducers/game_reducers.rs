use crate::models::game::game;
use crate::models::{Game, Occupant, SpotState};
use crate::scoring::analyze_game;
use crate::seed::seed_sample_games;
use crate::utils::{apply_move_to_board, coord_to_index};
use serde_json;
use spacetimedb::{reducer, ReducerContext, Table};
use std::convert::TryInto;

/// Default board size when not specified by the user
const DEFAULT_BOARD_SIZE: u8 = 9;

/// Creates a new Go game with optional custom board size and handicap.
///
/// # Arguments
/// * `ctx` - The reducer context containing sender identity and timestamp
/// * `board_size` - Optional board size (defaults to 9×9 if not specified)
/// * `handicap` - Optional handicap stones to place for the black player
///
/// # Returns
/// * `Ok(())` - Game was created successfully
/// * `Err(String)` - Error message if the creation failed
#[reducer]
pub fn create_game(
    ctx: &ReducerContext,
    board_size: Option<u8>,
    handicap: Option<u8>,
) -> Result<(), String> {
    let size = board_size.unwrap_or(DEFAULT_BOARD_SIZE);

    // Use the timestamp as a unique game ID
    let game_id: u64 = ctx
        .timestamp
        .to_micros_since_unix_epoch()
        .try_into()
        .unwrap();

    // Create an empty board with all positions set to empty
    let mut board: Vec<SpotState> = (0..(size as usize * size as usize))
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        })
        .collect();

    // Apply handicap if requested (pre-place black stones)
    let handicap = handicap.unwrap_or(0).min(9); // Cap at 9 handicap stones

    if handicap > 0 {
        // Calculate handicap stone positions based on standard Go patterns
        let handicap_positions = match size {
            9 => vec![(2, 2), (6, 6), (2, 6), (6, 2), (4, 4)],
            13 => vec![(3, 3), (9, 9), (3, 9), (9, 3), (6, 6)],
            19 => vec![(3, 3), (15, 15), (15, 3), (3, 15), (9, 9)],
            _ => vec![], // No handicap for non-standard board sizes
        };

        // Place the handicap stones on the board
        for i in 0..(handicap as usize).min(handicap_positions.len()) {
            let (x, y) = handicap_positions[i];
            let idx = coord_to_index(x, y, size as usize);
            board[idx].occupant = Occupant::Black;
            board[idx].move_number = Some(ctx.timestamp.to_micros_since_unix_epoch() as u64);
        }
    }

    // If handicap is used, White goes first; otherwise Black goes first
    let turn = if handicap > 0 {
        "W".to_string()
    } else {
        "B".to_string()
    };

    // Serialize the board to JSON for storage
    let board_json = serde_json::to_string(&board).unwrap();

    // Insert the new game into the database
    ctx.db.game().insert(Game {
        id: game_id,
        player_black: ctx.sender,
        player_white: None, // Will be filled when a second player joins
        board: board_json,
        turn,
        passes: 0,
        board_size: size,
        previous_board: None,
        game_over: false,
        final_score_black: None,
        final_score_white: None,
    });

    log::info!("Created game with id: {}", game_id);
    Ok(())
}

/// Allows a player to join an existing game.
///
/// # Arguments
/// * `ctx` - The reducer context containing sender identity
/// * `game_id` - The ID of the game to join
///
/// # Returns
/// * `Ok(())` - Successfully joined the game
/// * `Err(String)` - Error message if joining failed
#[reducer]
pub fn join_game(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    if let Some(mut game) = ctx.db.game().id().find(game_id) {
        // Check if the game already has two players
        if game.player_white.is_some() {
            // Allow the existing players to rejoin
            if ctx.sender != game.player_black && ctx.sender != game.player_white.unwrap() {
                return Err("Game already has two players".to_string());
            }
            return Ok(());
        }

        // Don't allow same player as both black and white
        if ctx.sender == game.player_black {
            return Err("You are already in this game as Black".to_string());
        }

        // Join as White
        game.player_white = Some(ctx.sender);
        ctx.db.game().id().update(game);
        log::info!("Player {} joined game {}", ctx.sender, game_id);
        Ok(())
    } else {
        Err(format!("Game with id {} not found", game_id))
    }
}

/// Player passes their turn.
///
/// In Go, a player can choose to "pass" instead of placing a stone.
/// If both players pass consecutively, the game ends.
///
/// # Arguments
/// * `ctx` - The reducer context containing sender identity
/// * `game_id` - The ID of the game
///
/// # Returns
/// * `Ok(())` - Pass was successful
/// * `Err(String)` - Error message if the pass failed
#[reducer]
pub fn pass_move(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err(format!("Game with id {} not found", game_id)),
    };

    // Verify the game isn't already over
    if game.game_over {
        return Err("Game is already over".to_string());
    }

    // Verify it's the sender's turn
    let is_sender_turn = match game.turn.as_str() {
        "B" => ctx.sender == game.player_black,
        "W" => game
            .player_white
            .map(|pw| ctx.sender == pw)
            .unwrap_or(false),
        _ => false,
    };

    if !is_sender_turn {
        return Err("It's not your turn".to_string());
    }

    // Increment pass counter
    game.passes += 1;

    // If both players have passed, end the game
    if game.passes >= 2 {
        game.game_over = true;
    } else {
        // Switch turns
        game.turn = if game.turn == "B" { "W" } else { "B" }.to_string();
    }

    // Analyze the game and update scores
    game = analyze_game(game);

    // Update the game state
    ctx.db.game().id().update(game);
    Ok(())
}

/// Places a stone on the board at the specified coordinates.
///
/// This is the main gameplay action that handles:
/// - Validating the move is legal
/// - Placing the stone
/// - Capturing any surrounded opponent stones
/// - Switching turns
/// - Checking for game end conditions
///
/// # Arguments
/// * `ctx` - The reducer context containing sender identity
/// * `game_id` - The ID of the game
/// * `x` - The x-coordinate (column) for the stone
/// * `y` - The y-coordinate (row) for the stone
///
/// # Returns
/// * `Ok(())` - Stone was placed successfully
/// * `Err(String)` - Error message if the placement failed
#[reducer]
pub fn place_stone(ctx: &ReducerContext, game_id: u64, x: u8, y: u8) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err(format!("Game with id {} not found", game_id)),
    };

    // Verify the game isn't already over
    if game.game_over {
        return Err("Game is already over".to_string());
    }

    // Verify the game has two players
    if game.player_white.is_none() {
        return Err("Waiting for second player to join".to_string());
    }

    // Determine which stone color the sender is playing
    let stone_color = if ctx.sender == game.player_black {
        Occupant::Black
    } else if Some(ctx.sender) == game.player_white {
        Occupant::White
    } else {
        return Err("You are not a player in this game".to_string());
    };

    // Verify it's the sender's turn
    let is_sender_turn = match game.turn.as_str() {
        "B" => stone_color == Occupant::Black,
        "W" => stone_color == Occupant::White,
        _ => false,
    };

    if !is_sender_turn {
        return Err("It's not your turn".to_string());
    }

    // Convert board from JSON string to a vector of SpotState
    let board_result: Result<Vec<SpotState>, _> = serde_json::from_str(&game.board);
    let board = match board_result {
        Ok(b) => b,
        Err(_) => return Err("Failed to parse game board".to_string()),
    };

    // Apply the move to the board
    let (_new_board, new_board_str) = apply_move_to_board(
        board,
        game.board_size as usize,
        stone_color,
        x as usize,
        y as usize,
        game.previous_board.clone(),
        ctx.timestamp.to_micros_since_unix_epoch() as u64,
    )?;

    // Update the game state
    game.board = new_board_str.unwrap();
    game.previous_board = Some(game.board.clone());
    game.turn = if game.turn == "B" { "W" } else { "B" }.to_string();
    game.passes = 0; // Reset pass counter after a stone is placed

    // Analyze the game and update scores
    game = analyze_game(game);

    // Update the game in the database
    ctx.db.game().id().update(game);
    Ok(())
}

/// Seeds the database with sample games for demonstration purposes.
///
/// This reducer creates pre-configured game boards to showcase the game's features.
///
/// # Arguments
/// * `ctx` - The reducer context
///
/// # Returns
/// * `Ok(())` - Sample games were created successfully
/// * `Err(String)` - Error message if seeding failed
#[reducer]
pub fn seed(ctx: &ReducerContext) -> Result<(), String> {
    seed_sample_games(ctx);
    Ok(())
}
