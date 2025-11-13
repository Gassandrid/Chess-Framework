use crate::engine::movegen::Move;
use crate::engine::position::Position;
use std::collections::HashMap;

/// Simple opening book using a hash map
pub struct OpeningBook {
    book: HashMap<u64, Vec<(Move, u32)>>, // hash -> (move, weight)
}

impl OpeningBook {
    pub fn new() -> Self {
        Self {
            book: HashMap::new(),
        }
    }

    /// Add a move to the opening book
    pub fn add_move(&mut self, pos: &Position, mv: Move, weight: u32) {
        self.book
            .entry(pos.hash)
            .or_insert_with(Vec::new)
            .push((mv, weight));
    }

    /// Get a move from the opening book
    pub fn get_move(&self, pos: &Position) -> Option<Move> {
        if let Some(moves) = self.book.get(&pos.hash) {
            if moves.is_empty() {
                return None;
            }

            // Select move based on weight
            let total_weight: u32 = moves.iter().map(|(_, w)| w).sum();
            if total_weight == 0 {
                return Some(moves[0].0);
            }

            let mut rng = total_weight % 100; // Simple pseudo-random
            for (mv, weight) in moves {
                if rng < *weight {
                    return Some(*mv);
                }
                rng -= weight;
            }

            Some(moves[0].0)
        } else {
            None
        }
    }

    /// Load from PGN (simplified - would need a full PGN parser)
    pub fn load_from_pgn(&mut self, _pgn: &str) {
        // TODO: Implement PGN parsing
        // For now, this is a stub
    }

    /// Clear the book
    pub fn clear(&mut self) {
        self.book.clear();
    }
}

impl Default for OpeningBook {
    fn default() -> Self {
        Self::new()
    }
}
