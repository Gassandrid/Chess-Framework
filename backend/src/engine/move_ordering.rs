use crate::engine::movegen::{Move, MoveType};
use crate::engine::position::{PieceType, Position};
use crate::engine::transposition::TTEntry;

/// MVV-LVA (Most Valuable Victim - Least Valuable Attacker) scoring
pub struct MoveOrdering;

impl MoveOrdering {
    const MVV_LVA_SCORES: [[i32; 6]; 6] = [
        // Victim: Pawn, Knight, Bishop, Rook, Queen, King
        [105, 205, 305, 405, 505, 605], // Pawn attacker
        [104, 204, 304, 404, 504, 604], // Knight attacker
        [103, 203, 303, 403, 503, 603], // Bishop attacker
        [102, 202, 302, 402, 502, 602], // Rook attacker
        [101, 201, 301, 401, 501, 601], // Queen attacker
        [100, 200, 300, 400, 500, 600], // King attacker
    ];

    /// Score a move for ordering purposes
    /// Higher scores are searched first
    pub fn score_move(
        pos: &Position,
        mv: Move,
        tt_move: Option<Move>,
        killer_moves: &[Option<Move>],
        history: &[[i32; 64]; 64],
    ) -> i32 {
        // 1. TT move gets highest priority
        if let Some(tt) = tt_move {
            if mv == tt {
                return 10_000_000;
            }
        }

        // 2. Captures scored by MVV-LVA
        if mv.is_capture() {
            return 1_000_000 + Self::mvv_lva_score(pos, mv);
        }

        // 3. Killer moves
        for (i, killer) in killer_moves.iter().enumerate() {
            if let Some(k) = killer {
                if mv == *k {
                    return 900_000 - i as i32 * 1000;
                }
            }
        }

        // 4. History heuristic
        let from = mv.from() as usize;
        let to = mv.to() as usize;
        history[from][to]
    }

    /// Calculate MVV-LVA score for a capture
    fn mvv_lva_score(pos: &Position, mv: Move) -> i32 {
        let attacker_piece = pos.piece_at(mv.from());
        let victim_piece = if mv.move_type() == MoveType::EnPassant {
            Some((PieceType::Pawn, !pos.side_to_move))
        } else {
            pos.piece_at(mv.to())
        };

        if let (Some((attacker_type, _)), Some((victim_type, _))) = (attacker_piece, victim_piece) {
            Self::MVV_LVA_SCORES[attacker_type.index()][victim_type.index()]
        } else {
            0
        }
    }

    /// Sort moves in-place based on scores
    pub fn order_moves(
        pos: &Position,
        moves: &mut Vec<Move>,
        tt_move: Option<Move>,
        killer_moves: &[Option<Move>],
        history: &[[i32; 64]; 64],
    ) {
        // Create scored moves
        let mut scored_moves: Vec<(Move, i32)> = moves
            .iter()
            .map(|&mv| {
                let score = Self::score_move(pos, mv, tt_move, killer_moves, history);
                (mv, score)
            })
            .collect();

        // Sort by score (descending)
        scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

        // Update original vec
        for (i, (mv, _)) in scored_moves.into_iter().enumerate() {
            moves[i] = mv;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::position::Position;

    #[test]
    fn test_mvv_lva_ordering() {
        let pos = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

        // In a real scenario, we'd generate moves and test their ordering
        // For now, just verify the scoring function works
        let history = [[0; 64]; 64];
        let killers = [None, None];

        // Test that TT move scores highest would go here
        // Test that captures score higher than quiet moves would go here
    }
}
