use crate::models::game::game;
use crate::models::{Game, Occupant, SpotState};
use serde_json;
use spacetimedb::{ReducerContext, Table};

/// Create a sample game with some stones already placed for demonstration purposes.
pub fn seed_sample_games(ctx: &ReducerContext) {
    // Create a basic demo board
    let board_size: u8 = 9;
    let board_size_usize = board_size as usize;

    // Create a board with some stones already placed
    let mut board = vec![
        SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        };
        board_size_usize * board_size_usize
    ];

    // Place some sample stones
    // Create a simple configuration with a black group and a white group
    let black_positions = vec![(3, 3), (3, 4), (4, 3), (4, 4), (2, 3)];
    let white_positions = vec![(1, 3), (2, 2), (3, 2), (4, 2), (5, 3)];

    for (idx, &(x, y)) in black_positions.iter().enumerate() {
        let index = y * board_size_usize + x;
        board[index].occupant = Occupant::Black;
        board[index].move_number = Some(idx as u64 * 2 + 1); // Odd move numbers for black
    }

    for (idx, &(x, y)) in white_positions.iter().enumerate() {
        let index = y * board_size_usize + x;
        board[index].occupant = Occupant::White;
        board[index].move_number = Some(idx as u64 * 2 + 2); // Even move numbers for white
    }

    let board_json = serde_json::to_string(&board).unwrap();

    // Add the sample game to the database
    let game_id = ctx.timestamp.to_micros_since_unix_epoch() as u64;
    ctx.db.game().insert(Game {
        id: game_id,
        player_black: ctx.sender,
        player_white: Some(ctx.sender), // Same player as both to keep it simple
        board: board_json,
        turn: "B".to_string(),
        passes: 0,
        board_size,
        previous_board: None,
        game_over: false,
        final_score_black: None,
        final_score_white: None,
    });

    // Create another sample game with a different configuration
    let board_size: u8 = 13;
    let board_size_usize = board_size as usize;

    // Create a second board with a different configuration
    let mut board2 = vec![
        SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        };
        board_size_usize * board_size_usize
    ];

    // Place some sample stones in a more complex pattern
    let black_positions2 = vec![(6, 6), (7, 6), (5, 7), (6, 7), (7, 7), (8, 7), (6, 8)];
    let white_positions2 = vec![(4, 6), (5, 6), (4, 7), (4, 8), (5, 8), (7, 8), (6, 9)];

    for (idx, &(x, y)) in black_positions2.iter().enumerate() {
        let index = y * board_size_usize + x;
        board2[index].occupant = Occupant::Black;
        board2[index].move_number = Some(idx as u64 * 2 + 1);
    }

    for (idx, &(x, y)) in white_positions2.iter().enumerate() {
        let index = y * board_size_usize + x;
        board2[index].occupant = Occupant::White;
        board2[index].move_number = Some(idx as u64 * 2 + 2);
    }

    let board2_json = serde_json::to_string(&board2).unwrap();

    // Add the second sample game
    let game_id2 = ctx.timestamp.to_micros_since_unix_epoch() as u64 + 1;
    ctx.db.game().insert(Game {
        id: game_id2,
        player_black: ctx.sender,
        player_white: Some(ctx.sender),
        board: board2_json,
        turn: "W".to_string(),
        passes: 0,
        board_size,
        previous_board: None,
        game_over: false,
        final_score_black: None,
        final_score_white: None,
    });
}
