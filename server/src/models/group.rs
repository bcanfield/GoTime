use std::collections::HashSet;
use super::spot::Occupant;

/// Represents a connected chain of stones of the same color in a Go game.
///
/// A group (also called a "chain" or "string") is a set of connected stones of the
/// same color. In Go, connected means orthogonally adjacent (not diagonally).
/// Each group has a set of liberties, which are empty adjacent points.
#[derive(Debug, Clone)]
pub struct Group {
    /// The color of all stones in this group
    pub occupant: Occupant,
    
    /// Coordinates of all stones in this group as (row, column) pairs
    pub stones: Vec<(u8, u8)>,
    
    /// Set of coordinates for all empty spots adjacent to this group
    pub liberties: HashSet<(u8, u8)>,
}