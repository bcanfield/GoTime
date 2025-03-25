use crate::models::{game, message, user};
use crate::models::{Game, Message, Occupant, ScoringMethod, SpotState, User};
use crate::scoring::{analyze_game, calculate_score};
use crate::seed::seed_sample_games;

use crate::utils::{apply_move_to_board, coord_to_index};
use serde_json;
use spacetimedb::{reducer, ReducerContext, Table};
use std::convert::TryInto;

const DEFAULT_BOARD_SIZE: u8 = 9;

fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer]
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

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empty".to_string())
    } else {
        Ok(name)
    }
}

#[reducer]
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

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    log::info!("ADASDASD existing user {:?}", ctx.sender);

    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        log::info!("Connected existing user {:?}", ctx.sender);

        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        log::info!("Created user {:?}", ctx.sender);

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

#[reducer]
pub fn create_game(
    ctx: &ReducerContext,
    board_size: Option<u8>,
    handicap: Option<u8>,
) -> Result<(), String> {
    let size = board_size.unwrap_or(DEFAULT_BOARD_SIZE);
    let game_id: u64 = ctx
        .timestamp
        .to_micros_since_unix_epoch()
        .try_into()
        .unwrap();
    let mut board: Vec<SpotState> = (0..(size as usize * size as usize))
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
            playable: true,
        })
        .collect();
    let handicap = handicap.unwrap_or(0);
    if handicap > 0 {
        let handicap_positions = match size {
            9 => vec![(2, 2), (6, 6), (2, 6), (6, 2), (4, 4)],
            13 => vec![(3, 3), (9, 9), (3, 9), (9, 3), (6, 6)],
            19 => vec![(3, 3), (15, 15), (3, 15), (15, 3), (9, 9)],
            _ => vec![],
        };
        for i in 0..(handicap as usize).min(handicap_positions.len()) {
            let (x, y) = handicap_positions[i];
            let idx = coord_to_index(x, y, size as usize);
            board[idx].occupant = Occupant::Black;
            board[idx].move_number = Some(
                ctx.timestamp
                    .to_micros_since_unix_epoch()
                    .try_into()
                    .unwrap(),
            );
        }
    }
    let turn = if handicap > 0 {
        "W".to_string()
    } else {
        "B".to_string()
    };
    let board_json = serde_json::to_string(&board).unwrap();
    ctx.db.game().insert(Game {
        id: game_id,
        player_black: ctx.sender,
        player_white: None,
        board: board_json,
        turn,
        passes: 0,
        board_size: size,
        previous_board: None,
        game_over: false,
        final_score_black: None,
        final_score_white: None,
    });
    log::info!("Created game with id: {}", game_id);
    Ok(())
}

#[reducer]
pub fn join_game(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    if let Some(game) = ctx.db.game().id().find(game_id) {
        if game.player_white.is_some() {
            return Err("Game already has two players".to_string());
        }
        ctx.db.game().id().update(Game {
            player_white: Some(ctx.sender),
            ..game
        });
        Ok(())
    } else {
        Err("Game not found".to_string())
    }
}

#[reducer]
pub fn pass_move(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err("Game not found".to_string()),
    };
    if game.game_over {
        return Err("Game is already over".to_string());
    }
    game.passes += 1;
    if game.passes >= 2 {
        let game_board = game.as_board().unwrap();
        let (black_score, white_score) = calculate_score(&game_board, ScoringMethod::Area, 6.5);
        game.final_score_black = Some(black_score);
        game.final_score_white = Some(white_score);
        game.game_over = true;
    } else {
        game.turn = if game.turn == "B" {
            "W".to_string()
        } else {
            "B".to_string()
        };
    }
    ctx.db.game().id().update(game);
    Ok(())
}

#[reducer]
pub fn place_stone(ctx: &ReducerContext, game_id: u64, x: u8, y: u8) -> Result<(), String> {
    let mut game = match ctx.db.game().id().find(game_id) {
        Some(g) => g,
        None => return Err("Game not found".to_string()),
    };
    if game.game_over {
        return Err("Game is over".to_string());
    }
    if game.player_white.is_none() {
        return Err("Waiting for second player".to_string());
    }
    let stone_color = if ctx.sender == game.player_black {
        Occupant::Black
    } else if Some(ctx.sender) == game.player_white {
        Occupant::White
    } else {
        return Err("You are not a player in this game".to_string());
    };
    let expected_turn = match stone_color {
        Occupant::Black => "B",
        Occupant::White => "W",
        _ => unreachable!(),
    };
    if game.turn != expected_turn {
        return Err("Not your turn".to_string());
    }
    let size = game.board_size as usize;
    let board: Vec<SpotState> = serde_json::from_str(&game.board).map_err(|e| e.to_string())?;
    let idx = coord_to_index(x as usize, y as usize, size);
    if board[idx].occupant != Occupant::Empty {
        return Err("Position already occupied".to_string());
    }
    let move_num: u64 = ctx
        .timestamp
        .to_micros_since_unix_epoch()
        .try_into()
        .unwrap();
    let (_new_board, new_board_str) = apply_move_to_board(
        board,
        size,
        stone_color,
        x as usize,
        y as usize,
        game.previous_board.clone(),
        move_num,
    )?;
    game.previous_board = Some(game.board.clone());
    game.board = new_board_str.unwrap();
    game.passes = 0;
    game.turn = if game.turn == "B" {
        "W".to_string()
    } else {
        "B".to_string()
    };

    game = analyze_game(game);

    ctx.db.game().id().update(game);
    Ok(())
}

#[reducer]
pub fn seed(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Seeding sample games");
    seed_sample_games(ctx);
    log::info!("Seeding completed");
    Ok(())
}
