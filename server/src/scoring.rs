use crate::models::{Board, EmptyRegion, Game, Group, Occupant, ScoringMethod, SpotState};

use std::collections::{HashSet, VecDeque};

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
/// the whole region is assigned as that color’s territory.
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

/// Perform scoring analysis on a game.
/// This function deserializes the board, runs in-place annotation,
/// calculates the current score, and then updates the game accordingly.
pub fn analyze_game(mut game: Game) -> Game {
    // Deserialize the board from JSON into a Vec<SpotState>
    let board_vec: Vec<SpotState> =
        serde_json::from_str(&game.board).expect("Failed to deserialize board");
    let mut board_obj = crate::models::Board::new(board_vec, game.board_size);

    // Run in-place annotation to update scoring fields on each spot.
    board_obj.annotate_for_scoring();

    // Calculate the score using our existing function (using Area scoring here).
    let (black_score, white_score) = calculate_score(&board_obj, ScoringMethod::Area, 6.5);

    // Update game fields with the computed score.
    game.final_score_black = Some(black_score);
    game.final_score_white = Some(white_score);

    // Re-serialize the annotated board so the client receives detailed insights.
    game.board = serde_json::to_string(&board_obj.spots).expect("Failed to serialize board");

    game
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
