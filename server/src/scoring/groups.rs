use crate::models::{Board, Group, Occupant};
use std::collections::HashSet;

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