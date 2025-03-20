use serde::{Deserialize, Serialize};
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
    pub final_score_black: Option<u64>,
    pub final_score_white: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
