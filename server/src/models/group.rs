use std::collections::HashSet;
use super::spot::Occupant;

/// A Group represents a connected chain of stones of a given color, along with its liberties.
#[derive(Debug)]
pub struct Group {
    pub occupant: Occupant,
    pub stones: Vec<(u8, u8)>,
    pub liberties: HashSet<(u8, u8)>,
}