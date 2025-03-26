use serde::{Deserialize, Serialize};

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
    pub playable: bool, // true if the spot is legal to play

    /// Optional field indicating which player gets the point for this spot.
    pub scoring_owner: Option<Occupant>,
    /// Optional explanation for scoring (e.g. "enclosed by Black", "Neutral", etc.)
    pub scoring_explanation: Option<String>,
}