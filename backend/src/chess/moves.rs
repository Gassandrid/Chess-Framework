use crate::chess::board::{Board, Position};
use crate::chess::piece::{Color, Piece, PieceType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    pub fn with_promotion(from: Position, to: Position, promotion: PieceType) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }

    // Convert to algebraic notation
    pub fn to_algebraic(&self) -> String {
        let mut result = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());

        if let Some(promotion) = self.promotion {
            let promotion_char = match promotion {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => panic!("Invalid promotion piece"),
            };
            result.push('=');
            result.push(promotion_char);
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoveResult {
    Normal,
    Capture(Piece),
    Castle,
    EnPassant,
    Promotion,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}

// legal move generator: ported from cpp
pub fn generate_legal_moves(board: &Board, pos: Position, _color: Color) -> Vec<Position> {
    let piece = match board.get_piece(pos) {
        Some(p) => p,
        None => return Vec::new(),
    };

    match piece.piece_type {
        PieceType::Pawn => generate_pawn_moves(board, pos, piece.color),
        PieceType::Knight => generate_knight_moves(board, pos, piece.color),
        PieceType::Bishop => generate_bishop_moves(board, pos, piece.color),
        PieceType::Rook => generate_rook_moves(board, pos, piece.color),
        PieceType::Queen => {
            let mut moves = generate_bishop_moves(board, pos, piece.color);
            moves.extend(generate_rook_moves(board, pos, piece.color));
            moves
        }
        PieceType::King => generate_king_moves(board, pos, piece.color),
    }
}

// Helper function to check if a position is valid and empty or contains an enemy piece
fn is_valid_target(board: &Board, pos: Position, color: Color) -> bool {
    if pos.rank >= 8 || pos.file >= 8 {
        return false;
    }

    match board.get_piece(pos) {
        Some(piece) => piece.color != color,
        None => true,
    }
}

// pawn moves
fn generate_pawn_moves(board: &Board, pos: Position, color: Color) -> Vec<Position> {
    let mut moves = Vec::new();
    let direction = if color == Color::White {
        -1isize
    } else {
        1isize
    };
    let start_rank = if color == Color::White { 6 } else { 1 };

    // Forward move
    if let Some(new_rank) = (pos.rank as isize)
        .checked_add(direction)
        .map(|r| r as usize)
    {
        if new_rank < 8 {
            let forward = Position::new(pos.file, new_rank);
            if board.get_piece(forward).is_none() {
                moves.push(forward);

                // Double forward from starting position
                if pos.rank == start_rank {
                    if let Some(double_rank) = (new_rank as isize)
                        .checked_add(direction)
                        .map(|r| r as usize)
                    {
                        if double_rank < 8 {
                            let double_forward = Position::new(pos.file, double_rank);
                            if board.get_piece(double_forward).is_none() {
                                moves.push(double_forward);
                            }
                        }
                    }
                }
            }
        }
    }

    // Captures
    for file_offset in [-1, 1].iter() {
        if let (Some(new_file), Some(new_rank)) = (
            (pos.file as isize)
                .checked_add(*file_offset)
                .map(|f| f as usize),
            (pos.rank as isize)
                .checked_add(direction)
                .map(|r| r as usize),
        ) {
            if new_file < 8 && new_rank < 8 {
                let capture_pos = Position::new(new_file, new_rank);
                if let Some(piece) = board.get_piece(capture_pos) {
                    if piece.color != color {
                        moves.push(capture_pos);
                    }
                }
                // En passant would be checked here in a full implementation
            }
        }
    }

    moves
}

// Generate knight moves
fn generate_knight_moves(board: &Board, pos: Position, color: Color) -> Vec<Position> {
    let mut moves = Vec::new();
    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    for (file_offset, rank_offset) in offsets.iter() {
        if let (Some(new_file), Some(new_rank)) = (
            (pos.file as isize)
                .checked_add(*file_offset)
                .map(|f| f as usize),
            (pos.rank as isize)
                .checked_add(*rank_offset)
                .map(|r| r as usize),
        ) {
            if new_file < 8 && new_rank < 8 {
                let new_pos = Position::new(new_file, new_rank);
                if is_valid_target(board, new_pos, color) {
                    moves.push(new_pos);
                }
            }
        }
    }

    moves
}

// Generate bishop moves
fn generate_bishop_moves(board: &Board, pos: Position, color: Color) -> Vec<Position> {
    let mut moves = Vec::new();
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (file_dir, rank_dir) in directions.iter() {
        let mut distance = 1;
        loop {
            if let (Some(new_file), Some(new_rank)) = (
                (pos.file as isize)
                    .checked_add(file_dir * distance)
                    .map(|f| f as usize),
                (pos.rank as isize)
                    .checked_add(rank_dir * distance)
                    .map(|r| r as usize),
            ) {
                if new_file >= 8 || new_rank >= 8 {
                    break;
                }

                let new_pos = Position::new(new_file, new_rank);
                match board.get_piece(new_pos) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(new_pos);
                        }
                        break;
                    }
                    None => {
                        moves.push(new_pos);
                        distance += 1;
                    }
                }
            } else {
                break;
            }
        }
    }

    moves
}

// Generate rook moves
fn generate_rook_moves(board: &Board, pos: Position, color: Color) -> Vec<Position> {
    let mut moves = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (file_dir, rank_dir) in directions.iter() {
        let mut distance = 1;
        loop {
            if let (Some(new_file), Some(new_rank)) = (
                (pos.file as isize)
                    .checked_add(file_dir * distance)
                    .map(|f| f as usize),
                (pos.rank as isize)
                    .checked_add(rank_dir * distance)
                    .map(|r| r as usize),
            ) {
                if new_file >= 8 || new_rank >= 8 {
                    break;
                }

                let new_pos = Position::new(new_file, new_rank);
                match board.get_piece(new_pos) {
                    Some(piece) => {
                        if piece.color != color {
                            moves.push(new_pos);
                        }
                        break;
                    }
                    None => {
                        moves.push(new_pos);
                        distance += 1;
                    }
                }
            } else {
                break;
            }
        }
    }

    moves
}

// Generate king moves
fn generate_king_moves(board: &Board, pos: Position, color: Color) -> Vec<Position> {
    let mut moves = Vec::new();
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (file_offset, rank_offset) in offsets.iter() {
        if let (Some(new_file), Some(new_rank)) = (
            (pos.file as isize)
                .checked_add(*file_offset)
                .map(|f| f as usize),
            (pos.rank as isize)
                .checked_add(*rank_offset)
                .map(|r| r as usize),
        ) {
            if new_file < 8 && new_rank < 8 {
                let new_pos = Position::new(new_file, new_rank);
                if is_valid_target(board, new_pos, color) {
                    moves.push(new_pos);
                }
            }
        }
    }

    // Castling would be implemented here in a full implementation

    moves
}
