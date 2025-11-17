/// Simplified V3 evaluation with piece-square tables
use crate::engine::position::{Position, Color, PieceType};

// Piece values
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

const BISHOP_PAIR_BONUS: i32 = 50;

// Simple piece-square tables
const PAWN_PST: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     50,  50,  50,  50,  50,  50,  50,  50,
     10,  10,  20,  30,  30,  20,  10,  10,
      5,   5,  10,  25,  25,  10,   5,   5,
      0,   0,   0,  20,  20,   0,   0,   0,
      5,  -5, -10,   0,   0, -10,  -5,   5,
      5,  10,  10, -20, -20,  10,  10,   5,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const KING_PST_MG: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];

pub struct EvaluatorV3;

impl EvaluatorV3 {
    pub fn evaluate(pos: &Position) -> i32 {
        let mut score = 0;

        // Material and PST
        let mut white_bishops = 0;
        let mut black_bishops = 0;

        for sq in 0..64 {
            if let Some((piece_type, color)) = pos.piece_at(sq) {
                let base_value = match piece_type {
                    PieceType::Pawn => PAWN_VALUE,
                    PieceType::Knight => KNIGHT_VALUE,
                    PieceType::Bishop => BISHOP_VALUE,
                    PieceType::Rook => ROOK_VALUE,
                    PieceType::Queen => QUEEN_VALUE,
                    PieceType::King => 0,
                };

                let pst_value = Self::get_pst_value(piece_type, sq, color);
                let piece_value = base_value + pst_value;

                if color == Color::White {
                    score += piece_value;
                    if piece_type == PieceType::Bishop {
                        white_bishops += 1;
                    }
                } else {
                    score -= piece_value;
                    if piece_type == PieceType::Bishop {
                        black_bishops += 1;
                    }
                }
            }
        }

        // Bishop pair
        if white_bishops >= 2 {
            score += BISHOP_PAIR_BONUS;
        }
        if black_bishops >= 2 {
            score -= BISHOP_PAIR_BONUS;
        }

        // Return from side to move perspective
        if pos.side_to_move == Color::White {
            score
        } else {
            -score
        }
    }

    fn get_pst_value(piece_type: PieceType, sq: usize, color: Color) -> i32 {
        let sq_idx = if color == Color::White {
            sq
        } else {
            sq ^ 56 // Flip rank
        };

        match piece_type {
            PieceType::Pawn => PAWN_PST[sq_idx],
            PieceType::Knight => KNIGHT_PST[sq_idx],
            PieceType::King => KING_PST_MG[sq_idx],
            _ => 0,
        }
    }
}
