use super::spot::{Occupant, SpotState};
use crate::scoring::{find_empty_regions, find_groups};

/// Represents a Go game board with its current state.
///
/// The board consists of a grid of spots arranged in a square. Each spot can be
/// empty or occupied by a black or white stone. The board is stored as a flat vector
/// in row-major order (i.e., each row is stored consecutively).
#[derive(Clone, Debug)]
pub struct Board {
    /// The size of the board (e.g., 9 for a 9×9 board)
    pub board_size: u8,

    /// The state of each spot on the board, stored in row-major order
    pub spots: Vec<SpotState>,
}

impl Board {
    /// Creates a new board from a vector of SpotState and a given board size.
    ///
    /// # Arguments
    /// * `spots` - Vector of spot states, must contain board_size² elements
    /// * `board_size` - Size of the board (e.g., 9, 13, 19)
    ///
    /// # Panics
    /// Panics if the number of spots does not equal board_size².
    pub fn new(spots: Vec<SpotState>, board_size: u8) -> Self {
        assert_eq!(
            spots.len(),
            (board_size as usize).pow(2),
            "Invalid board: contains {} spots but expected {} for size {}",
            spots.len(),
            (board_size as usize).pow(2),
            board_size
        );
        Board { board_size, spots }
    }

    /// Converts a (row, col) coordinate to the corresponding index in the spots vector.
    ///
    /// # Arguments
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    pub fn index(&self, row: u8, col: u8) -> usize {
        (row as usize) * (self.board_size as usize) + (col as usize)
    }

    /// Gets an immutable reference to the spot at (row, col) if the coordinates are within bounds.
    ///
    /// # Arguments
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    ///
    /// # Returns
    /// * `Some(&SpotState)` - Reference to the spot if coordinates are valid
    /// * `None` - If coordinates are out of bounds
    pub fn get(&self, row: u8, col: u8) -> Option<&SpotState> {
        if row < self.board_size && col < self.board_size {
            Some(&self.spots[self.index(row, col)])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the spot at (row, col) if the coordinates are within bounds.
    ///
    /// # Arguments
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    ///
    /// # Returns
    /// * `Some(&mut SpotState)` - Mutable reference to the spot if coordinates are valid
    /// * `None` - If coordinates are out of bounds
    pub fn get_mut(&mut self, row: u8, col: u8) -> Option<&mut SpotState> {
        if row < self.board_size && col < self.board_size {
            let idx = self.index(row, col);
            Some(&mut self.spots[idx])
        } else {
            None
        }
    }

    /// Returns all valid orthogonal neighbor coordinates of a given position.
    ///
    /// In Go, only orthogonally adjacent positions (not diagonals) are considered connected.
    ///
    /// # Arguments
    /// * `row` - Zero-based row index
    /// * `col` - Zero-based column index
    ///
    /// # Returns
    /// Vector of (row, col) tuples representing the valid neighboring positions
    pub fn neighbors(&self, row: u8, col: u8) -> Vec<(u8, u8)> {
        let mut result = Vec::with_capacity(4); // At most 4 orthogonal neighbors

        if row > 0 {
            result.push((row - 1, col)); // North
        }
        if row < self.board_size - 1 {
            result.push((row + 1, col)); // South
        }
        if col > 0 {
            result.push((row, col - 1)); // West
        }
        if col < self.board_size - 1 {
            result.push((row, col + 1)); // East
        }

        result
    }

    /// Annotates each empty spot with scoring metadata based on territory analysis.
    ///
    /// This function updates the `scoring_owner` and `scoring_explanation` fields in-place.
    /// Only empty regions are annotated because they are the only ones that can be scored.
    pub fn annotate_for_scoring(&mut self) {
        // Clear any previous scoring info.
        for spot in self.spots.iter_mut() {
            spot.scoring_owner = None;
            spot.scoring_explanation = None;
        }

        let regions = find_empty_regions(self);
        log::info!("Found {} empty regions", regions.len());

        for region in regions {
            if region.touches_edge {
                // In traditional Go scoring, regions that touch the edge are not territory
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = None;
                        spot.scoring_explanation = Some("Open (touches edge)".to_string());
                    }
                }
            } else if region.border.len() == 1 {
                // Region surrounded by stones of just one color - counts as territory for that color
                let owner = region.border.iter().next().unwrap().clone();
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = Some(owner.clone());
                        spot.scoring_explanation = Some(format!("Cell enclosed by {:?}", owner));
                    }
                }
            } else {
                // Region with mixed borders - neutral points ("dame")
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = None;
                        spot.scoring_explanation = Some("Neutral".to_string());
                    }
                }
            }
        }
    }

    /// Annotates each empty spot with a "playable" flag based on move legality.
    ///
    /// A move is legal if:
    /// 1. The spot is empty
    /// 2. The resulting group has at least one liberty, or captures an enemy group
    /// 3. The move doesn't violate the ko rule
    ///
    /// # Arguments
    /// * `current_turn` - Which player is currently moving
    pub fn annotate_playability(&mut self, current_turn: Occupant) {
        for row in 0..self.board_size {
            for col in 0..self.board_size {
                // Compute playability in an inner scope to avoid borrowing conflicts.
                let playable = {
                    if let Some(spot) = self.get(row, col) {
                        if spot.occupant != Occupant::Empty {
                            false
                        } else {
                            // Use an immutable borrow for simulation.
                            Self::is_move_playable(&*self, row, col, current_turn.clone())
                        }
                    } else {
                        false
                    }
                };

                // Now update the spot mutably.
                if let Some(spot_mut) = self.get_mut(row, col) {
                    spot_mut.playable = playable;
                }
            }
        }
    }

    /// Determines if a move at the given position would be legal.
    ///
    /// This simulates placing a stone and applies all Go rules:
    /// - Cannot place on an occupied spot
    /// - Cannot commit suicide (unless it captures enemy stones)
    ///
    /// Note: This function does not check for ko rule violations
    /// since that requires previous board state.
    ///
    /// # Arguments
    /// * `board` - Current board state
    /// * `row` - Zero-based row index for the move
    /// * `col` - Zero-based column index for the move
    /// * `stone_color` - Color of the stone to place
    ///
    /// # Returns
    /// `true` if the move is legal, `false` otherwise
    fn is_move_playable(board: &Board, row: u8, col: u8, stone_color: Occupant) -> bool {
        // 1. If the spot is already occupied, it's unplayable.
        if let Some(s) = board.get(row, col) {
            if s.occupant != Occupant::Empty {
                return false;
            }
        }

        // 2. Clone the board and simulate placing the stone.
        let mut simulated_spots = board.spots.clone();
        let idx = board.index(row, col);
        simulated_spots[idx].occupant = stone_color.clone();
        simulated_spots[idx].move_number = Some(0); // dummy move number for simulation
        let mut sim_board = Board::new(simulated_spots, board.board_size);

        // 3. For each neighbor, if it's an enemy stone, check if its group now has no liberties.
        let mut captured_something = false;
        for (nr, nc) in board.neighbors(row, col) {
            if let Some(neighbor) = sim_board.get(nr, nc) {
                if neighbor.occupant != Occupant::Empty && neighbor.occupant != stone_color {
                    // Recompute groups in the simulated board.
                    let groups = find_groups(&sim_board);
                    for group in groups.iter() {
                        // If the enemy group (not our color) that includes this neighbor has no liberties...
                        if group.occupant != stone_color
                            && group.stones.contains(&(nr, nc))
                            && group.liberties.is_empty()
                        {
                            captured_something = true;

                            // Remove every stone in that group.
                            for (r, c) in group.stones.iter() {
                                if let Some(spot) = sim_board.get_mut(*r, *c) {
                                    spot.occupant = Occupant::Empty;
                                    spot.move_number = None;
                                    spot.marker = Some("captured".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Recompute groups in the simulated board after simulated captures.
        let groups = find_groups(&sim_board);

        // Find the group that contains our newly placed stone.
        for group in groups {
            if group.stones.contains(&(row, col)) {
                // The move is legal if the new stone's group has at least one liberty
                // or if we captured something (which would give us liberties)
                return !group.liberties.is_empty() || captured_something;
            }
        }

        // If we get here, something went wrong (the placed stone's group wasn't found)
        false
    }
}
