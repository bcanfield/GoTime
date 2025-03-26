use crate::models::{Board, Occupant, SpotState};
use serde_json;

/// Creates an empty board and returns a Board object
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

pub fn place_test_stone(board: &mut Board, row: u8, col: u8, occupant: Occupant) {
    if let Some(spot) = board.get_mut(row, col) {
        spot.occupant = occupant;
    }
}

pub fn serialize_board(board: &Board) -> String {
    serde_json::to_string(&board.spots).unwrap()
}

// Helper: create a Board from a vector of Occupant values.
pub fn create_board_from_vec(vec: Vec<Occupant>, board_size: u8) -> Board {
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
