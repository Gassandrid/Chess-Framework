use crate::chess::piece::{Color, Piece, PieceType};
use serde::{Deserialize, Serialize};

// Represents a position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub rank: usize, // 0 to 7
    pub file: usize, // 0 to 7
}

impl Position {
    pub fn new(file: usize, rank: usize) -> Self {
        Self { file, rank }
    }

    // Convert from algebraic notation (e.g., "e4")
    pub fn from_algebraic(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }

        let chars: Vec<char> = s.chars().collect();
        let file = chars[0] as usize - 'a' as usize;
        let rank = 8 - chars[1].to_digit(10)? as usize;

        if file < 8 && rank < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }

    // Convert to algebraic notation
    pub fn to_algebraic(&self) -> String {
        let file = (self.file as u8 + b'a') as char;
        let rank = (8 - self.rank) as u8;
        format!("{}{}", file, rank)
    }
}

// The chess board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    // empty board
    pub fn new_empty() -> Self {
        Self {
            squares: [[None; 8]; 8],
        }
    }

    // fen: Forsyth-Edwards Notation
    // from fen
    // is optional because it may fail to parse the fen, however unlikely
    pub fn from_fen(fen: &str) -> Option<Self> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let position = parts[0];
        let mut board = Self::new_empty();
        let ranks: Vec<&str> = position.split('/').collect();

        if ranks.len() != 8 {
            return None;
        }

        for (rank_idx, rank) in ranks.iter().enumerate() {
            let mut file_idx = 0;

            for c in rank.chars() {
                if file_idx >= 8 {
                    return None; // Too many pieces in a rank
                }

                if c.is_digit(10) {
                    // Skip empty squares
                    file_idx += c.to_digit(10).unwrap() as usize;
                } else if let Some(piece) = Piece::from_fen_char(c) {
                    // Place a piece
                    board.set_piece(Position::new(file_idx, rank_idx), Some(piece));
                    file_idx += 1;
                } else {
                    return None; // Invalid character
                }
            }

            if file_idx != 8 {
                return None; // Rank doesn't have 8 squares
            }
        }

        Some(board)
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank in 0..8 {
            let mut empty_count = 0;

            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    fen.push(piece.to_fen_char());
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank < 7 {
                fen.push('/');
            }
        }
        fen.push_str(" w KQkq - 0 1");
        fen
    }

    // standard position
    pub fn new_standard() -> Self {
        let mut board = Self::new_empty();

        // Set up pawns
        for file in 0..8 {
            board.set_piece(
                Position::new(file, 1),
                Some(Piece::new(PieceType::Pawn, Color::Black)),
            );
            board.set_piece(
                Position::new(file, 6),
                Some(Piece::new(PieceType::Pawn, Color::White)),
            );
        }

        // Set up other pieces
        let back_rank_pieces = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (file, &piece_type) in back_rank_pieces.iter().enumerate() {
            board.set_piece(
                Position::new(file, 0),
                Some(Piece::new(piece_type, Color::Black)),
            );
            board.set_piece(
                Position::new(file, 7),
                Some(Piece::new(piece_type, Color::White)),
            );
        }

        board
    }

    // Get a piece at a position
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.rank < 8 && pos.file < 8 {
            self.squares[pos.rank][pos.file]
        } else {
            None
        }
    }

    // Set a piece at a position
    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        if pos.rank < 8 && pos.file < 8 {
            self.squares[pos.rank][pos.file] = piece;
        }
    }

    // Convert to a 2D array representation for the frontend
    pub fn to_array(&self) -> [[Option<char>; 8]; 8] {
        let mut result = [[None; 8]; 8];

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(piece) = self.squares[rank][file] {
                    result[rank][file] = Some(piece.to_fen_char());
                }
            }
        }

        result
    }

    // Create from a 2D array representation from the frontend
    pub fn from_array(array: [[Option<char>; 8]; 8]) -> Self {
        let mut board = Self::new_empty();

        for rank in 0..8 {
            for file in 0..8 {
                if let Some(c) = array[rank][file] {
                    if let Some(piece) = Piece::from_fen_char(c) {
                        board.squares[rank][file] = Some(piece);
                    }
                }
            }
        }

        board
    }
}
