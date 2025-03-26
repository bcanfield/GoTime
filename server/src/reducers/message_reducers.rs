use crate::models::game::message;
use crate::models::Message;
use spacetimedb::{reducer, ReducerContext, Table};

/// Validates a message to ensure it meets content requirements.
///
/// # Arguments
/// * `text` - The message text to validate
///
/// # Returns
/// * `Ok(String)` - The validated message text
/// * `Err(String)` - Error message if validation failed
fn validate_message(text: String) -> Result<String, String> {
    // Check for empty messages
    if text.trim().is_empty() {
        return Err("Messages must not be empty".to_string());
    }

    // Limit message length to prevent spam
    if text.len() > 1000 {
        return Err("Message is too long (maximum 1000 characters)".to_string());
    }

    Ok(text)
}

/// Sends a chat message in the game.
///
/// This reducer stores the message in the database with the sender's identity
/// and a timestamp, making it visible to all connected clients.
///
/// # Arguments
/// * `ctx` - The reducer context containing sender identity and timestamp
/// * `text` - The message content to send
///
/// # Returns
/// * `Ok(())` - Message was sent successfully
/// * `Err(String)` - Error message if sending failed
#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    // Validate the message content
    let text = validate_message(text)?;

    // Insert the message into the database
    ctx.db.message().insert(Message {
        sender: ctx.sender,
        text: text.clone(),
        sent: ctx.timestamp,
    });

    log::debug!("Message from {}: {}", ctx.sender, text);
    Ok(())
}
