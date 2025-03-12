use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

const FIELD_WIDTH: f32 = 800.0;
const FIELD_HEIGHT: f32 = 600.0;
const PLAYER_RADIUS: f32 = 20.0;

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
    pos_x: f32,
    pos_y: f32,
}

#[table(name = message, public)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[reducer]
/// Clients invoke this reducer to set their user names.
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            name: Some(name),
            ..user
        });
        Ok(())
    } else {
        Err("Cannot set name for unknown user".to_string())
    }
}

/// Takes a name and checks if it's acceptable as a user's name.
fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[reducer]
/// Clients invoke this reducer to send messages.
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    ctx.db.message().insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

/// Takes a message's text and checks if it's acceptable to send.
fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // Reconnecting user: reset position and mark online.
        ctx.db.user().identity().update(User {
            online: true,
            pos_x: FIELD_WIDTH / 2.0,
            pos_y: FIELD_HEIGHT / 2.0,
            ..user
        });
    } else {
        // New user: insert with default center position.
        ctx.db.user().insert(User {
            identity: ctx.sender,
            name: None,
            online: true,
            pos_x: FIELD_WIDTH / 2.0,
            pos_y: FIELD_HEIGHT / 2.0,
        });
    }
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: false,
            ..user
        });
    } else {
        log::warn!(
            "Disconnect event for unknown user with identity {:?}",
            ctx.sender
        );
    }
}

#[reducer]
pub fn move_user(ctx: &ReducerContext, delta_x: f32, delta_y: f32) -> Result<(), String> {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        let mut new_x = user.pos_x + delta_x;
        let mut new_y = user.pos_y + delta_y;

        // Clamp to boundaries.
        if new_x < PLAYER_RADIUS {
            new_x = PLAYER_RADIUS;
        }
        if new_y < PLAYER_RADIUS {
            new_y = PLAYER_RADIUS;
        }
        if new_x > FIELD_WIDTH - PLAYER_RADIUS {
            new_x = FIELD_WIDTH - PLAYER_RADIUS;
        }
        if new_y > FIELD_HEIGHT - PLAYER_RADIUS {
            new_y = FIELD_HEIGHT - PLAYER_RADIUS;
        }

        // Use iter() to loop over all users for collision detection.
        for other in ctx.db.user().iter() {
            if other.identity != ctx.sender {
                let dx = new_x - other.pos_x;
                let dy = new_y - other.pos_y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < PLAYER_RADIUS * 2.0 && distance > 0.0 {
                    // Calculate the overlap and push the current user away slightly.
                    let overlap = PLAYER_RADIUS * 2.0 - distance;
                    new_x += (dx / distance) * overlap * 0.5;
                    new_y += (dy / distance) * overlap * 0.5;

                    // Re-clamp in case the push moves outside boundaries.
                    if new_x < PLAYER_RADIUS {
                        new_x = PLAYER_RADIUS;
                    }
                    if new_y < PLAYER_RADIUS {
                        new_y = PLAYER_RADIUS;
                    }
                    if new_x > FIELD_WIDTH - PLAYER_RADIUS {
                        new_x = FIELD_WIDTH - PLAYER_RADIUS;
                    }
                    if new_y > FIELD_HEIGHT - PLAYER_RADIUS {
                        new_y = FIELD_HEIGHT - PLAYER_RADIUS;
                    }
                }
            }
        }

        // Update the user's position.
        ctx.db.user().identity().update(User {
            pos_x: new_x,
            pos_y: new_y,
            ..user
        });
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}
