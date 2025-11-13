use crate::engine::bitboard::Bitboard;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

impl Not for Color {
    type Output = Color;
    #[inline]
    fn not(self) -> Self::Output {
        self.opposite()
    }
}

use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    pub const ALL: [PieceType; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'r' => Some(PieceType::Rook),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn to_char(self, color: Color) -> char {
        let c = match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        if color == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    rights: u8,
}

impl CastlingRights {
    pub const NONE: u8 = 0;
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;
    pub const WHITE_BOTH: u8 = Self::WHITE_KINGSIDE | Self::WHITE_QUEENSIDE;
    pub const BLACK_BOTH: u8 = Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE;
    pub const ALL: u8 = Self::WHITE_BOTH | Self::BLACK_BOTH;

    #[inline]
    pub const fn new(rights: u8) -> Self {
        Self { rights }
    }

    #[inline]
    pub fn has(&self, right: u8) -> bool {
        (self.rights & right) != 0
    }

    #[inline]
    pub fn add(&mut self, right: u8) {
        self.rights |= right;
    }

    #[inline]
    pub fn remove(&mut self, right: u8) {
        self.rights &= !right;
    }

    #[inline]
    pub fn get(&self) -> u8 {
        self.rights
    }
}

/// Represents a chess position with full state
#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    // Piece bitboards [color][piece_type]
    pub pieces: [[Bitboard; 6]; 2],
    // Occupied squares by color
    pub occupied_by_color: [Bitboard; 2],
    // All occupied squares
    pub occupied: Bitboard,
    // Side to move
    pub side_to_move: Color,
    // Castling rights
    pub castling_rights: CastlingRights,
    // En passant target square (if any)
    pub en_passant_square: Option<u8>,
    // Halfmove clock for 50-move rule
    pub halfmove_clock: u16,
    // Fullmove number
    pub fullmove_number: u16,
    // Zobrist hash for transposition table
    pub hash: u64,
}

impl Position {
    /// Create an empty position
    pub fn empty() -> Self {
        Self {
            pieces: [[Bitboard::EMPTY; 6]; 2],
            occupied_by_color: [Bitboard::EMPTY; 2],
            occupied: Bitboard::EMPTY,
            side_to_move: Color::White,
            castling_rights: CastlingRights::new(CastlingRights::NONE),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: 0,
        }
    }

    /// Create the standard starting position
    pub fn startpos() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    /// Parse a FEN string into a position
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("Invalid FEN: not enough parts".to_string());
        }

        let mut pos = Self::empty();

        // Parse piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("Invalid FEN: must have 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_idx;
            let mut file = 0;

            for c in rank_str.chars() {
                if c.is_ascii_digit() {
                    file += c.to_digit(10).unwrap() as usize;
                } else {
                    let color = if c.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let piece_type = PieceType::from_char(c)
                        .ok_or_else(|| format!("Invalid piece character: {}", c))?;

                    if file >= 8 {
                        return Err("Invalid FEN: too many files".to_string());
                    }

                    let square = Bitboard::square_from_coords(file as u8, rank as u8);
                    pos.put_piece(square, piece_type, color);
                    file += 1;
                }
            }

            if file != 8 {
                return Err(format!("Invalid FEN: rank {} has {} files, expected 8", rank_idx, file));
            }
        }

        // Parse side to move
        pos.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN: invalid side to move".to_string()),
        };

        // Parse castling rights
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => pos.castling_rights.add(CastlingRights::WHITE_KINGSIDE),
                    'Q' => pos.castling_rights.add(CastlingRights::WHITE_QUEENSIDE),
                    'k' => pos.castling_rights.add(CastlingRights::BLACK_KINGSIDE),
                    'q' => pos.castling_rights.add(CastlingRights::BLACK_QUEENSIDE),
                    _ => return Err(format!("Invalid castling right: {}", c)),
                }
            }
        }

        // Parse en passant square
        if parts[3] != "-" {
            pos.en_passant_square = Bitboard::square_from_name(parts[3]);
            if pos.en_passant_square.is_none() {
                return Err("Invalid en passant square".to_string());
            }
        }

        // Parse halfmove clock
        if parts.len() > 4 {
            pos.halfmove_clock = parts[4].parse().map_err(|_| "Invalid halfmove clock")?;
        }

        // Parse fullmove number
        if parts.len() > 5 {
            pos.fullmove_number = parts[5].parse().map_err(|_| "Invalid fullmove number")?;
        }

        pos.update_hash();
        Ok(pos)
    }

    /// Convert position to FEN string
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let square = Bitboard::square_from_coords(file, rank);
                if let Some((piece_type, color)) = self.piece_at(square) {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(piece_type.to_char(color));
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling rights
        fen.push(' ');
        let mut has_castling = false;
        if self.castling_rights.has(CastlingRights::WHITE_KINGSIDE) {
            fen.push('K');
            has_castling = true;
        }
        if self.castling_rights.has(CastlingRights::WHITE_QUEENSIDE) {
            fen.push('Q');
            has_castling = true;
        }
        if self.castling_rights.has(CastlingRights::BLACK_KINGSIDE) {
            fen.push('k');
            has_castling = true;
        }
        if self.castling_rights.has(CastlingRights::BLACK_QUEENSIDE) {
            fen.push('q');
            has_castling = true;
        }
        if !has_castling {
            fen.push('-');
        }

        // En passant square
        fen.push(' ');
        if let Some(sq) = self.en_passant_square {
            fen.push_str(&Bitboard::square_to_name(sq));
        } else {
            fen.push('-');
        }

        // Halfmove and fullmove
        fen.push_str(&format!(" {} {}", self.halfmove_clock, self.fullmove_number));

        fen
    }

    /// Put a piece on a square
    #[inline]
    pub fn put_piece(&mut self, square: u8, piece_type: PieceType, color: Color) {
        let bb = Bitboard::from_square(square);
        self.pieces[color.index()][piece_type.index()] |= bb;
        self.occupied_by_color[color.index()] |= bb;
        self.occupied |= bb;
    }

    /// Remove a piece from a square
    #[inline]
    pub fn remove_piece(&mut self, square: u8, piece_type: PieceType, color: Color) {
        let bb = !Bitboard::from_square(square);
        self.pieces[color.index()][piece_type.index()] &= bb;
        self.occupied_by_color[color.index()] &= bb;
        self.occupied &= bb;
    }

    /// Get the piece at a square
    #[inline]
    pub fn piece_at(&self, square: u8) -> Option<(PieceType, Color)> {
        let bb = Bitboard::from_square(square);

        for color in [Color::White, Color::Black] {
            if (self.occupied_by_color[color.index()] & bb).is_not_empty() {
                for piece_type in PieceType::ALL {
                    if (self.pieces[color.index()][piece_type.index()] & bb).is_not_empty() {
                        return Some((piece_type, color));
                    }
                }
            }
        }

        None
    }

    /// Get bitboard for a specific piece type and color
    #[inline]
    pub fn piece_bb(&self, piece_type: PieceType, color: Color) -> Bitboard {
        self.pieces[color.index()][piece_type.index()]
    }

    /// Get all pieces of a color
    #[inline]
    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.occupied_by_color[color.index()]
    }

    /// Update the Zobrist hash (simplified version - full implementation would use precomputed random numbers)
    pub fn update_hash(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        for color in [Color::White, Color::Black] {
            for piece_type in PieceType::ALL {
                self.pieces[color.index()][piece_type.index()].0.hash(&mut hasher);
            }
        }

        (self.side_to_move as u8).hash(&mut hasher);
        self.castling_rights.get().hash(&mut hasher);
        self.en_passant_square.hash(&mut hasher);

        self.hash = hasher.finish();
    }

    /// Check if the current side is in check
    pub fn is_check(&self) -> bool {
        let king_square = self.piece_bb(PieceType::King, self.side_to_move).lsb();
        self.is_square_attacked(king_square, !self.side_to_move)
    }

    /// Check if a square is attacked by a given side
    pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
        // This is a simplified check - full implementation would use attack tables
        let target = Bitboard::from_square(square);

        // Check pawn attacks
        let pawn_attacks = if by_color == Color::White {
            target.south_west() | target.south_east()
        } else {
            target.north_west() | target.north_east()
        };
        if (pawn_attacks & self.piece_bb(PieceType::Pawn, by_color)).is_not_empty() {
            return true;
        }

        // Check knight attacks
        let knight_attacks = self.get_knight_attacks(square);
        if (knight_attacks & self.piece_bb(PieceType::Knight, by_color)).is_not_empty() {
            return true;
        }

        // Check king attacks
        let king_attacks = self.get_king_attacks(square);
        if (king_attacks & self.piece_bb(PieceType::King, by_color)).is_not_empty() {
            return true;
        }

        // Simplified sliding piece checks (diagonal and straight)
        // In a full implementation, this would use magic bitboards or similar
        false
    }

    fn get_knight_attacks(&self, square: u8) -> Bitboard {
        let bb = Bitboard::from_square(square);
        let l1 = (bb >> 1) & Bitboard::NOT_H_FILE;
        let l2 = (bb >> 2) & Bitboard::NOT_GH_FILE;
        let r1 = (bb << 1) & Bitboard::NOT_A_FILE;
        let r2 = (bb << 2) & Bitboard::NOT_AB_FILE;
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
    }

    fn get_king_attacks(&self, square: u8) -> Bitboard {
        let bb = Bitboard::from_square(square);
        let attacks = bb.east() | bb.west();
        let bb_with_attacks = bb | attacks;
        bb_with_attacks.north() | bb_with_attacks.south() | attacks
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n  a b c d e f g h")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = Bitboard::square_from_coords(file, rank);
                if let Some((piece_type, color)) = self.piece_at(square) {
                    write!(f, "{} ", piece_type.to_char(color))?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "\nFEN: {}", self.to_fen())?;
        writeln!(f, "Hash: {:016x}", self.hash)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpos() {
        let pos = Position::startpos();
        assert_eq!(pos.side_to_move, Color::White);
        assert_eq!(pos.piece_bb(PieceType::Pawn, Color::White).count(), 8);
        assert_eq!(pos.piece_bb(PieceType::Pawn, Color::Black).count(), 8);
    }

    #[test]
    fn test_fen_roundtrip() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }
}
