/*!
 * Core data models for the Go game.
 *
 * This module contains all the fundamental types and structures used
 * to represent game state, boards, stones, and players.
 */

pub mod board;
pub mod game;
pub mod group;
pub mod region;
pub mod spot;

pub use board::*;
pub use game::*;
pub use group::*;
pub use region::*;
pub use spot::*;
