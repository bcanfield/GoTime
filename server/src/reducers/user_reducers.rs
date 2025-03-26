use crate::models::game::user;
use crate::models::User;
use spacetimedb::{reducer, ReducerContext, Table};

/// Handles a client connection by creating or updating a user record.
///
/// This reducer is automatically called when a client connects to SpacetimeDB.
/// It ensures that every connected client has a corresponding User record.
///
/// # Arguments
/// * `ctx` - The reducer context containing the sender's identity
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
            log::info!("User {} reconnected", identity);
        }
        None => {
            // Create a new user record
            ctx.db.user().insert(User {
                identity,
                name: None,
                online: true,
            });
            log::info!("New user connected: {}", identity);
        }
    }
}

/// Marks a user as offline when they disconnect.
///
/// This reducer is automatically called when a client disconnects from SpacetimeDB.
///
/// # Arguments
/// * `ctx` - The reducer context containing the sender's identity
#[reducer]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
        log::info!("User {} disconnected", ctx.sender);
    }
}

/// Sets a display name for the user.
///
/// # Arguments
/// * `ctx` - The reducer context containing the sender's identity
/// * `name` - The display name to set for the user
///
/// # Returns
/// * `Ok(())` - Name was set successfully
/// * `Err(String)` - Error message if setting the name failed
#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    // Validate name
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    if name.len() > 30 {
        return Err("Name cannot exceed 30 characters".to_string());
    }

    // Update user record with the new name
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        let old_name = user.name.clone().unwrap_or_else(|| "unnamed".to_string());

        ctx.db.user().identity().update(User {
            name: Some(name.clone()),
            ..user
        });

        log::info!(
            "User {} changed name from '{}' to '{}'",
            ctx.sender,
            old_name,
            name
        );
        Ok(())
    } else {
        Err("User not found - please reconnect".to_string())
    }
}
