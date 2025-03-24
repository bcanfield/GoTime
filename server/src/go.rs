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
        if region.border.len() == 1 {
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
pub fn annotate_board(board: &Board) -> Vec<Vec<String>> {
    let mut annotations =
        vec![vec!["".to_string(); board.board_size as usize]; board.board_size as usize];
    for row in 0..board.board_size {
        for col in 0..board.board_size {
            let spot = board.get(row, col).unwrap();
            let annotation = match spot.occupant {
                Occupant::Black => format!("Black (move {:?})", spot.move_number),
                Occupant::White => format!("White (move {:?})", spot.move_number),
                Occupant::Empty => "Neutral".to_string(),
            };
            annotations[row as usize][col as usize] = annotation;
        }
    }
    annotations
}

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
        // The empty region should have a border that includes Black.
        let board_size = 3;
        let mut vec = vec![Occupant::Empty; 9];
        vec[0] = Occupant::Black;
        vec[1] = Occupant::Black;
        let board = create_board_from_vec(vec, board_size);
        let regions = find_empty_regions(&board);
        assert_eq!(regions.len(), 1);
        let region = &regions[0];
        assert!(region.border.contains(&Occupant::Black));
        // Determine territory.
        let (black_territory, white_territory) = determine_territory(&board);
        // Since the only border color is Black, the entire empty region is Black territory.
        assert_eq!(black_territory, region.spots.len() as u64);
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

    #[test]
    fn test_calculate_score_empty_board() {
        // 3x3 board completely empty.
        let board_size = 3;
        let total = (board_size as usize).pow(2);
        let board = create_board_from_vec(vec![Occupant::Empty; total], board_size);

        // For an entirely empty board, no territory is assigned.
        // Both scoring methods should yield 0 for Black and komi for White.
        let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
        assert_eq!(black_area, 0.0);
        assert_eq!(white_area, 6.5);

        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
        assert_eq!(black_territory, 0.0);
        assert_eq!(white_territory, 6.5);
    }

    #[test]
    fn test_calculate_score_single_black_center() {
        // 3x3 board: one Black stone in the center, rest empty.
        // This should yield an empty region of 8 spots bordered solely by Black.
        let board_size = 3;
        let mut occupants = vec![Occupant::Empty; 9];
        occupants[4] = Occupant::Black; // center cell
        let board = create_board_from_vec(occupants, board_size);

        // Area scoring: Black stones = 1, territory = 8, total = 9.
        // White gets only the komi.
        let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
        assert_eq!(black_area, 9.0);
        assert_eq!(white_area, 6.5);

        // Territory scoring: territory = 8 for Black, so Black gets 8; White gets komi.
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
        assert_eq!(black_territory, 8.0);
        assert_eq!(white_territory, 6.5);
    }

    #[test]
    fn test_calculate_score_enclosed_ring() {
        // 3x3 board: Black stones form a ring around the center.
        // Board layout:
        //  B B B
        //  B . B
        //  B B B
        // Here, the only empty cell (center) should count as territory for Black.
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

        // Area scoring: Black stones = 8, territory = 1, total = 9; White gets only komi.
        let (black_area, white_area) = calculate_score(&board, ScoringMethod::Area, 6.5);
        assert_eq!(black_area, 9.0);
        assert_eq!(white_area, 6.5);

        // Territory scoring: Black territory = 1, White = komi.
        let (black_territory, white_territory) =
            calculate_score(&board, ScoringMethod::Territory, 6.5);
        assert_eq!(black_territory, 1.0);
        assert_eq!(white_territory, 6.5);
    }
}
