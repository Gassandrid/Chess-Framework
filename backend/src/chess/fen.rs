use crate::chess::board::{Board, Position};

    // fen: Forsyth-Edwards Notation
    // from fen
    // is optional because it may fail to parse the fen, however unlikely
    // pub fn new_fen(fen: &str) -> Option<Self> {
    //     let parts: Vec<&str> = fen.split_whitespace().collect();
    //     if parts.is_empty() {
    //         return None;
    //     }
    //
    //     let position = parts[0];
    //     let mut board = Self::new_empty();
    //     let ranks: Vec<&str> = position.split('/').collect();
    //
    //     if ranks.len() != 8 {
    //         return None;
    //     }
    //
    //     for (rank_idx, rank) in ranks.iter().enumerate() {
    //         let mut file_idx = 0;
    //
    //         for c in rank.chars() {
    //             if file_idx >= 8 {
    //                 return None; // Too many pieces in a rank
    //             }
    //
    //             if c.is_digit(10) {
    //                 // Skip empty squares
    //                 file_idx += c.to_digit(10).unwrap() as usize;
    //             } else if let Some(piece) = Piece::from_fen_char(c) {
    //                 // Place a piece
    //                 board.set_piece(Position::new(file_idx, rank_idx), Some(piece));
    //                 file_idx += 1;
    //             } else {
    //                 return None; // Invalid character
    //             }
    //         }
    //
    //         if file_idx != 8 {
    //             return None; // Rank doesn't have 8 squares
    //         }
    //     }
    //
    //     Some(board)
    // }
    //

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

pub fn to_fen()
