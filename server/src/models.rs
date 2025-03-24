use serde::{Deserialize, Serialize};
use spacetimedb::{table, Identity, Timestamp};
use std::collections::HashSet;

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}

#[table(name = message, public)]
pub struct Message {
    pub sender: Identity,
    pub sent: Timestamp,
    pub text: String,
}

#[table(name = game, public)]
pub struct Game {
    #[primary_key]
    pub id: u64,
    pub player_black: Identity,
    pub player_white: Option<Identity>,
    pub board: String, // JSON serialized Vec<SpotState>
    pub turn: String,  // "B" for Black or "W" for White
    pub passes: u8,
    pub board_size: u8,
    pub previous_board: Option<String>, // For simple ko checking
    pub game_over: bool,
    pub final_score_black: Option<f32>,
    pub final_score_white: Option<f32>,
}

impl Game {
    /// Convert the JSON-serialized board into a Board struct.
    pub fn as_board(&self) -> Result<Board, serde_json::Error> {
        let spots: Vec<SpotState> = serde_json::from_str(&self.board)?;
        Ok(Board::new(spots, self.board_size))
    }
}

/// The Occupant enum represents what is on a given board spot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Occupant {
    Empty,
    Black,
    White,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotState {
    pub occupant: Occupant,
    pub move_number: Option<u64>,
    pub marker: Option<String>,
}

/// A Group represents a connected chain of stones of a given color, along with its liberties.
#[derive(Debug)]
pub struct Group {
    pub occupant: Occupant,
    pub stones: Vec<(u8, u8)>,
    pub liberties: HashSet<(u8, u8)>,
}

/// The Board type wraps a vector of SpotState with its board size.
/// We assume the board is stored in row-major order.
#[derive(Clone, Debug)]
pub struct Board {
    pub board_size: u8,
    pub spots: Vec<SpotState>,
}

/// Represents a contiguous empty region (dame) along with the colors of adjacent stones.
#[derive(Debug)]
pub struct EmptyRegion {
    pub spots: Vec<(u8, u8)>,
    pub border: HashSet<Occupant>, // Colors of adjacent stones.
    pub touches_edge: bool,
}

/// An enum to choose between scoring methods.
#[derive(Debug)]
pub enum ScoringMethod {
    Area,      // Area scoring: stones on board + enclosed territory.
    Territory, // Territory scoring: enclosed territory (and captured stones, if tracked).
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
