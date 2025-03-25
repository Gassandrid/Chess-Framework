use crate::chess::board::Position;
use crate::chess::piece::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalMovesRequest {
    pub position: [[Option<char>; 8]; 8],
    pub square: String,
    pub current_player: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalMovesResponse {
    pub moves: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeMoveRequest {
    pub position: [[Option<char>; 8]; 8],
    pub from: String,
    pub to: String,
    pub current_player: String,
    pub promotion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeMoveResponse {
    pub new_position: [[Option<char>; 8]; 8],
    pub game_status: String,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_draw: bool,
    pub move_notation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameResponse {
    pub position: [[Option<char>; 8]; 8],
    pub current_player: String,
    pub game_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndoMoveRequest {
    pub position: [[Option<char>; 8]; 8],
    pub move_history: Vec<MoveHistoryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndoMoveResponse {
    pub new_position: [[Option<char>; 8]; 8],
    pub new_move_history: Vec<MoveHistoryEntry>,
    pub current_player: String,
    pub game_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveHistoryEntry {
    pub from: String,
    pub to: String,
    pub piece: String,
    pub capture: Option<String>,
    pub promotion: Option<String>,
    pub check: Option<bool>,
    pub checkmate: Option<bool>,
    pub notation: String,
}
