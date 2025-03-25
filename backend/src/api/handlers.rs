use super::models::{
    LegalMovesRequest, LegalMovesResponse, MakeMoveRequest, MakeMoveResponse, MoveHistoryEntry,
    NewGameResponse, UndoMoveRequest, UndoMoveResponse,
};
use crate::chess::{
    board::{Board, Position},
    game::{Game, GameStatus},
    moves::{generate_legal_moves, Move},
    piece::{Color, Piece, PieceType},
};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use std::sync::{Arc, Mutex};

// Parse color from string
fn parse_color(color: &str) -> Color {
    match color.to_lowercase().as_str() {
        "white" => Color::White,
        "black" => Color::Black,
        _ => Color::White, // Default
    }
}

// Convert game status to string
fn game_status_to_string(status: &GameStatus) -> String {
    match status {
        GameStatus::Active => "active".to_string(),
        GameStatus::Check => "check".to_string(),
        GameStatus::Checkmate => "checkmate".to_string(),
        GameStatus::Stalemate => "stalemate".to_string(),
        GameStatus::Draw => "draw".to_string(),
    }
}

// Handler for legal moves
pub async fn legal_moves(
    Extension(game): Extension<Arc<Mutex<Game>>>,
    Json(request): Json<LegalMovesRequest>,
) -> Result<Json<LegalMovesResponse>, StatusCode> {
    // Log the request to help with debugging
    println!(
        "Legal moves request for square: {}, player: {}",
        request.square, request.current_player
    );

    // Parse the request
    let board = Board::from_array(request.position);
    let position = Position::from_algebraic(&request.square).ok_or(StatusCode::BAD_REQUEST)?;
    let color = parse_color(&request.current_player);

    // Generate legal moves
    let moves = generate_legal_moves(&board, position, color);

    // Convert to algebraic notation
    let moves = moves
        .into_iter()
        .map(|pos| pos.to_algebraic())
        .collect::<Vec<String>>();

    // Log the response for debugging
    println!("Legal moves response: {:?}", moves);

    Ok(Json(LegalMovesResponse { moves }))
}

// Handler for making a move
pub async fn make_move(
    Extension(game): Extension<Arc<Mutex<Game>>>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<MakeMoveResponse>, StatusCode> {
    let mut game_lock = game.lock().unwrap();

    // Parse the request
    let from = Position::from_algebraic(&request.from).ok_or(StatusCode::BAD_REQUEST)?;
    let to = Position::from_algebraic(&request.to).ok_or(StatusCode::BAD_REQUEST)?;

    // Create the move
    let mut chess_move = Move::new(from, to);

    // Handle promotion if specified
    if let Some(promotion) = request.promotion {
        let promotion_type = match promotion.to_lowercase().as_str() {
            "q" => PieceType::Queen,
            "r" => PieceType::Rook,
            "b" => PieceType::Bishop,
            "n" => PieceType::Knight,
            _ => return Err(StatusCode::BAD_REQUEST),
        };
        chess_move.promotion = Some(promotion_type);
    }

    // Make the move
    let result = game_lock
        .make_move(&chess_move)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Prepare the response
    let response = MakeMoveResponse {
        new_position: game_lock.board.to_array(),
        game_status: game_status_to_string(&game_lock.status),
        is_check: game_lock.status == GameStatus::Check,
        is_checkmate: game_lock.status == GameStatus::Checkmate,
        is_draw: game_lock.status == GameStatus::Draw || game_lock.status == GameStatus::Stalemate,
        move_notation: chess_move.to_algebraic(),
    };

    Ok(Json(response))
}

// Handler for starting a new game
pub async fn new_game(Extension(game): Extension<Arc<Mutex<Game>>>) -> Json<NewGameResponse> {
    let mut game_lock = game.lock().unwrap();

    // Reset the game
    *game_lock = Game::new();

    // Prepare the response
    let response = NewGameResponse {
        position: game_lock.board.to_array(),
        current_player: "white".to_string(),
        game_status: "active".to_string(),
    };

    Json(response)
}

// Handler for undoing a move
pub async fn undo_move(
    Extension(game): Extension<Arc<Mutex<Game>>>,
    Json(request): Json<UndoMoveRequest>,
) -> Result<Json<UndoMoveResponse>, StatusCode> {
    let mut game_lock = game.lock().unwrap();

    // Undo the last move
    game_lock.undo_move().map_err(|_| StatusCode::BAD_REQUEST)?;

    // Convert move history to the expected format
    let new_move_history: Vec<MoveHistoryEntry> = game_lock
        .move_history
        .iter()
        .map(|(m, _)| MoveHistoryEntry {
            from: m.from.to_algebraic(),
            to: m.to.to_algebraic(),
            piece: game_lock
                .board
                .get_piece(m.from)
                .map(|p| p.to_fen_char().to_string())
                .unwrap_or_default(),
            capture: None, // In a real implementation, we would track captures
            promotion: m.promotion.map(|_| "q".to_string()),
            check: None,
            checkmate: None,
            notation: m.to_algebraic(),
        })
        .collect();

    // Prepare the response
    let response = UndoMoveResponse {
        new_position: game_lock.board.to_array(),
        new_move_history,
        current_player: if game_lock.current_player == Color::White {
            "white"
        } else {
            "black"
        }
        .to_string(),
        game_status: game_status_to_string(&game_lock.status),
    };

    Ok(Json(response))
}
