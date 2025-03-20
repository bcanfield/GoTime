use serde::{Deserialize, Serialize};
use serde_json;
use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;

const DEFAULT_BOARD_SIZE: u8 = 9;

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}

#[table(name = message, public)]
pub struct Message {
    pub sender: Identity,
    pub sent: Timestamp,
    pub text: String,
}

/// In this version, we store the Go board as a JSON‑serialized Vec<SpotState>.
/// The board is a grid of board_size x board_size spots.
#[table(name = game, public)]
pub struct Game {
    #[primary_key]
    pub id: u64,
    pub player_black: Identity,
    pub player_white: Option<Identity>,
    pub board: String, // JSON serialized Vec<SpotState>
    pub turn: String,  // "B" for Black or "W" for White
    pub passes: u8,
    pub board_size: u8,
    pub previous_board: Option<String>, // For simple ko rule checking
    pub game_over: bool,
    pub final_score_black: Option<u64>,
    pub final_score_white: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Occupant {
    Empty,
    Black,
    White,
}

/// Each board spot holds not only which stone (if any) is there,
/// but also additional details (for example, when the stone was played).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotState {
    pub occupant: Occupant,
    pub move_number: Option<u64>, // e.g. timestamp when stone was placed
    pub marker: Option<String>,   // extra state details if needed
}

/// Create an empty board as a JSON string.
// fn init_board(size: u8) -> String {
//     let num_spots = (size as usize) * (size as usize);
//     let board: Vec<SpotState> = (0..num_spots)
//         .map(|_| SpotState {
//             occupant: Occupant::Empty,
//             move_number: None,
//             marker: None,
//         })
//         .collect();
//     serde_json::to_string(&board).unwrap()
// }

/// Returns the valid neighbor coordinates (up, down, left, right) for (x,y).
fn neighbors(x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if x + 1 < size {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y + 1 < size {
        result.push((x, y + 1));
    }
    result
}

/// Converts 2D coordinates (x,y) into a 1D index.
fn coord_to_index(x: usize, y: usize, size: usize) -> usize {
    y * size + x
}

/// Given a starting coordinate, return all indices in the connected group (stones with the same occupant).
fn get_group_indices(board: &Vec<SpotState>, size: usize, x: usize, y: usize) -> HashSet<usize> {
    let mut group = HashSet::new();
    let mut queue = VecDeque::new();
    let start_index = coord_to_index(x, y, size);
    group.insert(start_index);
    queue.push_back((x, y));
    let target = &board[start_index].occupant;
    while let Some((cx, cy)) = queue.pop_front() {
        for (nx, ny) in neighbors(cx, cy, size) {
            let n_index = coord_to_index(nx, ny, size);
            if !group.contains(&n_index) && board[n_index].occupant == *target {
                group.insert(n_index);
                queue.push_back((nx, ny));
            }
        }
    }
    group
}

/// Returns true if the group (given by indices) has at least one liberty.
fn group_has_liberty(board: &Vec<SpotState>, size: usize, group: &HashSet<usize>) -> bool {
    for &idx in group.iter() {
        let x = idx % size;
        let y = idx / size;
        for (nx, ny) in neighbors(x, y, size) {
            let n_idx = coord_to_index(nx, ny, size);
            if board[n_idx].occupant == Occupant::Empty {
                return true;
            }
        }
    }
    false
}

/// Remove a group from the board by resetting its spots to empty.
fn remove_group(board: &mut Vec<SpotState>, group: &HashSet<usize>) {
    for &idx in group.iter() {
        board[idx].occupant = Occupant::Empty;
        board[idx].move_number = None;
        board[idx].marker = None;
    }
}

/// A basic evaluation that uses area scoring.
fn evaluate_game(board: &Vec<SpotState>, size: usize) -> (u64, u64) {
    let mut visited = vec![false; board.len()];
    let mut score_black = 0;
    let mut score_white = 0;
    for i in 0..board.len() {
        match board[i].occupant {
            Occupant::Black => score_black += 1,
            Occupant::White => score_white += 1,
            Occupant::Empty => {
                if !visited[i] {
                    let mut region = Vec::new();
                    let mut queue = VecDeque::new();
                    queue.push_back(i);
                    visited[i] = true;
                    while let Some(idx) = queue.pop_front() {
                        region.push(idx);
                        let x = idx % size;
                        let y = idx / size;
                        for (nx, ny) in neighbors(x, y, size) {
                            let n_idx = coord_to_index(nx, ny, size);
                            if !visited[n_idx] && board[n_idx].occupant == Occupant::Empty {
                                visited[n_idx] = true;
                                queue.push_back(n_idx);
                            }
                        }
                    }
                    // Determine border colors.
                    let mut border_black = false;
                    let mut border_white = false;
                    for &idx in &region {
                        let x = idx % size;
                        let y = idx / size;
                        for (nx, ny) in neighbors(x, y, size) {
                            let n_idx = coord_to_index(nx, ny, size);
                            match board[n_idx].occupant {
                                Occupant::Black => border_black = true,
                                Occupant::White => border_white = true,
                                _ => {}
                            }
                        }
                    }
                    if border_black && !border_white {
                        score_black += region.len() as u64;
                    } else if border_white && !border_black {
                        score_white += region.len() as u64;
                    }
                }
            }
        }
    }
    (score_black, score_white)
}

#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    ctx.db.message().insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            name: Some(name),
            ..user
        });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
        });
    }
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    } else {
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        );
    }
}

/// Reducer to create a new game with optional handicap support.
#[reducer]
pub fn create_game(
    ctx: &ReducerContext,
    board_size: Option<u8>,
    handicap: Option<u8>,
) -> Result<(), String> {
    let size = board_size.unwrap_or(DEFAULT_BOARD_SIZE);
    let game_id = ctx
        .timestamp
        .to_micros_since_unix_epoch()
        .try_into()
        .unwrap();
    let mut board: Vec<SpotState> = (0..(size as usize * size as usize))
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
        })
        .collect();
    // Pre-place handicap stones if provided.
    let handicap = handicap.unwrap_or(0);
    if handicap > 0 {
        // For simplicity, predefined handicap positions (for a 9x9 board; extend as needed).
        let handicap_positions = match size {
            9 => vec![(2, 2), (6, 6), (2, 6), (6, 2), (4, 4)],
            13 => vec![(3, 3), (9, 9), (3, 9), (9, 3), (6, 6)],
            19 => vec![(3, 3), (15, 15), (3, 15), (15, 3), (9, 9)],
            _ => vec![],
        };
        for i in 0..(handicap as usize).min(handicap_positions.len()) {
            let (x, y) = handicap_positions[i];
            let idx = coord_to_index(x, y, size as usize);
            board[idx].occupant = Occupant::Black;
            board[idx].move_number = Some(
                ctx.timestamp
                    .to_micros_since_unix_epoch()
                    .try_into()
                    .unwrap(),
            );
        }
    }
    // If handicap is used, White starts; otherwise Black starts.
    let turn = if handicap > 0 {
        "W".to_string()
    } else {
        "B".to_string()
    };
    let board_json = serde_json::to_string(&board).unwrap();
    ctx.db.game().insert(Game {
        id: game_id,
        player_black: ctx.sender,
        player_white: None,
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

#[reducer]
pub fn join_game(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    if let Some(game) = ctx.db.game().id().find(game_id) {
        if game.player_white.is_some() {
            return Err("Game already has two players".to_string());
        }
        ctx.db.game().id().update(Game {
            player_white: Some(ctx.sender),
            ..game
        });
        Ok(())
    } else {
        Err("Game not found".to_string())
    }
}

/// Reducer to pass a turn. Two consecutive passes end the game.
#[reducer]
pub fn pass_move(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err("Game not found".to_string()),
    };
    if game.game_over {
        return Err("Game is already over".to_string());
    }
    game.passes += 1;
    if game.passes >= 2 {
        // End game: evaluate board.
        let board: Vec<SpotState> = serde_json::from_str(&game.board).map_err(|e| e.to_string())?;
        let (score_black, score_white) = evaluate_game(&board, game.board_size as usize);
        game.final_score_black = Some(score_black);
        game.final_score_white = Some(score_white);
        game.game_over = true;
    } else {
        // Switch turn without changing the board.
        game.turn = if game.turn == "B" {
            "W".to_string()
        } else {
            "B".to_string()
        };
    }
    ctx.db.game().id().update(game);
    Ok(())
}

/// Reducer to place a stone at coordinates (x, y).
#[reducer]
pub fn place_stone(ctx: &ReducerContext, game_id: u64, x: u8, y: u8) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err("Game not found".to_string()),
    };
    if game.game_over {
        return Err("Game is over".to_string());
    }
    if game.player_white.is_none() {
        return Err("Waiting for second player".to_string());
    }
    let stone_color = if ctx.sender == game.player_black {
        Occupant::Black
    } else if Some(ctx.sender) == game.player_white {
        Occupant::White
    } else {
        return Err("You are not a player in this game".to_string());
    };
    let expected_turn = match stone_color {
        Occupant::Black => "B",
        Occupant::White => "W",
        _ => unreachable!(),
    };
    if game.turn != expected_turn {
        return Err("Not your turn".to_string());
    }
    let size = game.board_size as usize;
    let mut board: Vec<SpotState> = serde_json::from_str(&game.board).map_err(|e| e.to_string())?;
    let idx = coord_to_index(x as usize, y as usize, size);
    if board[idx].occupant != Occupant::Empty {
        return Err("Position already occupied".to_string());
    }
    let move_num: u64 = ctx
        .timestamp
        .to_micros_since_unix_epoch()
        .try_into()
        .unwrap();
    board[idx].occupant = stone_color.clone();
    board[idx].move_number = Some(move_num);
    let prev_board = game.board.clone();

    // Check each neighboring cell: capture opponent groups if they lose their liberties.
    let opponent = match stone_color {
        Occupant::Black => Occupant::White,
        Occupant::White => Occupant::Black,
        _ => unreachable!(),
    };
    for (nx, ny) in neighbors(x as usize, y as usize, size) {
        let n_idx = coord_to_index(nx, ny, size);
        if board[n_idx].occupant == opponent {
            let group = get_group_indices(&board, size, nx, ny);
            if !group_has_liberty(&board, size, &group) {
                remove_group(&mut board, &group);
            }
        }
    }
    // Check for suicide.
    let group = get_group_indices(&board, size, x as usize, y as usize);
    if !group_has_liberty(&board, size, &group) {
        return Err("Illegal move: suicide".to_string());
    }
    let new_board_str = serde_json::to_string(&board).map_err(|e| e.to_string())?;
    // Basic ko rule: disallow move if board reverts to previous state.
    if let Some(prev) = &game.previous_board {
        if new_board_str == *prev {
            return Err("Illegal move: violates ko rule".to_string());
        }
    }
    // Update game state.
    game.previous_board = Some(prev_board);
    game.board = new_board_str;
    game.passes = 0;
    game.turn = if game.turn == "B" {
        "W".to_string()
    } else {
        "B".to_string()
    };
    ctx.db.game().id().update(game);
    Ok(())
}
