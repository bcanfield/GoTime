use crate::models::{Board, Game, Occupant, ScoringMethod};
use super::territory::determine_territory;

/// Performs a comprehensive scoring analysis on a game.
///
/// This function takes a Game object, deserializes its board data, runs territory and
/// playability analysis, calculates the current score, and then updates the game object
/// with the results.
///
/// # Arguments
/// * `game` - The Game object to analyze
///
/// # Returns
/// The updated Game object with scoring information
pub fn analyze_game(mut game: Game) -> Game {
    // Deserialize the board from JSON into a Board object
    let board_result = game.as_board();
    
    if let Ok(mut board_obj) = board_result {
        // Run in-place scoring annotation
        board_obj.annotate_for_scoring();
        
        // Determine which spots are legal moves based on whose turn is next
        let current_turn = match game.turn.as_str() {
            "B" => Occupant::Black,
            "W" => Occupant::White,
            _ => panic!("Invalid turn value: {}", game.turn),
        };
        board_obj.annotate_playability(current_turn);
        
        // Calculate current score using Chinese rules (area scoring) with standard 6.5 komi
        let (black_score, white_score) = calculate_score(&board_obj, ScoringMethod::Area, 6.5);
        game.final_score_black = Some(black_score);
        game.final_score_white = Some(white_score);
        
        // Re-serialize the annotated board so the client receives the analysis
        game.board = serde_json::to_string(&board_obj.spots)
            .expect("Failed to serialize board");
    } else {
        log::error!("Failed to deserialize board for game {}", game.id);
    }
    
    game
}

/// Calculates the score for both players using the specified scoring method.
///
/// Go has two main scoring systems:
/// - Area scoring (Chinese rules): score = stones on board + surrounded territory
/// - Territory scoring (Japanese rules): score = surrounded territory only
///
/// # Arguments
/// * `board` - The game board to analyze
/// * `method` - Which scoring method to use (Area or Territory)
/// * `komi` - Compensation points given to White (typically 6.5 to prevent draws)
///
/// # Returns
/// A tuple (black_score, white_score) with the final scores
pub fn calculate_score(board: &Board, method: ScoringMethod, komi: f32) -> (f32, f32) {
    // Count stones on board
    let mut black_stones = 0;
    let mut white_stones = 0;
    
    for spot in &board.spots {
        match spot.occupant {
            Occupant::Black => black_stones += 1,
            Occupant::White => white_stones += 1,
            Occupant::Empty => {} // Skip empty spots
        }
    }
    
    // Calculate territory (empty intersections surrounded by a single color)
    let (black_territory, white_territory) = determine_territory(board);
    
    match method {
        ScoringMethod::Area => {
            // Chinese rules: stones + territory
            let black_score = black_stones as f32 + black_territory as f32;
            let white_score = white_stones as f32 + white_territory as f32 + komi;
            (black_score, white_score)
        }
        ScoringMethod::Territory => {
            // Japanese rules: territory only (plus captures, which aren't tracked here)
            let black_score = black_territory as f32;
            let white_score = white_territory as f32 + komi;
            (black_score, white_score)
        }
    }
}