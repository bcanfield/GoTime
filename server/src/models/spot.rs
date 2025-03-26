use serde::{Deserialize, Serialize};

/// Represents what occupies a position on the Go board.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Occupant {
    /// No stone present
    Empty,
    /// Black stone
    Black,
    /// White stone
    White,
}

/// Represents the complete state of a single position on the Go board.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpotState {
    /// The stone (or lack thereof) at this position
    pub occupant: Occupant,

    /// Move number when this spot was last modified, if any
    pub move_number: Option<u64>,

    /// Optional marker for UI display (e.g., "removed", "last_move")
    pub marker: Option<String>,

    /// Whether placing a stone here would be a legal move
    pub playable: bool,

    /// Which player gets a point for this spot during scoring
    pub scoring_owner: Option<Occupant>,

    /// Human-readable explanation for why this spot was scored as it was
    pub scoring_explanation: Option<String>,
}
