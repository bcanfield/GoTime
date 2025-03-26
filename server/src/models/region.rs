use super::spot::Occupant;
use std::collections::HashSet;

/// Represents a contiguous region of empty spaces on the Go board.
///
/// Empty regions are used for territory analysis and scoring calculations.
/// An empty region may represent neutral space ("dame") or territory
/// belonging to one player depending on its border properties.
#[derive(Debug, Clone)]
pub struct EmptyRegion {
    /// Coordinates of all empty spots in this region as (row, column) pairs
    pub spots: Vec<(u8, u8)>,

    /// Set of stone colors that border this empty region
    /// If this set contains only one color, the region is territory for that player
    pub border: HashSet<Occupant>,

    /// Whether this region touches the edge of the board
    /// In traditional scoring, regions touching the edge are not counted as territory
    pub touches_edge: bool,
}
