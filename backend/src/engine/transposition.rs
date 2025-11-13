use crate::engine::movegen::Move;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Exact,      // Exact score (PV node)
    LowerBound, // Beta cutoff (fail-high)
    UpperBound, // Alpha cutoff (fail-low)
}

#[derive(Debug, Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: u8,
    pub score: i32,
    pub node_type: NodeType,
    pub best_move: Option<Move>,
    pub age: u8,
}

impl TTEntry {
    pub fn new(hash: u64, depth: u8, score: i32, node_type: NodeType, best_move: Option<Move>, age: u8) -> Self {
        Self {
            hash,
            depth,
            score,
            node_type,
            best_move,
            age,
        }
    }
}

pub struct TranspositionTable {
    table: Vec<RwLock<Option<TTEntry>>>,
    size: usize,
    age: u8,
}

impl TranspositionTable {
    /// Create a new transposition table with given size in MB
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<TTEntry>>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;

        // Round to power of 2 for efficient modulo
        let size = num_entries.next_power_of_two();

        Self {
            table: (0..size).map(|_| RwLock::new(None)).collect(),
            size,
            age: 0,
        }
    }

    /// Probe the transposition table
    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash as usize) % self.size;
        if let Ok(entry) = self.table[index].read() {
            if let Some(e) = *entry {
                if e.hash == hash {
                    return Some(e);
                }
            }
        }
        None
    }

    /// Store an entry in the transposition table
    pub fn store(&self, hash: u64, depth: u8, score: i32, node_type: NodeType, best_move: Option<Move>) {
        let index = (hash as usize) % self.size;
        let new_entry = TTEntry::new(hash, depth, score, node_type, best_move, self.age);

        if let Ok(mut entry) = self.table[index].write() {
            // Replacement scheme: replace if:
            // 1. Slot is empty
            // 2. Same position (hash match)
            // 3. New entry has greater depth
            // 4. Entry is from old search (different age)
            let should_replace = match *entry {
                None => true,
                Some(old_entry) => {
                    old_entry.hash == hash
                        || new_entry.depth >= old_entry.depth
                        || old_entry.age != self.age
                }
            };

            if should_replace {
                *entry = Some(new_entry);
            }
        }
    }

    /// Clear the transposition table
    pub fn clear(&mut self) {
        for entry in &self.table {
            if let Ok(mut e) = entry.write() {
                *e = None;
            }
        }
    }

    /// Increment age for new search
    pub fn new_search(&mut self) {
        self.age = self.age.wrapping_add(1);
    }

    /// Get table usage (percentage of filled entries)
    pub fn usage(&self) -> f64 {
        let mut filled = 0;
        let sample_size = 1000.min(self.size);

        for i in 0..sample_size {
            if let Ok(entry) = self.table[i].read() {
                if entry.is_some() {
                    filled += 1;
                }
            }
        }

        (filled as f64 / sample_size as f64) * 100.0
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(64) // 64 MB default
    }
}

// Zobrist hashing for incremental hash updates
pub struct ZobristKeys {
    pub pieces: [[[u64; 64]; 6]; 2], // [color][piece_type][square]
    pub castling: [u64; 16],          // Castling rights combinations
    pub en_passant: [u64; 8],         // En passant file
    pub side_to_move: u64,            // Side to move
}

impl ZobristKeys {
    pub fn new() -> Self {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut rng = StdRng::seed_from_u64(0x123456789ABCDEF0);

        let mut pieces = [[[0u64; 64]; 6]; 2];
        for color in 0..2 {
            for piece_type in 0..6 {
                for square in 0..64 {
                    pieces[color][piece_type][square] = rng.gen();
                }
            }
        }

        let mut castling = [0u64; 16];
        for i in 0..16 {
            castling[i] = rng.gen();
        }

        let mut en_passant = [0u64; 8];
        for i in 0..8 {
            en_passant[i] = rng.gen();
        }

        let side_to_move = rng.gen();

        Self {
            pieces,
            castling,
            en_passant,
            side_to_move,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref ZOBRIST: ZobristKeys = ZobristKeys::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transposition_table() {
        let tt = TranspositionTable::new(1); // 1 MB

        // Store an entry
        tt.store(0x123456, 10, 100, NodeType::Exact, None);

        // Probe for the entry
        let entry = tt.probe(0x123456);
        assert!(entry.is_some());

        let entry = entry.unwrap();
        assert_eq!(entry.depth, 10);
        assert_eq!(entry.score, 100);
        assert_eq!(entry.node_type, NodeType::Exact);
    }

    #[test]
    fn test_replacement_scheme() {
        let tt = TranspositionTable::new(1);

        // Store an entry
        tt.store(0x123456, 5, 50, NodeType::Exact, None);

        // Store a deeper entry with same hash - should replace
        tt.store(0x123456, 10, 100, NodeType::Exact, None);

        let entry = tt.probe(0x123456).unwrap();
        assert_eq!(entry.depth, 10);
        assert_eq!(entry.score, 100);
    }
}
