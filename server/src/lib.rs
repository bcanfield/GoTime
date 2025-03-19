use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

const DEFAULT_BOARD_SIZE: u8 = 9;

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[table(name = message, public)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

/// Represents a Go game.
/// The board is stored as a string of length board_size * board_size,
/// where '0' = empty, 'B' = black stone, and 'W' = white stone.
#[table(name = game, public)]
pub struct Game {
    #[primary_key]
    id: u64,
    player_black: Identity,
    player_white: Option<Identity>,
    board: String,
    turn: String,   // "B" for Black or "W" for White
    passes: u8,     // number of consecutive passes (could be used to end the game)
    board_size: u8, // e.g. 9 for a 9x9 board
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

// Called when a client connects to the SpacetimeDB
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        // If this is a returning user, i.e. we already have a `User` with this `Identity`,
        // set `online: true`, but leave `name` and `identity` unchanged.
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        // If this is a new user, create a `User` row for the `Identity`,
        // which is online, but hasn't set a name.
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
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

/// Helper function to create an empty board.
fn init_board(size: u8) -> String {
    std::iter::repeat('0')
        .take((size as usize) * (size as usize))
        .collect()
}

/// Reducer to create a new game.
/// The caller becomes the Black player.
#[reducer]
pub fn create_game(ctx: &ReducerContext, board_size: Option<u8>) -> Result<(), String> {
    // Use the provided board size or default.
    let size = board_size.unwrap_or(DEFAULT_BOARD_SIZE);
    // Use ctx.timestamp cast as u64 as game ID.
    let game_id = ctx.timestamp.to_micros_since_unix_epoch() as u64;
    let board = init_board(size);
    ctx.db.game().insert(Game {
        id: game_id,
        player_black: ctx.sender,
        player_white: None,
        board,
        turn: "B".to_string(),
        passes: 0,
        board_size: size,
    });
    log::info!("Created game with id: {}", game_id);
    Ok(())
}

/// Reducer for a second player to join an existing game.
#[reducer]
pub fn join_game(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    // Use the .id() accessor to work with the primary key.
    if let Some(game) = ctx.db.game().id().find(game_id) {
        if game.player_white.is_some() {
            return Err("Game already has two players".to_string());
        }
        // The joining player becomes White.
        ctx.db.game().id().update(Game {
            player_white: Some(ctx.sender),
            ..game
        });
        Ok(())
    } else {
        Err("Game not found".to_string())
    }
}

/// Reducer to place a stone on the board.
/// x and y are zero-indexed coordinates.
#[reducer]
pub fn place_stone(ctx: &ReducerContext, game_id: u64, x: u8, y: u8) -> Result<(), String> {
    if let Some(game) = ctx.db.game().id().find(game_id) {
        // Ensure game is full (has two players).
        if game.player_white.is_none() {
            return Err("Waiting for second player".to_string());
        }
        // Determine the stone color for the current user.
        let stone = if ctx.sender == game.player_black {
            "B".to_string()
        } else if Some(ctx.sender) == game.player_white {
            "W".to_string()
        } else {
            return Err("You are not a player in this game".to_string());
        };

        // Check if it's the correct turn.
        if stone != game.turn {
            return Err("Not your turn".to_string());
        }

        let size = game.board_size;
        let board_len = (size as usize) * (size as usize);
        let idx = (y as usize) * (size as usize) + (x as usize);
        if idx >= board_len {
            return Err("Position out of bounds".to_string());
        }

        // Convert board string to a vector of chars for easier manipulation.
        let board_chars: Vec<char> = game.board.chars().collect();
        if board_chars[idx] != '0' {
            return Err("Position already occupied".to_string());
        }

        let mut new_board = board_chars.clone();
        // Place the stone.
        let stone_char = stone.chars().next().unwrap();
        new_board[idx] = stone_char;

        // For each adjacent position containing an opponent stone,
        // check if its group has any liberties.
        let opponent = if stone == "B" { 'W' } else { 'B' };
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= size as i32 || ny >= size as i32 {
                continue;
            }
            let nidx = (ny as usize) * (size as usize) + (nx as usize);
            if new_board[nidx] == opponent {
                // If the opponent group has no liberties, capture it.
                if !has_liberty(&new_board, size, nx as usize, ny as usize, opponent) {
                    remove_group(&mut new_board, size, nx as usize, ny as usize, opponent);
                }
            }
        }

        // Check for suicide: the placed stone's group must have at least one liberty.
        if !has_liberty(&new_board, size, x as usize, y as usize, stone_char) {
            return Err("Illegal move: suicide".to_string());
        }

        // Convert board back to string.
        let updated_board: String = new_board.into_iter().collect();
        // Switch turn.
        let next_turn = if game.turn == "B" { "W" } else { "B" };

        ctx.db.game().id().update(Game {
            board: updated_board,
            turn: next_turn.to_string(),
            passes: 0, // reset passes on stone placement.
            ..game
        });
        Ok(())
    } else {
        Err("Game not found".to_string())
    }
}

/// Helper: Check if the group connected to (x, y) of color `stone` has any liberties.
fn has_liberty(board: &Vec<char>, size: u8, x: usize, y: usize, stone: char) -> bool {
    let board_len = board.len();
    let mut visited = vec![false; board_len];
    let mut stack = vec![(x, y)];
    while let Some((cx, cy)) = stack.pop() {
        let idx = cy * (size as usize) + cx;
        if visited[idx] {
            continue;
        }
        visited[idx] = true;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;
            if nx < 0 || ny < 0 || nx >= size as i32 || ny >= size as i32 {
                continue;
            }
            let nidx = (ny as usize) * (size as usize) + (nx as usize);
            if board[nidx] == '0' {
                return true;
            }
            if board[nidx] == stone && !visited[nidx] {
                stack.push((nx as usize, ny as usize));
            }
        }
    }
    false
}

/// Helper: Remove the group of stones connected to (x, y) of color `stone`.
fn remove_group(board: &mut Vec<char>, size: u8, x: usize, y: usize, stone: char) {
    let board_len = board.len();
    let mut visited = vec![false; board_len];
    let mut stack = vec![(x, y)];
    while let Some((cx, cy)) = stack.pop() {
        let idx = cy * (size as usize) + cx;
        if visited[idx] {
            continue;
        }
        visited[idx] = true;
        if board[idx] == stone {
            board[idx] = '0'; // Remove the stone.
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions.iter() {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx < 0 || ny < 0 || nx >= size as i32 || ny >= size as i32 {
                    continue;
                }
                stack.push((nx as usize, ny as usize));
            }
        }
    }
}
