use crate::models::{Board, Occupant, SpotState};
use serde_json;

/// Creates an empty board with the specified size.
///
/// This utility function is useful for creating test boards without
/// having to manually initialize all spots.
///
/// # Arguments
/// * `size` - The size of the board to create (e.g., 9, 13, 19)
///
/// # Returns
/// A new Board instance with all spots empty
pub fn create_empty_board(size: u8) -> Board {
    let spots = vec![
        SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        };
        (size as usize).pow(2)
    ];
    Board::new(spots, size)
}

/// Places a stone at the specified position on a board for testing.
///
/// # Arguments
/// * `board` - Mutable reference to the board to modify
/// * `row` - The row coordinate (0-based)
/// * `col` - The column coordinate (0-based)
/// * `occupant` - The type of stone to place (Black, White, or Empty)
pub fn place_test_stone(board: &mut Board, row: u8, col: u8, occupant: Occupant) {
    if let Some(spot) = board.get_mut(row, col) {
        spot.occupant = occupant;
    } else {
        panic!("Attempted to place stone at invalid position ({}, {})", row, col);
    }
}

/// Serializes a board to a JSON string.
///
/// This is useful for testing functions that expect a serialized board.
///
/// # Arguments
/// * `board` - The board to serialize
///
/// # Returns
/// A JSON string representation of the board's spots
pub fn serialize_board(board: &Board) -> String {
    serde_json::to_string(&board.spots).unwrap()
}

/// Creates a board from a vector of Occupants.
///
/// This utility makes it easy to create test boards with specific patterns.
/// The vector should contain occupants in row-major order.
///
/// # Arguments
/// * `vec` - Vector of Occupants representing the board state
/// * `board_size` - The size of the board
///
/// # Returns
/// A new Board instance with the specified stone pattern
///
/// # Panics
/// Panics if the vector length doesn't match board_size²
pub fn create_board_from_vec(vec: Vec<Occupant>, board_size: u8) -> Board {
    assert_eq!(
        vec.len(),
        (board_size as usize).pow(2),
        "Vector length {} doesn't match expected board size {}²={}",
        vec.len(),
        board_size,
        (board_size as usize).pow(2)
    );
    
    let spots: Vec<SpotState> = vec
        .into_iter()
        .map(|occ| SpotState {
            occupant: occ,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        })
        .collect();
    
    Board::new(spots, board_size)
}

/// Creates a board from a string representation.
///
/// This is a convenient way to create test boards using a visual representation.
/// Each character in the string represents a spot:
/// - 'B' or 'b': Black stone
/// - 'W' or 'w': White stone
/// - Any other character: Empty spot
///
/// # Arguments
/// * `board_str` - String representation of the board (whitespace is ignored)
/// * `board_size` - The size of the board
///
/// # Returns
/// A new Board instance with the specified stone pattern
///
/// # Example
/// ```
/// let board = create_board_from_string(
///     "
///     .B...
///     .BW..
///     .....
///     .....
///     .....
///     ",
///     5
/// );
/// ```
pub fn create_board_from_string(board_str: &str, board_size: u8) -> Board {
    // Remove whitespace
    let clean_str: String = board_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    
    assert_eq!(
        clean_str.len(),
        (board_size as usize).pow(2),
        "String length {} doesn't match expected board size {}²={}",
        clean_str.len(),
        board_size,
        (board_size as usize).pow(2)
    );
    
    let spots: Vec<SpotState> = clean_str
        .chars()
        .map(|c| SpotState {
            occupant: match c {
                'B' | 'b' => Occupant::Black,
                'W' | 'w' => Occupant::White,
                _ => Occupant::Empty,
            },
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        })
        .collect();
    
    Board::new(spots, board_size)
}
