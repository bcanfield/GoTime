use super::board::Board;
use spacetimedb::{table, Identity, Timestamp};

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
        let spots = serde_json::from_str(&self.board)?;
        Ok(Board::new(spots, self.board_size))
    }
}

/// An enum to choose between scoring methods.
#[derive(Debug)]
pub enum ScoringMethod {
    Area,      // Area scoring: stones on board + enclosed territory.
    Territory, // Territory scoring: enclosed territory (and captured stones, if tracked).
}
