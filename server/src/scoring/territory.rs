use crate::models::{Board, EmptyRegion, Occupant};
use std::collections::{HashSet, VecDeque};

/// Finds contiguous regions of empty spaces on the board using a flood-fill algorithm.
///
/// This is a key function for scoring in Go, as it identifies potential territory.
/// Each empty region is analyzed to determine:
/// - All empty points in the region
/// - Which player colors border the region
/// - Whether the region touches the edge of the board
///
/// # Arguments
/// * `board` - The game board to analyze
///
/// # Returns
/// A vector of EmptyRegion objects describing each connected empty area
pub fn find_empty_regions(board: &Board) -> Vec<EmptyRegion> {
    let mut regions: Vec<EmptyRegion> = Vec::new();
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
            
            // Start a new empty region
            let mut region = EmptyRegion {
                spots: Vec::new(),
                border: HashSet::new(),
                touches_edge: false,
            };
            
            // Use breadth-first search to find all connected empty spaces
            let mut queue = VecDeque::new();
            queue.push_back((row, col));
            
            while let Some((r, c)) = queue.pop_front() {
                let i = board.index(r, c);
                if visited[i] {
                    continue;
                }
                
                visited[i] = true;
                region.spots.push((r, c));
                
                // Check if this empty point touches the board edge
                if r == 0 || r == board.board_size - 1 || c == 0 || c == board.board_size - 1 {
                    region.touches_edge = true;
                }
                
                // Explore neighbors
                for (nr, nc) in board.neighbors(r, c) {
                    let neighbor = board.get(nr, nc).unwrap();
                    
                    if neighbor.occupant == Occupant::Empty {
                        // Add unvisited empty neighbors to the queue
                        if !visited[board.index(nr, nc)] {
                            queue.push_back((nr, nc));
                        }
                    } else {
                        // Add non-empty neighbors to the border set
                        region.border.insert(neighbor.occupant.clone());
                    }
                }
            }
            
            regions.push(region);
        }
    }
    
    regions
}

/// Calculates the territory (empty intersections) for each player.
///
/// In Go scoring, an empty region is considered territory for a player if:
/// 1. It is fully enclosed (doesn't touch the edge of the board)
/// 2. It is bordered by stones of only one color
///
/// # Arguments
/// * `board` - The game board to analyze
///
/// # Returns
/// A tuple (black_territory, white_territory) with the count of territory points for each player
pub fn determine_territory(board: &Board) -> (u64, u64) {
    let regions = find_empty_regions(board);
    let mut black_territory = 0;
    let mut white_territory = 0;
    
    for region in regions {
        // Regions that touch the edge are not considered territory in traditional Go rules
        if region.touches_edge {
            continue;
        }
        
        // If the region is bordered by stones of only one color, it's that player's territory
        if region.border.len() == 1 {
            let color = region.border.iter().next().unwrap();
            match color {
                Occupant::Black => black_territory += region.spots.len() as u64,
                Occupant::White => white_territory += region.spots.len() as u64,
                Occupant::Empty => {} // This shouldn't happen logically
            }
        }
        // Regions with mixed borders (dame) don't count as territory for either player
    }
    
    (black_territory, white_territory)
}