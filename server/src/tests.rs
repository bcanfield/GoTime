use crate::models::{Occupant, SpotState};
use crate::utils::{apply_move_to_board, coord_to_index, evaluate_game};

/// Helper: Create an empty board as a Vec<SpotState>.
fn empty_board(size: usize) -> Vec<SpotState> {
    let num_spots = size * size;
    (0..num_spots)
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
        })
        .collect()
}

#[test]
fn test_legal_move() {
    let size = 9;
    let board = empty_board(size);
    let timestamp = 1000;

    let (new_board, new_board_str) =
        apply_move_to_board(board, size, Occupant::Black, 4, 4, None, timestamp)
            .expect("Legal move should succeed");
    let idx = coord_to_index(4, 4, size);
    assert_eq!(
        new_board[idx].occupant,
        Occupant::Black,
        "Black stone should be at (4,4)"
    );
    assert!(new_board_str.is_some(), "Board string should be updated");
}

#[test]
fn test_move_on_occupied() {
    let size = 9;
    let board = empty_board(size);
    let timestamp = 1000;
    let (board, prev) =
        apply_move_to_board(board, size, Occupant::Black, 2, 2, None, timestamp).unwrap();
    let result = apply_move_to_board(board, size, Occupant::White, 2, 2, prev, timestamp + 1);
    assert!(result.is_err(), "Should not allow move on an occupied spot");
}

#[test]
fn test_capture() {
    let size = 5;
    let mut board = empty_board(size);
    let ts = 1000;
    board = apply_move_to_board(board, size, Occupant::White, 2, 2, None, ts)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 2, 1, None, ts + 1)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 1, 2, None, ts + 2)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 3, 2, None, ts + 3)
        .unwrap()
        .0;

    let (new_board, _new_board_str) =
        apply_move_to_board(board, size, Occupant::Black, 2, 3, None, ts + 4).unwrap();
    let idx_white = coord_to_index(2, 2, size);
    assert_eq!(
        new_board[idx_white].occupant,
        Occupant::Empty,
        "White stone should be captured"
    );
}

#[test]
fn test_suicide() {
    let size = 5;
    let mut board = empty_board(size);
    let ts = 1000;
    board = apply_move_to_board(board, size, Occupant::Black, 1, 2, None, ts)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 2, 1, None, ts + 1)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 3, 2, None, ts + 2)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 2, 3, None, ts + 3)
        .unwrap()
        .0;
    let result = apply_move_to_board(board, size, Occupant::White, 2, 2, None, ts + 4);
    assert!(result.is_err(), "Suicide move should be rejected");
}

#[test]
fn test_ko_rule() {
    let size = 5;
    let mut board = empty_board(size);
    let ts = 1000;
    board = apply_move_to_board(board, size, Occupant::White, 2, 2, None, ts)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 2, 1, None, ts + 1)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 1, 2, None, ts + 2)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::Black, 3, 2, None, ts + 3)
        .unwrap()
        .0;
    board = apply_move_to_board(board, size, Occupant::White, 2, 3, None, ts + 4)
        .unwrap()
        .0;
    let prev = serde_json::to_string(&board).unwrap();
    let result = apply_move_to_board(
        board,
        size,
        Occupant::Black,
        2,
        1,
        Some(prev.clone()),
        ts + 5,
    );
    assert!(
        result.is_err(),
        "Ko rule should prevent immediate recapture"
    );
}

#[test]
fn test_pass_and_game_over() {
    let size = 5;
    let board = empty_board(size);
    let mut board = board;
    let ts = 1000;
    for y in 0..3 {
        for x in 0..3 {
            let idx = coord_to_index(x, y, size);
            board[idx].occupant = Occupant::Black;
            board[idx].move_number = Some(ts);
        }
    }
    let (score_black, score_white) = evaluate_game(&board, size);
    assert!(
        score_black > score_white,
        "Black should have a higher score"
    );
}

#[test]
fn test_handicap_creation() {
    let size = 9;
    let handicap = 2;
    let ts = 1000;
    let mut board: Vec<SpotState> = (0..(size * size))
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
        })
        .collect();
    let handicap_positions = vec![(2, 2), (6, 6), (2, 6), (6, 2), (4, 4)];
    for i in 0..(handicap as usize).min(handicap_positions.len()) {
        let (x, y) = handicap_positions[i];
        let idx = coord_to_index(x, y, size);
        board[idx].occupant = Occupant::Black;
        board[idx].move_number = Some(ts);
    }
    let count_black = board
        .iter()
        .filter(|s| s.occupant == Occupant::Black)
        .count();
    assert_eq!(count_black, 2, "Two handicap stones should be pre-placed");
}
