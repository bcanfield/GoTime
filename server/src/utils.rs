use crate::models::{Occupant, SpotState};
use serde_json;
use std::collections::{HashSet, VecDeque};

/// Returns valid neighbor coordinates (up, down, left, right) for (x,y).
pub fn neighbors(x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
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
pub fn coord_to_index(x: usize, y: usize, size: usize) -> usize {
    y * size + x
}

/// Returns all indices in the connected group (stones with the same occupant).
pub fn get_group_indices(board: &[SpotState], size: usize, x: usize, y: usize) -> HashSet<usize> {
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

/// Returns true if the group has at least one liberty.
pub fn group_has_liberty(board: &[SpotState], size: usize, group: &HashSet<usize>) -> bool {
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
pub fn remove_group(board: &mut [SpotState], group: &HashSet<usize>) {
    for &idx in group.iter() {
        board[idx].occupant = Occupant::Empty;
        board[idx].move_number = None;
        board[idx].marker = None;
    }
}

/// Evaluate the board using a simple area scoring method.
/// Returns (score_black, score_white).
pub fn evaluate_game(board: &[SpotState], size: usize) -> (u64, u64) {
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

/// Pure function to apply a move on a board.
/// Returns the updated board and a new board serialization string.
pub fn apply_move_to_board(
    board: Vec<SpotState>,
    size: usize,
    stone_color: Occupant,
    x: usize,
    y: usize,
    previous_board: Option<String>,
    timestamp: u64,
) -> Result<(Vec<SpotState>, Option<String>), String> {
    let mut board = board;
    let idx = coord_to_index(x, y, size);
    if board[idx].occupant != Occupant::Empty {
        return Err("Position already occupied".to_string());
    }
    board[idx].occupant = stone_color.clone();
    board[idx].move_number = Some(timestamp);
    // Capture adjacent opponent groups.
    let opponent = match stone_color {
        Occupant::Black => Occupant::White,
        Occupant::White => Occupant::Black,
        _ => unreachable!(),
    };
    for (nx, ny) in neighbors(x, y, size) {
        let n_idx = coord_to_index(nx, ny, size);
        if board[n_idx].occupant == opponent {
            let group = get_group_indices(&board, size, nx, ny);
            if !group_has_liberty(&board, size, &group) {
                remove_group(&mut board, &group);
            }
        }
    }
    // Check for suicide.
    let group = get_group_indices(&board, size, x, y);
    if !group_has_liberty(&board, size, &group) {
        return Err("Illegal move: suicide".to_string());
    }
    let new_board_str = serde_json::to_string(&board).map_err(|e| e.to_string())?;
    if let Some(prev) = &previous_board {
        if new_board_str == *prev {
            return Err("Illegal move: violates ko rule".to_string());
        }
    }
    Ok((board, Some(new_board_str)))
}
