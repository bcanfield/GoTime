use crate::models::Occupant;
use crate::tests::test_utils::{
    create_board_from_string, create_empty_board, place_test_stone, serialize_board,
};
use crate::utils::{apply_move_to_board, coord_to_index, get_group_indices, group_has_liberty};

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

/// Tests that self_capture moves (placing a stone with no liberties) are rejected.
#[test]
fn test_self_capture() {
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
    
    assert!(result.is_err(), "self_capture move should be rejected");
    assert!(
        result.unwrap_err().contains("self_capture"),
        "Error message should mention self_capture"
    );
}

/// Tests that the ko rule prevents immediate recapture.
#[test]
fn test_ko_rule() {
    // Create an empty board
    let mut board = create_empty_board(9).spots;
    let ts = 1000;
    
    // Set up the Ko pattern with symmetrical stones
    // This creates a situation where a capture will lead to a Ko
    
    // Place Black stones on three sides of center
    board[coord_to_index(2, 3, 9)].occupant = Occupant::Black; // C4
    board[coord_to_index(3, 2, 9)].occupant = Occupant::Black; // D3
    board[coord_to_index(4, 3, 9)].occupant = Occupant::Black; // E4
    
    // Place White stones symmetrically opposite
    board[coord_to_index(2, 4, 9)].occupant = Occupant::White; // C5
    board[coord_to_index(4, 4, 9)].occupant = Occupant::White; // E5
    board[coord_to_index(3, 5, 9)].occupant = Occupant::White; // D6
    
    // Place the White stone that will be captured
    board[coord_to_index(3, 3, 9)].occupant = Occupant::White; // D4
    
    // Print the initial board state
    println!("Initial board state with Ko pattern:");
    for y in 0..9 {
        let mut row_str = String::new();
        for x in 0..9 {
            let idx = coord_to_index(x, y, 9);
            let stone = match board[idx].occupant {
                Occupant::Black => "B",
                Occupant::White => "W",
                Occupant::Empty => "."
            };
            row_str.push_str(stone);
        }
        println!("{}", row_str);
    }
    
    // Black completes the capture by playing at D5 (3,4)
    let (board_after_capture, _) = apply_move_to_board(
        board.clone(),
        9,
        Occupant::Black,
        3, // D
        4, // 5
        None,
        ts
    ).unwrap_or_else(|e| panic!("Black's capture move failed: {}", e));
    
    // Print board after Black's capture
    println!("\nAfter Black captures White by playing at D5:");
    for y in 0..9 {
        let mut row_str = String::new();
        for x in 0..9 {
            let idx = coord_to_index(x, y, 9);
            let stone = match board_after_capture[idx].occupant {
                Occupant::Black => "B",
                Occupant::White => "W",
                Occupant::Empty => if board_after_capture[idx].marker.is_some() { "C" } else { "." }
            };
            row_str.push_str(stone);
        }
        println!("{}", row_str);
    }
    
    // Verify the White stone at D4 was captured
    let d4_idx = coord_to_index(3, 3, 9);
    assert_eq!(
        board_after_capture[d4_idx].occupant,
        Occupant::Empty,
        "White stone at D4 should be captured"
    );
    
    // Save this board state for ko detection
    let prev_board_str = serialize_board(&crate::models::Board::new(board_after_capture.clone(), 9));
    
    // White plays at D4 to capture the Black stone at D5
    let (board_after_white, _) = apply_move_to_board(
        board_after_capture.clone(),
        9,
        Occupant::White,
        3, // D
        3, // 4
        None,
        ts + 1
    ).unwrap_or_else(|e| panic!("White's move at D4 failed: {}", e));
    
    // Print board after White's move
    println!("\nAfter White plays at D4 (capturing Black's stone at D5):");
    for y in 0..9 {
        let mut row_str = String::new();
        for x in 0..9 {
            let idx = coord_to_index(x, y, 9);
            let stone = match board_after_white[idx].occupant {
                Occupant::Black => "B",
                Occupant::White => "W",
                Occupant::Empty => if board_after_white[idx].marker.is_some() { "C" } else { "." }
            };
            row_str.push_str(stone);
        }
        println!("{}", row_str);
    }
    
    // Verify that White's stone is at D4 and the Black stone at D5 was captured
    assert_eq!(
        board_after_white[d4_idx].occupant,
        Occupant::White,
        "White stone should be at D4 now"
    );
    
    let d5_idx = coord_to_index(3, 4, 9);
    assert_eq!(
        board_after_white[d5_idx].occupant,
        Occupant::Empty,
        "Black stone at D5 should be captured"
    );
    
    // Now Black tries to recapture at D5, which should violate the Ko rule
    println!("\nBlack attempts to recapture at D5 (should violate Ko rule):");
    let result = apply_move_to_board(
        board_after_white,
        9,
        Occupant::Black,
        3, // D
        4, // 5
        Some(prev_board_str),
        ts + 2
    );
    
    // This should fail due to Ko rule
    assert!(result.is_err(), "Ko rule should prevent immediate recapture");
    
    let error_message = result.unwrap_err();
    println!("Error message: '{}'", error_message);
    
    // Check that the error mentions the Ko rule
    assert!(
        error_message.contains("violates ko rule"),
        "Error message should mention ko rule, got: '{}'", error_message
    );

    
}
