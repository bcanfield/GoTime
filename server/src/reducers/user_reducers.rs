use crate::models::game::user;
use crate::models::User;
use spacetimedb::{reducer, ReducerContext, Table};

/// This reducer ensures the user is in the database when they connect.
/// If the user already exists, update to reflect they are online.
#[reducer]
pub fn client_connected(ctx: &ReducerContext) {
    let identity = ctx.sender;
    match ctx.db.user().identity().find(identity) {
        Some(user) => {
            // Update existing user to mark them as online
            ctx.db.user().identity().update(User {
                online: true,
                ..user
            });
        }
        None => {
            // Create a new user record
            ctx.db.user().insert(User {
                identity,
                name: None,
                online: true,
            });
        }
    }
}

#[reducer]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    }
}

#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            name: Some(name),
            ..user
        });
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}
