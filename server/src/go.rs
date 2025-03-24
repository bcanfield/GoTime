// board_evaluator.rs

use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

/// The Occupant enum represents what is on a given board spot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Occupant {
    Empty,
    Black,
    White,
}

/// A SpotState holds the state for one board intersection.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotState {
    pub occupant: Occupant,
    pub move_number: Option<u64>,
    pub marker: Option<String>,
}

/// The Game struct from your data model is left elsewhere.
/// Here we work on the board as a Vec<SpotState> and the board size.

/// The Board type wraps a vector of SpotState with its board size.
/// We assume the board is stored in row-major order.
#[derive(Clone, Debug)]
pub struct Board {
    pub board_size: u8,
    pub spots: Vec<SpotState>,
}

impl Board {
    /// Create a new board from a vector of SpotState and a given board size.
    /// Panics if the number of spots does not equal board_size².
    pub fn new(spots: Vec<SpotState>, board_size: u8) -> Self {
        assert_eq!(spots.len(), (board_size as usize).pow(2));
        Board { board_size, spots }
    }

    /// Compute the index in the vector from a (row, col) coordinate.
    pub fn index(&self, row: u8, col: u8) -> usize {
        (row as usize) * (self.board_size as usize) + (col as usize)
    }

    /// Get an immutable reference to the spot at (row, col) if in bounds.
    pub fn get(&self, row: u8, col: u8) -> Option<&SpotState> {
        if row < self.board_size && col < self.board_size {
            Some(&self.spots[self.index(row, col)])
        } else {
            None
        }
    }

    /// Get a mutable reference to the spot at (row, col) if in bounds.
    pub fn get_mut(&mut self, row: u8, col: u8) -> Option<&mut SpotState> {
        if row < self.board_size && col < self.board_size {
            let idx = (row as usize) * (self.board_size as usize) + (col as usize);
            Some(&mut self.spots[idx])
        } else {
            None
        }
    }

    /// Return all valid orthogonal neighbor coordinates of (row, col).
    pub fn neighbors(&self, row: u8, col: u8) -> Vec<(u8, u8)> {
        let mut result = Vec::new();
        if row > 0 {
            result.push((row - 1, col));
        }
        if row < self.board_size - 1 {
            result.push((row + 1, col));
        }
        if col > 0 {
            result.push((row, col - 1));
        }
        if col < self.board_size - 1 {
            result.push((row, col + 1));
        }
        result
    }
}

/// A Group represents a connected chain of stones of a given color, along with its liberties.
#[derive(Debug)]
pub struct Group {
    pub occupant: Occupant,
    pub stones: Vec<(u8, u8)>,
    pub liberties: HashSet<(u8, u8)>,
}

/// Find all stone groups (i.e. connected components) on the board.
/// Empty spots are skipped.
pub fn find_groups(board: &Board) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut visited = vec![false; board.spots.len()];

    for row in 0..board.board_size {
        for col in 0..board.board_size {
            let idx = board.index(row, col);
            if visited[idx] {
                continue;
            }
            let spot = &board.spots[idx];
            match spot.occupant {
                Occupant::Empty => {
                    visited[idx] = true; // skip empties
                }
                ref occ @ Occupant::Black | ref occ @ Occupant::White => {
                    let mut group = Group {
                        occupant: occ.clone(),
                        stones: Vec::new(),
                        liberties: HashSet::new(),
                    };
                    let mut stack = vec![(row, col)];
                    while let Some((r, c)) = stack.pop() {
                        let i = board.index(r, c);
                        if visited[i] {
                            continue;
                        }
                        visited[i] = true;
                        let current = &board.spots[i];
                        if current.occupant == *occ {
                            group.stones.push((r, c));
                            // Check each neighbor
                            for (nr, nc) in board.neighbors(r, c) {
                                let neighbor = board.get(nr, nc).unwrap();
                                if neighbor.occupant == *occ && !visited[board.index(nr, nc)] {
                                    stack.push((nr, nc));
                                } else if neighbor.occupant == Occupant::Empty {
                                    group.liberties.insert((nr, nc));
                                }
                            }
                        }
                    }
                    groups.push(group);
                }
            }
        }
    }
    groups
}

/// Remove dead stones from the board.
/// A stone group with zero liberties is considered dead and is removed (set to Empty).
/// Returns a vector of removed (dead) groups.
pub fn remove_dead_stones(board: &mut Board) -> Vec<Group> {
    let groups = find_groups(board);
    let mut removed_groups = Vec::new();
    for group in groups {
        if group.liberties.is_empty()
            && group.stones.iter().all(|&(r, c)| {
                r != 0 && r != board.board_size - 1 && c != 0 && c != board.board_size - 1
            })
        {
            for (r, c) in &group.stones {
                if let Some(spot) = board.get_mut(*r, *c) {
                    spot.occupant = Occupant::Empty;
                    spot.move_number = None;
                    spot.marker = Some("removed".to_string());
                }
            }
            removed_groups.push(group);
        }
    }
    removed_groups
}

/// Represents a contiguous empty region (dame) along with the colors of adjacent stones.
#[derive(Debug)]
pub struct EmptyRegion {
    pub spots: Vec<(u8, u8)>,
    pub border: HashSet<Occupant>, // Colors of adjacent stones.
    pub touches_edge: bool,
}

/// Find contiguous empty regions using a flood-fill algorithm.
pub fn find_empty_regions(board: &Board) -> Vec<EmptyRegion> {
    let mut regions = Vec::new();
    let mut visited = vec![false; board.spots.len()];

    for row in 0..board.board_size {
        for col in 0..board.board_size {
            let idx = board.index(row, col);
            if visited[idx] {
                continue;
            }
            let spot = &board.spots[idx];
            if spot.occupant != Occupant::Empty {
                visited[idx] = true;
                continue;
            }
            // Start a new empty region.
            let mut region = EmptyRegion {
                spots: Vec::new(),
                border: HashSet::new(),
                touches_edge: false,
            };
            let mut queue = VecDeque::new();
            queue.push_back((row, col));
            while let Some((r, c)) = queue.pop_front() {
                let i = board.index(r, c);
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                region.spots.push((r, c));
                // Mark if this cell touches the board edge.
                if r == 0 || r == board.board_size - 1 || c == 0 || c == board.board_size - 1 {
                    region.touches_edge = true;
                }
                for (nr, nc) in board.neighbors(r, c) {
                    let neighbor = board.get(nr, nc).unwrap();
                    if neighbor.occupant == Occupant::Empty {
                        if !visited[board.index(nr, nc)] {
                            queue.push_back((nr, nc));
                        }
                    } else {
                        region.border.insert(neighbor.occupant.clone());
                    }
                }
            }
            regions.push(region);
        }
    }
    regions
}

/// Determine the territory (empty intersections) for each color.
/// For each empty region, if all adjacent stones are of a single color,
/// the whole region is assigned as that color’s territory.
pub fn determine_territory(board: &Board) -> (u64, u64) {
    let regions = find_empty_regions(board);
    let mut black_territory = 0;
    let mut white_territory = 0;
    for region in regions {
        if region.touches_edge {
            // The region is open – not fully enclosed – so do not count it as territory.
            continue;
        }
        if region.border.len() == 1 {
            // assign the region's spots as territory for the single bordering color.
            let color = region.border.iter().next().unwrap();
            match color {
                Occupant::Black => black_territory += region.spots.len() as u64,
                Occupant::White => white_territory += region.spots.len() as u64,
                Occupant::Empty => {}
            }
        }
    }
    (black_territory, white_territory)
}

/// An enum to choose between scoring methods.
#[derive(Debug)]
pub enum ScoringMethod {
    Area,      // Area scoring: stones on board + enclosed territory.
    Territory, // Territory scoring: enclosed territory (and captured stones, if tracked).
}

/// Calculate scores for Black and White.
/// For Area scoring, score = (number of stones) + (empty intersections in territory).
/// For Territory scoring, we use just the territory (plus komi to White).
/// (Komi is added as a float bonus to White.)
pub fn calculate_score(board: &Board, method: ScoringMethod, komi: f32) -> (f32, f32) {
    // Count stones on board.
    let mut black_stones = 0;
    let mut white_stones = 0;
    for spot in &board.spots {
        match spot.occupant {
            Occupant::Black => black_stones += 1,
            Occupant::White => white_stones += 1,
            _ => {}
        }
    }
    // Evaluate empty territory.
    let (black_territory, white_territory) = determine_territory(board);
    match method {
        ScoringMethod::Area => {
            let black_score = black_stones as f32 + black_territory as f32;
            let white_score = white_stones as f32 + white_territory as f32 + komi;
            (black_score, white_score)
        }
        ScoringMethod::Territory => {
            let black_score = black_territory as f32;
            let white_score = white_territory as f32 + komi;
            (black_score, white_score)
        }
    }
}

/// Create a simple annotation for each spot (useful for debugging).
/// Occupied spots are annotated with their color and move number,
/// while empty spots are annotated as "Neutral".
// pub fn annotate_board(board: &Board) -> Vec<Vec<String>> {
//     let mut annotations =
//         vec![vec!["".to_string(); board.board_size as usize]; board.board_size as usize];
//     for row in 0..board.board_size {
//         for col in 0..board.board_size {
//             let spot = board.get(row, col).unwrap();
//             let annotation = match spot.occupant {
//                 Occupant::Black => format!("Black (move {:?})", spot.move_number),
//                 Occupant::White => format!("White (move {:?})", spot.move_number),
//                 Occupant::Empty => "Neutral".to_string(),
//             };
//             annotations[row as usize][col as usize] = annotation;
//         }
//     }
//     annotations
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: create a Board from a vector of Occupant values.
    fn create_board_from_vec(vec: Vec<Occupant>, board_size: u8) -> Board {
        let spots: Vec<SpotState> = vec
            .into_iter()
            .map(|occ| SpotState {
                occupant: occ,
                move_number: None,
                marker: None,
            })
            .collect();
        Board::new(spots, board_size)
    }

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
        // The black group should have no liberties.
        let groups = find_groups(&board);
        let black_group = groups
            .into_iter()
            .find(|g| g.occupant == Occupant::Black)
            .unwrap();
        assert!(black_group.liberties.is_empty());
        // Remove dead stones.
        let removed_groups = remove_dead_stones(&mut board);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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

        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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

        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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

        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
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

        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 7.5);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 0.0);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 0.0);
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
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 0.0);
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
}
