use crate::models::{Board, Game, ScoringMethod};
use super::territory::determine_territory;

/// Perform scoring analysis on a game.
/// This function deserializes the board, runs in-place annotation,
/// calculates the current score, and then updates the game accordingly.
pub fn analyze_game(mut game: Game) -> Game {
    // Deserialize the board from JSON into a Vec<SpotState>
    let board_vec =
        serde_json::from_str(&game.board).expect("Failed to deserialize board");
    let mut board_obj = Board::new(board_vec, game.board_size);
    // Run our in-place scoring annotation.
    board_obj.annotate_for_scoring();
    // Determine playability based on whose turn is next.
    let current_turn = match game.turn.as_str() {
        "B" => crate::models::Occupant::Black,
        "W" => crate::models::Occupant::White,
        _ => panic!("Invalid turn value"),
    };
    board_obj.annotate_playability(current_turn);
    let (black_score, white_score) = calculate_score(&board_obj, ScoringMethod::Area, 6.5);
    game.final_score_black = Some(black_score);
    game.final_score_white = Some(white_score);
    // Re-serialize the annotated board so the client receives insights.
    game.board = serde_json::to_string(&board_obj.spots).expect("Failed to serialize board");

    game
}

/// Calculate scores for Black and White.
/// For Area scoring, score = (number of stones) + (empty intersections in territory).
/// For Territory scoring, we use just the territory (plus komi to White).
/// (Komi is added as a float bonus to White.)
pub fn calculate_score(board: &Board, method: ScoringMethod, komi: f32) -> (f32, f32) {
    // Count stones on board.
    let mut black_stones = 0;
    let mut white_stones = 0;
    for spot in &board.spots {
        match spot.occupant {
            crate::models::Occupant::Black => black_stones += 1,
            crate::models::Occupant::White => white_stones += 1,
            _ => {}
        }
    }
    // Evaluate empty territory.
    let (black_territory, white_territory) = determine_territory(board);
    match method {
        ScoringMethod::Area => {
            let black_score = black_stones as f32 + black_territory as f32;
            let white_score = white_stones as f32 + white_territory as f32 + komi;
            (black_score, white_score)
        }
        ScoringMethod::Territory => {
            let black_score = black_territory as f32;
            let white_score = white_territory as f32 + komi;
            (black_score, white_score)
        }
    }
}