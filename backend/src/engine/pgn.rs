/// PGN (Portable Game Notation) parser and writer
use crate::engine::{
    position::Position,
    movegen::{Move, MoveGen},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PGNGame {
    pub headers: HashMap<String, String>,
    pub moves: Vec<String>,
    pub result: String,
}

impl PGNGame {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            moves: Vec::new(),
            result: "*".to_string(),
        }
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn add_move(&mut self, san: String) {
        self.moves.push(san);
    }

    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        // Write headers (standard seven tag roster first)
        let str_headers = vec!["Event", "Site", "Date", "Round", "White", "Black", "Result"];

        for header in &str_headers {
            if let Some(value) = self.headers.get(*header) {
                pgn.push_str(&format!("[{} \"{}\"]\n", header, value));
            } else {
                pgn.push_str(&format!("[{} \"?\"]\n", header));
            }
        }

        // Write other headers
        for (key, value) in &self.headers {
            if !str_headers.contains(&key.as_str()) {
                pgn.push_str(&format!("[{} \"{}\"]\n", key, value));
            }
        }

        pgn.push('\n');

        // Write moves
        let mut move_text = String::new();
        for (i, san_move) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                move_text.push_str(&format!("{}. ", i / 2 + 1));
            }
            move_text.push_str(san_move);
            move_text.push(' ');

            // Wrap lines at reasonable length
            if move_text.len() > 70 && i % 2 == 1 {
                pgn.push_str(&move_text);
                pgn.push('\n');
                move_text.clear();
            }
        }

        if !move_text.is_empty() {
            pgn.push_str(&move_text);
        }

        pgn.push_str(&self.result);
        pgn.push('\n');

        pgn
    }

    /// Convert UCI move to SAN notation
    pub fn uci_to_san(uci_move: &str, pos: &Position) -> Option<String> {
        let mv = Move::from_uci(uci_move, pos)?;
        Some(Self::move_to_san(&mv, pos))
    }

    /// Convert Move to SAN notation
    pub fn move_to_san(mv: &Move, pos: &Position) -> String {
        let legal_moves = MoveGen::generate_legal_moves(pos);

        // Castling
        let from = mv.from() as usize;
        let to = mv.to() as usize;

        if let Some((piece_type, _)) = pos.piece_at(from as u8) {
            if piece_type == crate::engine::position::PieceType::King {
                let from_file = from % 8;
                let to_file = to % 8;

                if (from_file as i32 - to_file as i32).abs() > 1 {
                    return if to_file > from_file {
                        "O-O".to_string()
                    } else {
                        "O-O-O".to_string()
                    };
                }
            }
        }

        let mut san = String::new();

        // Piece indicator (skip for pawns)
        if let Some((piece_type, _)) = pos.piece_at(from as u8) {
            match piece_type {
                crate::engine::position::PieceType::Knight => san.push('N'),
                crate::engine::position::PieceType::Bishop => san.push('B'),
                crate::engine::position::PieceType::Rook => san.push('R'),
                crate::engine::position::PieceType::Queen => san.push('Q'),
                crate::engine::position::PieceType::King => san.push('K'),
                _ => {} // Pawn
            }
        }

        // Disambiguation
        if san.len() > 0 {
            let mut same_piece_moves = Vec::new();
            for other_mv in legal_moves.iter() {
                if other_mv.to() == mv.to() && other_mv.from() != mv.from() {
                    if let Some((other_piece, _)) = pos.piece_at(other_mv.from()) {
                        if let Some((this_piece, _)) = pos.piece_at(from as u8) {
                            if other_piece == this_piece {
                                same_piece_moves.push(other_mv);
                            }
                        }
                    }
                }
            }

            if !same_piece_moves.is_empty() {
                let from_file = from % 8;
                let from_rank = from / 8;

                let different_files = same_piece_moves.iter()
                    .any(|m| (m.from() as usize % 8) != from_file);

                let different_ranks = same_piece_moves.iter()
                    .any(|m| (m.from() as usize / 8) != from_rank);

                if different_files {
                    san.push((b'a' + from_file as u8) as char);
                } else if different_ranks {
                    san.push((b'1' + from_rank as u8) as char);
                } else {
                    san.push((b'a' + from_file as u8) as char);
                    san.push((b'1' + from_rank as u8) as char);
                }
            }
        }

        // Capture
        if mv.is_capture() {
            if san.is_empty() {
                // Pawn capture
                san.push((b'a' + (from % 8) as u8) as char);
            }
            san.push('x');
        }

        // Destination
        let to_file = to % 8;
        let to_rank = to / 8;
        san.push((b'a' + to_file as u8) as char);
        san.push((b'1' + to_rank as u8) as char);

        // Promotion
        if mv.is_promotion() {
            san.push('=');
            match mv.promotion_piece() {
                Some(crate::engine::position::PieceType::Queen) => san.push('Q'),
                Some(crate::engine::position::PieceType::Rook) => san.push('R'),
                Some(crate::engine::position::PieceType::Bishop) => san.push('B'),
                Some(crate::engine::position::PieceType::Knight) => san.push('N'),
                _ => {}
            }
        }

        // Check/Checkmate
        let mut new_pos = pos.clone();
        if new_pos.make_move(*mv).is_ok() {
            if new_pos.is_check() {
                let legal_moves_after = MoveGen::generate_legal_moves(&new_pos);
                if legal_moves_after.is_empty() {
                    san.push('#');
                } else {
                    san.push('+');
                }
            }
        }

        san
    }
}

impl Default for PGNGame {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse PGN text into games
pub struct PGNParser;

impl PGNParser {
    pub fn parse(pgn_text: &str) -> Vec<PGNGame> {
        let mut games = Vec::new();
        let mut current_game = PGNGame::new();
        let mut in_headers = true;

        for line in pgn_text.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Parse header
            if line.starts_with('[') && line.ends_with(']') {
                in_headers = true;
                let content = &line[1..line.len() - 1];

                if let Some(space_idx) = content.find(' ') {
                    let key = content[..space_idx].to_string();
                    let value = content[space_idx + 1..]
                        .trim_matches('"')
                        .to_string();
                    current_game.set_header(key, value);
                }
            } else {
                // Parse moves
                if in_headers {
                    in_headers = false;
                }

                // Simple move parsing (remove move numbers and result)
                let cleaned = line
                    .replace(char::is_numeric, "")
                    .replace('.', "")
                    .replace("1-0", "")
                    .replace("0-1", "")
                    .replace("1/2-1/2", "")
                    .replace("*", "");

                for token in cleaned.split_whitespace() {
                    if !token.is_empty() && token.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '+' || c == '#' || c == '=') {
                        current_game.add_move(token.to_string());
                    }
                }

                // Check for result
                if line.contains("1-0") {
                    current_game.result = "1-0".to_string();
                    games.push(current_game.clone());
                    current_game = PGNGame::new();
                    in_headers = true;
                } else if line.contains("0-1") {
                    current_game.result = "0-1".to_string();
                    games.push(current_game.clone());
                    current_game = PGNGame::new();
                    in_headers = true;
                } else if line.contains("1/2-1/2") {
                    current_game.result = "1/2-1/2".to_string();
                    games.push(current_game.clone());
                    current_game = PGNGame::new();
                    in_headers = true;
                }
            }
        }

        // Add last game if it exists
        if !current_game.moves.is_empty() {
            games.push(current_game);
        }

        games
    }

    /// Create PGN from a sequence of positions
    pub fn from_positions(positions: &[Position]) -> PGNGame {
        let mut game = PGNGame::new();

        game.set_header("Event".to_string(), "Computer Game".to_string());
        game.set_header("Site".to_string(), "localhost".to_string());
        game.set_header("Date".to_string(), "????.??.??".to_string());
        game.set_header("Round".to_string(), "?".to_string());
        game.set_header("White".to_string(), "Engine".to_string());
        game.set_header("Black".to_string(), "Engine".to_string());
        game.set_header("Result".to_string(), "*".to_string());

        for i in 0..positions.len() - 1 {
            // Find the move that was played
            let legal_moves = MoveGen::generate_legal_moves(&positions[i]);

            for mv in legal_moves.iter() {
                let mut new_pos = positions[i].clone();
                if new_pos.make_move(*mv).is_ok() && new_pos.hash == positions[i + 1].hash {
                    let san = PGNGame::move_to_san(&mv, &positions[i]);
                    game.add_move(san);
                    break;
                }
            }
        }

        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn_creation() {
        let mut game = PGNGame::new();
        game.set_header("Event".to_string(), "Test Game".to_string());
        game.set_header("White".to_string(), "Player 1".to_string());
        game.set_header("Black".to_string(), "Player 2".to_string());
        game.add_move("e4".to_string());
        game.add_move("e5".to_string());
        game.result = "1-0".to_string();

        let pgn = game.to_pgn();
        assert!(pgn.contains("[Event \"Test Game\"]"));
        assert!(pgn.contains("e4 e5"));
    }
}
