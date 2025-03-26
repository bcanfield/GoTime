use super::board::Board;
use spacetimedb::{table, Identity, Timestamp};

/// Represents a user in the Go game system.
#[table(name = user, public)]
pub struct User {
    /// Unique identifier for the user
    #[primary_key]
    pub identity: Identity,
    
    /// Display name chosen by the user
    pub name: Option<String>,
    
    /// Whether the user is currently connected
    pub online: bool,
}

/// Represents a chat message in the system.
#[table(name = message, public)]
pub struct Message {
    /// Identity of the user who sent this message
    pub sender: Identity,
    
    /// When the message was sent
    pub sent: Timestamp,
    
    /// Content of the message
    pub text: String,
}

/// Represents a Go game instance.
#[table(name = game, public)]
pub struct Game {
    /// Unique identifier for the game
    #[primary_key]
    pub id: u64,
    
    /// Identity of the player playing Black stones
    pub player_black: Identity,
    
    /// Identity of the player playing White stones (None if waiting for opponent)
    pub player_white: Option<Identity>,
    
    /// JSON-serialized string representation of the game board
    pub board: String,
    
    /// Current player's turn: "B" for Black or "W" for White
    pub turn: String,
    
    /// Number of consecutive passes
    pub passes: u8,
    
    /// Size of the board (typically 9x9, 13x13, or 19x19)
    pub board_size: u8,
    
    /// Previous board state for ko rule checking
    pub previous_board: Option<String>,
    
    /// Whether the game has concluded
    pub game_over: bool,
    
    /// Final score for the Black player
    pub final_score_black: Option<f32>,
    
    /// Final score for the White player
    pub final_score_white: Option<f32>,
}

impl Game {
    /// Converts the JSON-serialized board into a Board struct.
    /// 
    /// # Returns
    /// - `Ok(Board)` - Board instance deserialized from the JSON
    /// - `Err` - If deserialization fails
    pub fn as_board(&self) -> Result<Board, serde_json::Error> {
        let spots = serde_json::from_str(&self.board)?;
        Ok(Board::new(spots, self.board_size))
    }
}

/// Defines different methods for scoring a Go game.
#[derive(Debug, Clone, Copy)]
pub enum ScoringMethod {
    /// Area scoring: stones on board + enclosed territory
    Area,
    /// Territory scoring: enclosed territory only (plus captured stones)
    Territory,
}
