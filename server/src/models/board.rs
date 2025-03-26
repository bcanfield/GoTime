use super::spot::{Occupant, SpotState};
use crate::scoring::{find_empty_regions, find_groups};

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
    /// Annotate each empty spot with scoring metadata based on territory.
    /// This function updates the `scoring_owner` and `scoring_explanation` fields in-place.
    /// Only empty regions are annotated because they are the only ones that can be scored.
    pub fn annotate_for_scoring(&mut self) {
        // Clear any previous scoring info.
        for spot in self.spots.iter_mut() {
            spot.scoring_owner = None;
            spot.scoring_explanation = None;
        }
        log::info!("{}", "In here");

        let regions = find_empty_regions(self);
        // log number of regions
        log::info!("Found {} empty regions", regions.len());

        for region in regions {
            if region.touches_edge {
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = None;
                        spot.scoring_explanation = Some("Open (touches edge)".to_string());
                    }
                }
            } else if region.border.len() == 1 {
                let owner = region.border.iter().next().unwrap().clone();
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = Some(owner.clone());
                        spot.scoring_explanation = Some(format!("Cell enclosed by {:?}", owner));
                    }
                }
            } else {
                for (r, c) in region.spots {
                    if let Some(spot) = self.get_mut(r, c) {
                        spot.scoring_owner = None;
                        spot.scoring_explanation = Some("Neutral".to_string());
                    }
                }
            }
        }
    }

    /// Annotate each empty spot with a "playable" flag based on whether a move
    /// by the current player would have at least one liberty.
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
                // The move is legal if the new stone's group has at least one liberty.
                return !group.liberties.is_empty();
            }
        }
        false
    }
}
