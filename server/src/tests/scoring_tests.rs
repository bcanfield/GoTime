use crate::models::{Occupant, ScoringMethod};
use crate::scoring::{
    calculate_score, determine_territory, find_empty_regions, find_groups, remove_dead_stones,
};
use crate::tests::test_utils::create_board_from_vec;
use std::collections::HashSet;

#[test]
fn test_indexing() {
    let board = create_board_from_vec(vec![Occupant::Empty; 9], 3);
    assert_eq!(board.index(0, 0), 0);
    assert_eq!(board.index(1, 1), 4);
    assert_eq!(board.index(2, 2), 8);
}

#[test]
fn test_find_groups() {
    // 3x3 board: one Black stone at top left.
    let mut vec = vec![Occupant::Empty; 9];
    vec[0] = Occupant::Black;
    let board = create_board_from_vec(vec, 3);
    let groups = find_groups(&board);
    assert_eq!(groups.len(), 1);
    let group = &groups[0];
    assert_eq!(group.stones.len(), 1);
    assert_eq!(group.occupant, Occupant::Black);
    // Black stone at (0,0) has neighbors (0,1) and (1,0)
    let mut expected = HashSet::new();
    expected.insert((0, 1));
    expected.insert((1, 0));
    assert_eq!(group.liberties, expected);
}

#[test]
fn test_remove_dead_stones() {
    // 3x3 board where a Black stone at (1,1) is completely surrounded by White.
    // Board layout:
    //   W  W  W
    //   W  B  W
    //   W  W  W
    let board_size = 3;
    let mut vec = Vec::new();
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 1 && col == 1 {
                vec.push(Occupant::Black);
            } else {
                vec.push(Occupant::White);
            }
        }
    }
    let mut board = create_board_from_vec(vec, board_size);
    
    // Debug: Print all groups and their liberties
    let groups = find_groups(&board);
    println!("Number of groups found: {}", groups.len());
    for (i, group) in groups.iter().enumerate() {
        println!("Group {}: color = {:?}, stones = {}, liberties = {}", 
                 i, group.occupant, group.stones.len(), group.liberties.len());
        println!("  Stones: {:?}", group.stones);
        println!("  Liberties: {:?}", group.liberties);
    }
    
    // The black group should have no liberties.
    let black_group = groups
        .iter()
        .find(|g| g.occupant == Occupant::Black)
        .unwrap();
    assert!(black_group.liberties.is_empty());
    
    // Remove dead stones.
    let removed_groups = remove_dead_stones(&mut board);
    
    // Debug: Print removed groups
    println!("Number of removed groups: {}", removed_groups.len());
    for (i, group) in removed_groups.iter().enumerate() {
        println!("Removed group {}: color = {:?}, stones = {}", 
                 i, group.occupant, group.stones.len());
        println!("  Stones: {:?}", group.stones);
    }
    
    // Check that the black stone was removed.
    assert_eq!(board.get(1, 1).unwrap().occupant, Occupant::Empty);
    assert_eq!(board.get(1, 1).unwrap().marker.as_deref(), Some("removed"));
    
    // Also, removed_groups should contain one group.
    assert_eq!(removed_groups.len(), 1);
}

#[test]
fn test_find_empty_regions_and_territory() {
    // 3x3 board: Black stones at (0,0) and (0,1); others empty.
    // Although the only border color is Black, the empty region touches the board edge.
    // According to Japanese territory scoring rules, only regions fully enclosed
    // (i.e. not touching the edge) count as territory. Thus, both Black and White
    // should get 0 territory.
    let board_size = 3;
    let mut vec = vec![Occupant::Empty; 9];
    vec[0] = Occupant::Black; // (0,0)
    vec[1] = Occupant::Black; // (0,1)
    let board = create_board_from_vec(vec, board_size);
    let regions = find_empty_regions(&board);
    assert_eq!(regions.len(), 1);
    let region = &regions[0];
    assert!(region.border.contains(&Occupant::Black));
    // Determine territory.
    let (black_territory, white_territory) = determine_territory(&board);
    // Since the empty region touches the board edge, it is not counted as territory.
    assert_eq!(black_territory, 0);
    assert_eq!(white_territory, 0);
}

#[test]
fn test_calculate_score_area() {
    // 3x3 board: place some Black stones to “enclose” territory.
    let board_size = 3;
    let mut vec = vec![Occupant::Empty; 9];
    // Place Black stones at (0,0), (0,1), and (1,0)
    vec[0] = Occupant::Black;
    vec[1] = Occupant::Black;
    vec[3] = Occupant::Black;
    let board = create_board_from_vec(vec, board_size);
    let (black_score, white_score) = calculate_score(&board, ScoringMethod::Area, 6.5);
    // Black's area score should be at least the number of stones (3) plus some territory.
    assert!(black_score >= 3.0);
    // White's score should be at least the komi.
    assert!(white_score >= 6.5);
}

// Test 1: Empty Board Test
// A completely empty board should have 0 stones and no enclosed territory.
// Thus, for area scoring, Black should have 0 and White only gets komi;
// for territory scoring, both get 0 except White’s komi.
#[test]
fn test_empty_board() {
    let board_size = 3;
    let total = (board_size as usize).pow(2);
    let board = create_board_from_vec(vec![Occupant::Empty; total], board_size);

    // Area scoring: no stones and no territory.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 0.0);
    assert_eq!(white_area, 6.5);

    // Territory scoring: no territory for either side (other than komi for White).
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 6.5);
}

// Test 2: Single Stone Tests
// These tests verify that a single stone does not erroneously enclose territory.
// 2a. Center stone on a 3x3 board.
#[test]
fn test_single_stone_center() {
    let board_size = 3;
    let mut occupants = vec![Occupant::Empty; 9];
    occupants[4] = Occupant::Black; // Center (row 1, col 1)
    let board = create_board_from_vec(occupants, board_size);

    // Area scoring: Black stone count = 1, but the empty region is not fully bordered by Black.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 1.0);
    assert_eq!(white_area, 6.5);

    // Territory scoring: No territory enclosed.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 6.5);
}

// 2b. Corner stone on a 3x3 board.
#[test]
fn test_single_stone_corner() {
    let board_size = 3;
    let mut occupants = vec![Occupant::Empty; 9];
    occupants[0] = Occupant::Black; // Top-left corner
    let board = create_board_from_vec(occupants, board_size);

    // Area scoring: Black stone count = 1, no enclosed territory.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 1.0);
    assert_eq!(white_area, 6.5);

    // Territory scoring: No territory enclosed.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 6.5);
}

// 2c. Edge stone on a 3x3 board.
#[test]
fn test_single_stone_edge() {
    let board_size = 3;
    let mut occupants = vec![Occupant::Empty; 9];
    occupants[1] = Occupant::Black; // Middle of top edge (row 0, col 1)
    let board = create_board_from_vec(occupants, board_size);

    // Area scoring: Black stone count = 1, but no enclosed territory.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 1.0);
    assert_eq!(white_area, 6.5);

    // Territory scoring: No territory enclosed.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 6.5);
}

// Test 3: Corner Territory Test
// A 5x5 board is filled with Black stones except one inner cell (1,1) which is empty.
// That single empty cell is completely bordered by Black stones and should be assigned as Black territory.
#[test]
fn test_corner_territory() {
    let board_size = 5;
    let total = (board_size as usize).pow(2);
    let mut occupants = vec![Occupant::Black; total];
    // Make cell (1,1) empty (neighbors at (0,1), (1,0), (1,2), (2,1) remain Black).
    occupants[(1 as usize) * (board_size as usize) + 1] = Occupant::Empty;
    let board = create_board_from_vec(occupants, board_size);

    // Territory scoring: The empty region at (1,1) should count as 1 point for Black.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 1.0);
    assert_eq!(white_territory, 6.5);

    // Area scoring: Black stones count = 24 plus territory of 1 yields 25.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 25.0);
    assert_eq!(white_area, 6.5);
}

// Test 4: Side Territory Test
// A 5x5 board is filled with White stones except one cell (2,3) is empty.
// That cell is completely bordered by White stones and should count as territory for White.
#[test]
fn test_side_territory() {
    let board_size = 5;
    let total = (board_size as usize).pow(2);
    let mut occupants = vec![Occupant::White; total];
    // Set cell (2,3) to empty.
    occupants[(2 as usize) * (board_size as usize) + 3] = Occupant::Empty;
    let board = create_board_from_vec(occupants, board_size);

    // Territory scoring: The empty cell should yield White territory = 1 (plus komi).
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    // Territory score for White is the empty region (1) plus komi.
    assert_eq!(white_territory, 1.0 + 6.5);

    // Area scoring: White stones count = 24 plus territory of 1 yields 25, then adding komi gives 6.5 added to White's base score.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    // Here, area scoring: White: 24 + 1 = 25, plus komi when calculating final result.
    // Our calculate_score function for area scoring adds komi directly to White's computed area.
    assert_eq!(white_area, 25.0 + 6.5);
    assert_eq!(black_area, 0.0);
}

// Test 5: Enclosed Ring Test
// A 3x3 board with a ring of Black stones enclosing a single empty intersection.
// Board layout:
//  B B B
//  B . B
//  B B B
// The center should count as territory for Black.
#[test]
fn test_enclosed_ring() {
    let board_size = 3;
    let mut occupants = Vec::with_capacity(9);
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 1 && col == 1 {
                occupants.push(Occupant::Empty);
            } else {
                occupants.push(Occupant::Black);
            }
        }
    }
    let board = create_board_from_vec(occupants, board_size);

    // Territory scoring: Only the center empty cell counts as Black's territory.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 1.0);
    assert_eq!(white_territory, 6.5);

    // Area scoring: Black stones = 8 plus territory 1 gives 9.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 9.0);
    assert_eq!(white_area, 6.5);
}

// Test 6: Neutral Region Test (Mixed Border)
#[test]
fn test_neutral_region() {
    let board_size = 3;
    let mut vec = vec![Occupant::Empty; 9];
    vec[0] = Occupant::Black; // (0,0)
    vec[8] = Occupant::White; // (2,2)
    let board = create_board_from_vec(vec, board_size);

    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 6.5);

    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 1.0);
    assert_eq!(white_area, 1.0 + 6.5);
}

// Test 7: Multiple Regions Test
#[test]
fn test_multiple_regions() {
    let board_size = 5;
    let total = (board_size as usize).pow(2);
    let mut vec = vec![Occupant::Black; total];
    // Region A: (2,2) is internal, so should be counted as Black territory.
    vec[(2 as usize) * (board_size as usize) + 2] = Occupant::Empty;
    // Region B: (0,4) is on the edge, so it is open.
    vec[(0 as usize) * (board_size as usize) + 4] = Occupant::Empty;
    let board = create_board_from_vec(vec, board_size);

    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    // Only Region A (internal) counts as territory.
    assert_eq!(black_territory, 1.0);
    assert_eq!(white_territory, 6.5);

    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    // Black stones: 23, plus enclosed territory 1.
    assert_eq!(black_area, 23.0 + 1.0);
    assert_eq!(white_area, 6.5);
}

// Test 8: Dead Stone Removal Effect on Scoring
#[test]
fn test_dead_stone_removal_scoring() {
    let board_size = 3;
    // Layout:
    // Row0: White, White, White
    // Row1: White, Black, White
    // Row2: White, White, White
    let mut vec = Vec::with_capacity(9);
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 1 && col == 1 {
                vec.push(Occupant::Black);
            } else {
                vec.push(Occupant::White);
            }
        }
    }
    let mut board = create_board_from_vec(vec, board_size);
    let _removed_groups = remove_dead_stones(&mut board);

    // After removal, the cell (1,1) should be Empty (with marker "removed"),
    // and all other cells should remain White.
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 1 && col == 1 {
                let spot = board.get(row, col).unwrap();
                assert_eq!(spot.occupant, Occupant::Empty);
                assert_eq!(spot.marker.as_deref(), Some("removed"));
            } else {
                assert_eq!(board.get(row, col).unwrap().occupant, Occupant::White);
            }
        }
    }

    // Scoring: For area scoring, White stones = 8 and the removed empty cell
    // will be counted as territory if fully enclosed by White.
    // In a 3x3 board, (1,1) is internal so its empty region should count for White.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    // White area = 8 (White stones) + 1 (territory from (1,1)) + komi.
    assert_eq!(black_area, 0.0);
    assert_eq!(white_area, 8.0 + 1.0 + 6.5);

    // For territory scoring, the only territory is (1,1) for White.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 1.0 + 6.5);
}
// Test 9: Complex Configuration / Real-Game Scenario
#[test]
fn test_complex_configuration_scoring() {
    let board_size = 5;
    let mut vec = Vec::with_capacity((board_size as usize).pow(2));
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 0 || row == board_size - 1 || col == 0 || col == board_size - 1 {
                vec.push(Occupant::Black);
            } else if row == 2 && col == 2 {
                vec.push(Occupant::Black);
            } else {
                vec.push(Occupant::Empty);
            }
        }
    }
    let board = create_board_from_vec(vec, board_size);

    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 8.0);
    assert_eq!(white_territory, 6.5);

    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 17.0 + 8.0);
    assert_eq!(white_area, 6.5);
}

// Test 10: Komi Application Test
#[test]
fn test_komi_application() {
    let board_size = 3;
    let mut vec = vec![Occupant::White; 9];
    vec[4] = Occupant::Empty; // center is empty and fully enclosed by White
    let board = create_board_from_vec(vec, board_size);

    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 7.5);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 1.0 + 7.5);

    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 7.5);
    assert_eq!(black_area, 0.0);
    assert_eq!(white_area, 8.0 + 1.0 + 7.5);
}

// Test 11: Single Intersection Region Test
// A board with a single empty cell that is completely enclosed (does not touch the board edge)
// by White stones should count fully as White territory.
#[test]
fn test_single_intersection_region() {
    // Use a 5x5 board so that an internal cell is fully enclosed.
    let board_size = 5;
    let total = (board_size as usize).pow(2);
    // Fill the board with White.
    let mut vec = vec![Occupant::White; total];
    // Make the central cell (2,2) empty.
    vec[(2 as usize) * (board_size as usize) + 2] = Occupant::Empty;
    let board = create_board_from_vec(vec, board_size);

    // Territory scoring: The internal empty cell is completely enclosed by White,
    // so it should count as 1 territory point for White.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 0.0);
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 1.0);

    // Area scoring: White stones = total - 1, plus territory 1 equals total.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 0.0);
    assert_eq!(white_area, (total as f32 - 1.0) + 1.0);
    assert_eq!(black_area, 0.0);
}

// Test 12: Overlapping Groups/Adjacent Groups Test
// Create a board where two separate Black groups have an empty cell between them.
// That gap should be merged (as one contiguous empty region) and count as territory for Black
// if it is fully enclosed by Black.
#[test]
fn test_overlapping_groups_territory() {
    // Use a 7x7 board.
    let board_size = 7;
    let total = (board_size as usize).pow(2);
    // Fill the board with Black.
    let mut vec = vec![Occupant::Black; total];
    // Carve out a connected empty region inside that does not touch the edge.
    // For example, remove stones from these cells:
    // (3,2), (3,3), (3,4), (4,3) -- they form a T-shape region.
    vec[(3 * board_size as usize) + 2] = Occupant::Empty;
    vec[(3 * board_size as usize) + 3] = Occupant::Empty;
    vec[(3 * board_size as usize) + 4] = Occupant::Empty;
    vec[(4 * board_size as usize) + 3] = Occupant::Empty;
    // This empty region is fully enclosed by Black and its size should be 4.
    let board = create_board_from_vec(vec, board_size);

    // Territory scoring: The empty region (if not touching the edge) counts as Black's territory.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 0.0);
    // Since the board border is Black, the empty region does not touch the edge.
    assert_eq!(black_territory, 4.0);
    assert_eq!(white_territory, 0.0);

    // Area scoring: Black stones count plus territory.
    // Total Black stones = total cells - 4, plus territory 4 equals total.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 0.0);
    assert_eq!(black_area, (total as f32 - 4.0) + 4.0);
    assert_eq!(white_area, 0.0);
}

// Test 13: Consistency Between Scoring Methods Test
// For a given board configuration, verify that although absolute scores differ,
// the winner (and relative advantage) is consistent between area and territory scoring.
#[test]
fn test_scoring_consistency() {
    // 5x5 board with a clear advantage for Black.
    // Layout:
    // Row0: B, B, B, B, B
    // Row1: B, Empty, Empty, Empty, B
    // Row2: B, Empty, B, Empty, B
    // Row3: B, Empty, Empty, Empty, B
    // Row4: B, B, B, B, B
    // Black encloses 8 empty cells as territory and has 17 stones on board.
    let board_size = 5;
    let mut vec = Vec::with_capacity((board_size as usize).pow(2));
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 0 || row == board_size - 1 || col == 0 || col == board_size - 1 {
                vec.push(Occupant::Black);
            } else if row == 2 && col == 2 {
                vec.push(Occupant::Black);
            } else {
                vec.push(Occupant::Empty);
            }
        }
    }
    let board = create_board_from_vec(vec, board_size);

    // Calculate scores using both methods (without komi).
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 0.0);
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 0.0);

    // In Territory scoring: Black gets territory = 8, White gets 0.
    // In Area scoring: Black = 17 + 8 = 25.
    // In both cases, Black wins.
    assert!(black_territory > white_territory);
    assert!(black_area > white_area);
}

// Test 14: Various Board Sizes Test
// Verify that scoring works on boards of different sizes.
#[test]
fn test_various_board_sizes() {
    // Test on 3x3 board
    {
        let board_size = 3;
        let total = (board_size as usize).pow(2);
        // A board with center empty and others Black.
        let mut vec = vec![Occupant::Black; total];
        vec[4] = Occupant::Empty; // center cell (1,1)
        let board = create_board_from_vec(vec, board_size);
        let (black_territory, _) = calculate_score(&board, ScoringMethod::Territory, 0.0);
        // In a 3x3 board, center touches all sides? Actually, center does not touch edge.
        // So territory = 1.
        assert_eq!(black_territory, 1.0);
    }
    // Test on 5x5 board (using previous complex configuration from test 9)
    {
        let board_size = 5;
        let mut vec = Vec::with_capacity((board_size as usize).pow(2));
        for row in 0..board_size {
            for col in 0..board_size {
                if row == 0 || row == board_size - 1 || col == 0 || col == board_size - 1 {
                    vec.push(Occupant::Black);
                } else if row == 2 && col == 2 {
                    vec.push(Occupant::Black);
                } else {
                    vec.push(Occupant::Empty);
                }
            }
        }
        let board = create_board_from_vec(vec, board_size);
        let (black_territory, _) = calculate_score(&board, ScoringMethod::Territory, 0.0);
        // As in test 9, internal empty region (8 cells) is fully enclosed.
        assert_eq!(black_territory, 8.0);
    }
    // Test on 9x9 board: Construct a simple scenario with a small fully enclosed territory.
    {
        let board_size = 9;
        let total = (board_size as usize).pow(2);
        // Create a board with all cells Black.
        let mut vec = vec![Occupant::Black; total];
        // Carve out an internal 3x3 empty block that does not touch the border.
        for row in 3..6 {
            for col in 3..6 {
                let idx = (row as usize) * (board_size as usize) + (col as usize);
                vec[idx] = Occupant::Empty;
            }
        }
        let board = create_board_from_vec(vec, board_size);
        let (black_territory, _) = calculate_score(&board, ScoringMethod::Territory, 0.0);
        // The empty region is 3x3 = 9 cells, and it is fully enclosed by Black.
        assert_eq!(black_territory, 9.0);
    }
}

// Test 15: Pre-Removed Dead Stones Test
// This test simulates a board where dead stones have already been removed.
// Here we create a 3x3 board that would have had a Black stone at (1,1),
// but it is pre-marked as removed (Empty with marker "removed").
// All other cells are White.
// According to Japanese territory scoring, the now-empty internal cell
// is fully enclosed by White and should count as White territory.
#[test]
fn test_pre_removed_dead_stones() {
    let board_size = 3;
    let total = (board_size as usize).pow(2);
    let mut vec = vec![Occupant::White; total]; // no extra mut removal here if needed
                                                // Compute index for (1,1): index = 1 * board_size + 1.
    let center_index = 1 * (board_size as usize) + 1;
    vec[center_index] = Occupant::Empty; // Pre-removed dead stone.
                                         // Manually create a board with the pre-removed stone.
    let board = create_board_from_vec(vec, board_size);

    // Territory scoring: the internal empty cell should count as territory for White.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    // Expected: Black 0; White gets territory of 1 plus komi.
    assert_eq!(black_territory, 0.0);
    assert_eq!(white_territory, 1.0 + 6.5);

    // Area scoring: White stones count = 8 and the empty (removed) cell is counted as territory.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 0.0);
    assert_eq!(white_area, 8.0 + 1.0 + 6.5);
}

// Test 16: No False Territory Assignment
// Ensure that an empty region which touches both Black and White is not assigned to either.
// We'll create a 3x3 board with:
// Row0: Black, Empty, White
// Rows1-2: all Empty.
// The large empty region touches both Black and White, so neither side gets territory.
#[test]
fn test_no_false_territory_assignment() {
    let board_size = 3;
    let total = (board_size as usize).pow(2);
    // Start with an entirely empty board.
    let vec = vec![Occupant::Empty; total];
    // Place a Black stone at (0,0) and a White stone at (0,2).
    let mut board_vec = vec.clone();
    board_vec[0] = Occupant::Black; // (0,0)
    board_vec[2] = Occupant::White; // (0,2)
    let board = create_board_from_vec(board_vec, board_size);

    // Territory scoring: the single empty region touches both colors,
    // so no territory should be awarded.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 0.0);
    // White only gets komi.
    assert_eq!(white_territory, 6.5);

    // Area scoring: No enclosed territory, so only stones count.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    // Black has 1 stone, White has 1 stone, plus komi for White.
    assert_eq!(black_area, 1.0);
    assert_eq!(white_area, 1.0 + 6.5);
}

// Test 17: Known Board Position from Literature
// Use a known 5x5 configuration (from literature or trusted source) where:
// - Black forms a border around an inner region.
// - Board layout:
//   Row0: B, B, B, B, B
//   Row1: B, Empty, Empty, Empty, B
//   Row2: B, Empty, B, Empty, B
//   Row3: B, Empty, Empty, Empty, B
//   Row4: B, B, B, B, B
// Expected for Territory scoring:
//   Black territory = 8 (the internal empty region, not touching the edge)
//   White territory = 0 (plus komi for White)
// Expected for Area scoring:
//   Black stones = 17, plus territory = 8 => 25.
#[test]
fn test_known_board_position() {
    let board_size = 5;
    let mut vec = Vec::with_capacity((board_size as usize).pow(2));
    for row in 0..board_size {
        for col in 0..board_size {
            if row == 0 || row == board_size - 1 || col == 0 || col == board_size - 1 {
                vec.push(Occupant::Black);
            } else if row == 2 && col == 2 {
                vec.push(Occupant::Black);
            } else {
                vec.push(Occupant::Empty);
            }
        }
    }
    let board = create_board_from_vec(vec, board_size);

    // Territory scoring: Only the internal empty region counts.
    let (black_territory, white_territory) = calculate_score(&board, ScoringMethod::Territory, 6.5);
    assert_eq!(black_territory, 8.0);
    assert_eq!(white_territory, 6.5);

    // Area scoring: Black stones = 17 + territory (8) = 25.
    let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
    assert_eq!(black_area, 17.0 + 8.0);
    assert_eq!(white_area, 6.5);
}
