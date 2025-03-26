use crate::models::{Board, EmptyRegion, Occupant};
use std::collections::{HashSet, VecDeque};

/// Find contiguous empty regions using a flood-fill algorithm.
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
/// the whole region is assigned as that color's territory.
pub fn determine_territory(board: &Board) -> (u64, u64) {
    let regions: Vec<EmptyRegion> = find_empty_regions(board);
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