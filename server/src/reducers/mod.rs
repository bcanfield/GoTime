/*!
 * SpacetimeDB reducers for the Go game.
 *
 * Reducers are functions that modify the database state and implement
 * the game's business logic. Each reducer handles a specific action
 * like creating a game, placing a stone, or managing user connections.
 */

pub mod game_reducers;
pub mod message_reducers;
pub mod user_reducers;

pub use game_reducers::*;
pub use message_reducers::*;
pub use user_reducers::*;