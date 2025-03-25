use crate::models::game;
use crate::models::{Game, Occupant, SpotState};
use crate::scoring::analyze_game;
use serde_json;
use spacetimedb::{Identity, Table};

const DEFAULT_BOARD_SIZE: u8 = 9;

fn generate_empty_board(size: u8) -> Vec<SpotState> {
    (0..(size as usize * size as usize))
        .map(|_| SpotState {
            occupant: Occupant::Empty,
            move_number: None,
            marker: None,
            scoring_owner: None,
            scoring_explanation: None,
        })
        .collect()
}

/// Seed a game with a given static layout (board), and a specified turn and status.
fn create_seed_game(
    id: u64,
    player_black: Identity,
    player_white: Option<Identity>,
    board: Vec<SpotState>,
    turn: &str,
    board_size: u8,
    game_over: bool,
    final_score_black: Option<f32>,
    final_score_white: Option<f32>,
) -> Game {
    let board_json = serde_json::to_string(&board).unwrap();
    Game {
        id,
        player_black,
        player_white,
        board: board_json,
        turn: turn.to_string(),
        passes: 0,
        board_size,
        previous_board: None,
        game_over,
        final_score_black,
        final_score_white,
    }
}

/// This would be your main seeding entry point.
pub fn seed_sample_games(ctx: &spacetimedb::ReducerContext) {
    let identity1 = Identity::__dummy();
    let identity2 = Identity::__dummy();
    let timestamp = ctx.timestamp;
    let base_id: u64 = timestamp.to_micros_since_unix_epoch().try_into().unwrap();

    // 1. Game with an empty board
    let mut game1 = create_seed_game(
        base_id,
        identity1,
        Some(identity2),
        generate_empty_board(DEFAULT_BOARD_SIZE),
        "B",
        DEFAULT_BOARD_SIZE,
        false,
        None,
        None,
    );
    game1 = analyze_game(game1);
    ctx.db.game().insert(game1);

    // 2. Game with a few stones played
    let mut board2 = generate_empty_board(DEFAULT_BOARD_SIZE);
    let mut place = |x: usize, y: usize, color: Occupant, move_num: u64| {
        let idx = y * DEFAULT_BOARD_SIZE as usize + x;
        board2[idx].occupant = color;
        board2[idx].move_number = Some(move_num);
    };
    place(2, 2, Occupant::Black, base_id + 1);
    place(3, 2, Occupant::White, base_id + 2);
    place(2, 3, Occupant::Black, base_id + 3);

    let mut game2 = create_seed_game(
        base_id + 1,
        identity1,
        Some(identity2),
        board2,
        "W",
        DEFAULT_BOARD_SIZE,
        false,
        None,
        None,
    );
    game2 = analyze_game(game2);
    ctx.db.game().insert(game2);

    // 3. Finished game with a score
    let mut board3 = generate_empty_board(DEFAULT_BOARD_SIZE);
    for y in 0..DEFAULT_BOARD_SIZE {
        for x in 0..DEFAULT_BOARD_SIZE {
            let idx = y as usize * DEFAULT_BOARD_SIZE as usize + x as usize;
            board3[idx].occupant = if (x + y) % 2 == 0 {
                Occupant::Black
            } else {
                Occupant::White
            };
        }
    }

    let mut game3 = create_seed_game(
        base_id + 2,
        identity1,
        Some(identity2),
        board3,
        "B",
        DEFAULT_BOARD_SIZE,
        true,
        None,
        None,
    );

    game3 = analyze_game(game3);
    ctx.db.game().insert(game3);

    // Define the layout for a complex game board.
    let layout = [
        "BBBBBBBBB",
        "B...W...B",
        "B.B.W.W.B",
        "B...W...B",
        "BBBBWBBBB",
        "W...B...W",
        "W.W.B.B.W",
        "W...B...W",
        "WWWWBWWWW",
    ];

    // Create the board from layout, starting move numbers from base_id + 100.
    let board = board_from_layout(&layout, base_id + 100, DEFAULT_BOARD_SIZE);

    // Create a new game with this board.
    let mut game4 = create_seed_game(
        base_id + 3,
        identity1,
        Some(identity2),
        board,
        "B", // Next turn set arbitrarily.
        DEFAULT_BOARD_SIZE,
        true,
        None,
        None,
    );

    game4 = analyze_game(game4);
    ctx.db.game().insert(game4);

    let layout_19x19 = [
        "...................", // row 0
        ".BBBBBBB...........", // row 1
        ".B.....B...........", // row 2
        ".B.....B...........", // row 3
        ".B.....B...........", // row 4
        ".BBBBBBB...........", // row 5
        "...................", // row 6
        "...................", // row 7
        "...................", // row 8
        "...................", // row 9
        "...................", // row10
        "...................", // row11
        "...........WWWWWWW.", // row12
        "...........W.....W.", // row13
        "...........W.....W.", // row14
        "...........W.....W.", // row15
        "...........WWWWWWW.", // row16
        "...................", // row17
        "...................", // row18
    ];

    let board_size = 19;
    let board = board_from_layout(&layout_19x19, base_id + 200, board_size);

    let mut game5 = create_seed_game(
        base_id + 4,
        identity1,
        Some(identity2),
        board,
        "B", // Arbitrary turn
        board_size,
        true, // Game is finished
        None,
        None,
    );

    // Annotate and score
    game5 = analyze_game(game5);

    // Insert into DB
    ctx.db.game().insert(game5);
}

/// Given a layout (each string represents a row) where:
/// 'B' => Black, 'W' => White, '.' => Empty,
/// this function returns a board (Vec<SpotState>) with move numbers assigned starting at `base_move`.
pub fn board_from_layout(layout: &[&str], base_move: u64, board_size: u8) -> Vec<SpotState> {
    // Start with an empty board.
    let mut board = generate_empty_board(board_size);
    let mut move_num = base_move;

    for (y, row) in layout.iter().enumerate() {
        // Trim row in case there are leading/trailing spaces.
        for (x, ch) in row.trim().chars().enumerate() {
            let idx = y * board_size as usize + x;
            match ch {
                'B' => {
                    board[idx].occupant = crate::models::Occupant::Black;
                    board[idx].move_number = Some(move_num);
                    move_num += 1;
                }
                'W' => {
                    board[idx].occupant = crate::models::Occupant::White;
                    board[idx].move_number = Some(move_num);
                    move_num += 1;
                }
                '.' => {
                    // Leave spot as Empty.
                }
                _ => panic!("Unknown board character: {}", ch),
            }
        }
    }
    board
}
