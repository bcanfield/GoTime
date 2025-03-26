/*!
 * Go Game Backend
 *
 * A Rust implementation of the board game Go (also known as Baduk or Weiqi)
 * built on SpacetimeDB.
 *
 * This crate provides:
 * - Game rules enforcement
 * - Board state tracking and analysis
 * - Scoring and territory calculation
 * - User and game management
 */

pub mod models;
pub mod reducers;
pub mod scoring;
pub mod seed;
pub mod utils;

#[cfg(test)]
pub mod tests;

// Re-export the public items from each module
pub use models::*;
pub use reducers::*;
pub use scoring::*;
pub use seed::*;
pub use utils::*;

// Export test modules for testing
#[cfg(test)]
pub use tests::*;
