use crate::models::{Occupant, SpotState};
use serde_json;
use std::collections::{HashSet, VecDeque};

/// Returns the valid orthogonal neighbor coordinates (up, down, left, right) for a given position.
///
/// # Arguments
/// * `x` - The x-coordinate (column)
/// * `y` - The y-coordinate (row)
/// * `size` - The size of the board
///
/// # Returns
/// A vector of (x, y) coordinate pairs representing valid neighboring positions
pub fn neighbors(x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4); // Pre-allocate for up to 4 neighbors
    
    if x > 0 {
        result.push((x - 1, y)); // Left
    }
    if x + 1 < size {
        result.push((x + 1, y)); // Right
    }
    if y > 0 {
        result.push((x, y - 1)); // Up
    }
    if y + 1 < size {
        result.push((x, y + 1)); // Down
    }
    
    result
}

/// Converts 2D board coordinates (x, y) into a 1D index for array access.
///
/// # Arguments
/// * `x` - The x-coordinate (column)
/// * `y` - The y-coordinate (row)
/// * `size` - The size of the board
///
/// # Returns
/// The 1D array index corresponding to the given coordinates
pub fn coord_to_index(x: usize, y: usize, size: usize) -> usize {
    y * size + x
}

/// Finds all positions that are part of the same connected group as the given position.
///
/// A group is a connected set of stones of the same color. In Go, stones are 
/// connected if they are adjacent horizontally or vertically (not diagonally).
///
/// # Arguments
/// * `board` - The current board state
/// * `size` - The size of the board
/// * `x` - The x-coordinate (column) of the starting position
/// * `y` - The y-coordinate (row) of the starting position
///
/// # Returns
/// A HashSet containing the indices of all positions in the group
pub fn get_group_indices(board: &[SpotState], size: usize, x: usize, y: usize) -> HashSet<usize> {
    let mut group = HashSet::new();
    let mut queue = VecDeque::new();
    
    let start_index = coord_to_index(x, y, size);
    let target_occupant = &board[start_index].occupant;
    
    // Handle empty spaces - they don't form groups in Go
    if *target_occupant == Occupant::Empty {
        group.insert(start_index);
        return group;
    }
    
    group.insert(start_index);
    queue.push_back((x, y));
    
    // Breadth-first search to find all connected stones of the same color
    while let Some((cx, cy)) = queue.pop_front() {
        for (nx, ny) in neighbors(cx, cy, size) {
            let n_index = coord_to_index(nx, ny, size);
            
            if !group.contains(&n_index) && board[n_index].occupant == *target_occupant {
                group.insert(n_index);
                queue.push_back((nx, ny));
            }
        }
    }
    
    group
}

/// Checks if a group of stones has at least one liberty (empty adjacent point).
///
/// In Go, a liberty is an empty intersection adjacent to a stone. A group with
/// no liberties is captured and removed from the board.
///
/// # Arguments
/// * `board` - The current board state
/// * `size` - The size of the board
/// * `group` - A set of indices representing the group to check
///
/// # Returns
/// `true` if the group has at least one liberty, `false` otherwise
pub fn group_has_liberty(board: &[SpotState], size: usize, group: &HashSet<usize>) -> bool {
    for &idx in group {
        let x = idx % size;
        let y = idx / size;
        
        for (nx, ny) in neighbors(x, y, size) {
            let n_idx = coord_to_index(nx, ny, size);
            
            if board[n_idx].occupant == Occupant::Empty {
                return true; // Found a liberty
            }
        }
    }
    
    false // No liberties found
}

/// Removes a group of stones from the board by setting their spots to empty.
///
/// This is used when a group is captured (has no liberties).
///
/// # Arguments
/// * `board` - The board to modify
/// * `group` - A set of indices representing the group to remove
pub fn remove_group(board: &mut [SpotState], group: &HashSet<usize>) {
    for &idx in group {
        board[idx].occupant = Occupant::Empty;
        board[idx].move_number = None;
        board[idx].marker = Some("captured".to_string()); // Mark as captured for UI
    }
}

/// Applies a move to the board and handles captures and rule enforcement.
///
/// This is a pure function that returns a new board state rather than modifying the input.
/// It handles:
/// - Placing a stone
/// - Capturing opponent groups with no liberties
/// - Checking for self_capture moves (illegal)
/// - Checking for ko rule violations (illegal)
///
/// # Arguments
/// * `board` - The current board state
/// * `size` - The size of the board
/// * `stone_color` - The color of the stone to place
/// * `x` - The x-coordinate (column) for the move
/// * `y` - The y-coordinate (row) for the move
/// * `previous_board` - Optional previous board state for ko rule checking
/// * `timestamp` - Timestamp for this move
///
/// # Returns
/// * `Ok((new_board, serialized_board))` - The updated board and its serialized form
/// * `Err(message)` - An error message if the move is illegal
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
    
    // Print occupant for debugging
    println!("Occupant at index {}: {:?}", idx, board[idx].occupant);
    // Check if the position is already occupied
    if board[idx].occupant != Occupant::Empty {
        return Err("Position already occupied".to_string());
    }
    
    // Place the stone
    board[idx].occupant = stone_color.clone();
    board[idx].move_number = Some(timestamp);
    
    // Determine the opponent's color
    let opponent = match stone_color {
        Occupant::Black => Occupant::White,
        Occupant::White => Occupant::Black,
        Occupant::Empty => return Err("Cannot play an empty stone".to_string()),
    };
    
    // Check and capture any adjacent opponent groups with no liberties
    let mut captured_something = false;
    for (nx, ny) in neighbors(x, y, size) {
        let n_idx = coord_to_index(nx, ny, size);
        
        if board[n_idx].occupant == opponent {
            let group = get_group_indices(&board, size, nx, ny);
            
            if !group_has_liberty(&board, size, &group) {
                remove_group(&mut board, &group);
                captured_something = true;
            }
        }
    }
    
    // Check for self_capture moves (unless we captured something)
    if !captured_something {
        let group = get_group_indices(&board, size, x, y);
        
        if !group_has_liberty(&board, size, &group) {
            // Undo the move
            board[idx].occupant = Occupant::Empty;
            board[idx].move_number = None;
            
            return Err("Illegal move: self_capture".to_string());
        }
    }
    
    // Serialize the new board state for comparison and storage
    let new_board_str = match serde_json::to_string(&board) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to serialize board: {}", e)),
    };
    
    // Check for ko rule violation
    // if let Some(prev) = &previous_board {
    //     if new_board_str == *prev {
    //         // Log the ko rule violation
    //         println!("Ko rule violation detected: new board state matches previous board state");
            
            // // Undo the move
            // board[idx].occupant = Occupant::Empty;
            // board[idx].move_number = None;
            
    //         return Err("Illegal move: violates ko rule".to_string());
    //     } else {
    //         // Log the new board state
    //         println!("No vialoation detected: new board state does not match previous board state");
    //     }
    // }
    if let Some(prev) = &previous_board {
        // Deserialize the previous board state
        let prev_board: Vec<SpotState> = serde_json::from_str(prev)
            .map_err(|e| format!("Failed to deserialize previous board state: {}", e))?;
        
        // Extract only the occupant values by cloning each one
        let current_occupants: Vec<Occupant> = board.iter().map(|spot| spot.occupant.clone()).collect();
        let previous_occupants: Vec<Occupant> = prev_board.iter().map(|spot| spot.occupant.clone()).collect();
        
        // Compare the occupant vectors
        if current_occupants == previous_occupants {
            println!("Ko rule violation: occupant configurations are identical.");
                        // Undo the move
                        board[idx].occupant = Occupant::Empty;
                        board[idx].move_number = None;
            return Err("Illegal move: violates ko rule".to_string());
        }
    }
    
    // Log the board state for debugging
    for y in 0..size {
        let mut row_str = String::new();
        for x in 0..size {
            let idx = coord_to_index(x, y, size);
            let stone = match board[idx].occupant {
                Occupant::Black => "B",
                Occupant::White => "W",
                Occupant::Empty => ".",
            };
            row_str.push_str(stone);
        }
        println!("New {}", row_str);
    }

    // log the previous board state
    if let Some(prev) = &previous_board {
        // we need to convert the previous board string to a Vec<SpotState>
        let prev_board: Vec<SpotState> = serde_json::from_str(prev).unwrap_or_else(|e| {
            panic!("Failed to deserialize previous board state: {}", e)
        });
        // Print the previous board state
        for y in 0..size {
            let mut row_str = String::new();
            for x in 0..size {
                let idx = coord_to_index(x, y, size);
                let stone = match prev_board[idx].occupant {
                    Occupant::Black => "B",
                    Occupant::White => "W",
                    Occupant::Empty => ".",
                };
                row_str.push_str(stone);
            }
            println!("Previous {}", row_str);
        }
        // println!("Previous board state: {}", prev);
    } else {
        println!("No previous board state provided");
    }
    
    Ok((board, Some(new_board_str)))
}
