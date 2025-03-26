use crate::models::Occupant;
use crate::tests::test_utils::{
    create_board_from_string, create_empty_board, place_test_stone, serialize_board,
};
use crate::utils::{apply_move_to_board, coord_to_index};

/// Tests that a legal move is successfully applied to the board.
#[test]
fn test_legal_move() {
    let size = 9;
    let board = create_empty_board(size).spots;
    let timestamp = 1000;
    
    // Place a black stone at the center of the board
    let (new_board, new_board_str) = apply_move_to_board(
        board, 
        size as usize, 
        Occupant::Black, 
        4, 
        4, 
        None, 
        timestamp
    ).expect("Legal move should succeed");
    
    // Verify the stone was placed correctly
    let idx = coord_to_index(4, 4, size as usize);
    assert_eq!(
        new_board[idx].occupant,
        Occupant::Black,
        "Black stone should be at (4,4)"
    );
    assert!(new_board_str.is_some(), "Board string should be updated");
}

/// Tests that placing a stone on an already occupied position fails.
#[test]
fn test_move_on_occupied() {
    let size = 9;
    let board = create_empty_board(size).spots;
    let timestamp = 1000;
    
    // First place a black stone
    let (board, prev) = apply_move_to_board(
        board, 
        size as usize, 
        Occupant::Black, 
        2, 
        2, 
        None, 
        timestamp
    ).unwrap();
    
    // Then try to place a white stone in the same position
    let result = apply_move_to_board(
        board, 
        size as usize, 
        Occupant::White, 
        2, 
        2, 
        prev, 
        timestamp + 1
    );
    
    assert!(result.is_err(), "Should not allow move on an occupied spot");
    assert!(
        result.unwrap_err().contains("occupied"),
        "Error message should mention the spot is occupied"
    );
}

/// Tests that stones with no liberties are captured.
#[test]
fn test_capture() {
    let size = 5;
    let board_str = "
        .....
        .....
        ..W..
        .....
        .....
    ";
    let mut board = create_board_from_string(board_str, size).spots;
    let ts = 1000;
    
    // Surround the white stone with black stones
    board = apply_move_to_board(board, size as usize, Occupant::Black, 2, 1, None, ts + 1).unwrap().0;
    board = apply_move_to_board(board, size as usize, Occupant::Black, 1, 2, None, ts + 2).unwrap().0;
    board = apply_move_to_board(board, size as usize, Occupant::Black, 3, 2, None, ts + 3).unwrap().0;
    
    // Complete the capture with the final black stone
    let (new_board, _) = apply_move_to_board(board, size as usize, Occupant::Black, 2, 3, None, ts + 4).unwrap();
    
    // Verify the white stone was captured (removed)
    let idx_white = coord_to_index(2, 2, size as usize);
    assert_eq!(
        new_board[idx_white].occupant,
        Occupant::Empty,
        "White stone should be captured"
    );
    assert!(
        new_board[idx_white].marker.is_some(),
        "Captured spot should be marked"
    );
}

/// Tests that suicide moves (placing a stone with no liberties) are rejected.
#[test]
fn test_suicide() {
    // Create a board with black stones surrounding an empty spot
    let board_str = "
        .....
        ..B..
        .B.B.
        ..B..
        .....
    ";
    let board = create_board_from_string(board_str, 5).spots;
    let ts = 1000;
    
    // Try to place a white stone in the surrounded empty spot
    let result = apply_move_to_board(board, 5, Occupant::White, 2, 2, None, ts);
    
    assert!(result.is_err(), "Suicide move should be rejected");
    assert!(
        result.unwrap_err().contains("suicide"),
        "Error message should mention suicide"
    );
}

/// Tests that the ko rule prevents immediate recapture.
#[test]
fn test_ko_rule() {
    // Set up a ko situation
    // Initial board with white at center
    let mut board = create_empty_board(5).spots;
    let ts = 1000;
    
    // Place stones to set up the ko position
    board = apply_move_to_board(board, 5, Occupant::White, 2, 2, None, ts).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 2, 1, None, ts + 1).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 1, 2, None, ts + 2).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 3, 2, None, ts + 3).unwrap().0;
    
    // White plays to capture one stone and create ko
    let (board, _) = apply_move_to_board(board, 5, Occupant::White, 2, 3, None, ts + 4).unwrap();
    
    // Record the board state after the capture
    let prev = serialize_board(&crate::models::Board::new(board.clone(), 5));
    
    // Black tries to immediately recapture, which should violate ko
    let result = apply_move_to_board(
        board,
        5,
        Occupant::Black,
        2,
        2,
        Some(prev),
        ts + 5,
    );
    
    assert!(
        result.is_err(),
        "Ko rule should prevent immediate recapture"
    );
    assert!(
        result.unwrap_err().contains("ko"),
        "Error message should mention ko rule"
    );
}

/// Tests creating a board with handicap stones.
#[test]
fn test_handicap_creation() {
    let size = 9;
    let handicap = 2;
    let ts = 1000;
    
    // Create an empty board
    let mut board = create_empty_board(size).spots;
    
    // Place handicap stones
    let handicap_positions = vec![(2, 2), (6, 6), (2, 6), (6, 2), (4, 4)];
    for i in 0..(handicap as usize).min(handicap_positions.len()) {
        let (x, y) = handicap_positions[i];
        let idx = coord_to_index(x, y, size as usize);
        board[idx].occupant = Occupant::Black;
        board[idx].move_number = Some(ts);
    }
    
    // Verify the correct number of handicap stones were placed
    let count_black = board
        .iter()
        .filter(|s| s.occupant == Occupant::Black)
        .count();
    
    assert_eq!(
        count_black, 
        handicap as usize, 
        "{} handicap stones should be pre-placed", 
        handicap
    );
}

/// Tests that a black stone can capture multiple white groups simultaneously.
#[test]
fn test_multiple_group_capture() {
    // Create a board with two separate white groups that can be captured with one move
    let board_str = "
        .....
        .W.W.
        .....
        .....
        .....
    ";
    let mut board = create_board_from_string(board_str, 5).spots;
    let ts = 1000;
    
    // Surround the white stones except for one shared liberty
    board = apply_move_to_board(board, 5, Occupant::Black, 0, 1, None, ts).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 1, 0, None, ts + 1).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 2, 0, None, ts + 2).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 3, 0, None, ts + 3).unwrap().0;
    board = apply_move_to_board(board, 5, Occupant::Black, 4, 1, None, ts + 4).unwrap().0;
    
    // Place the final capturing stone
    let (new_board, _) = apply_move_to_board(board, 5, Occupant::Black, 2, 1, None, ts + 5).unwrap();
    
    // Verify both white stones were captured
    let idx_white1 = coord_to_index(1, 1, 5);
    let idx_white2 = coord_to_index(3, 1, 5);
    
    assert_eq!(
        new_board[idx_white1].occupant,
        Occupant::Empty,
        "First white stone should be captured"
    );
    assert_eq!(
        new_board[idx_white2].occupant,
        Occupant::Empty,
        "Second white stone should be captured"
    );
}
