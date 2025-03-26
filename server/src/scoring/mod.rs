/*!
 * Scoring and board analysis for the Go game.
 *
 * This module contains functionality for analyzing the game board, including:
 * - Group detection and liberty counting
 * - Territory determination 
 * - Scoring calculations (both area and territory scoring methods)
 * - Dead stone removal
 */

pub mod analysis;
pub mod groups;
pub mod territory;

pub use analysis::*;
pub use groups::*;
pub use territory::*;