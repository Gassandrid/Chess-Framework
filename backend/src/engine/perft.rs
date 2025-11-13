use crate::engine::movegen::MoveGen;
use crate::engine::position::Position;

pub struct Perft;

impl Perft {
    /// Run perft test to count leaf nodes at a given depth
    pub fn perft(pos: &Position, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = MoveGen::generate_legal_moves(pos);
        let mut nodes = 0;

        for mv in moves.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_ok() {
                nodes += Self::perft(&new_pos, depth - 1);
            }
        }

        nodes
    }

    /// Divide perft - shows move-by-move breakdown
    pub fn divide(pos: &Position, depth: u8) {
        let moves = MoveGen::generate_legal_moves(pos);
        let mut total = 0;

        println!("\nPerft divide (depth {}):", depth);
        println!("------------------------");

        for mv in moves.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_ok() {
                let count = if depth > 1 {
                    Self::perft(&new_pos, depth - 1)
                } else {
                    1
                };
                println!("{}: {}", mv.to_uci(), count);
                total += count;
            }
        }

        println!("------------------------");
        println!("Total: {}", total);
    }

    /// Test against known perft positions
    pub fn test_suite() {
        let tests = vec![
            // Position, depth, expected nodes
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 1, 20),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 2, 400),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 3, 8902),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197281),
            // Kiwipete position
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 1, 48),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 2, 2039),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 3, 97862),
            // Position 3
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 1, 14),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 2, 191),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 3, 2812),
        ];

        println!("Running Perft test suite...\n");

        let mut passed = 0;
        let mut failed = 0;

        for (fen, depth, expected) in tests {
            match Position::from_fen(fen) {
                Ok(pos) => {
                    let start = std::time::Instant::now();
                    let result = Self::perft(&pos, depth);
                    let elapsed = start.elapsed();

                    if result == expected {
                        println!("✓ Depth {} PASS: {} nodes in {:?}", depth, result, elapsed);
                        passed += 1;
                    } else {
                        println!(
                            "✗ Depth {} FAIL: got {}, expected {}",
                            depth, result, expected
                        );
                        println!("  FEN: {}", fen);
                        failed += 1;
                    }
                }
                Err(e) => {
                    println!("✗ Error parsing FEN: {}", e);
                    failed += 1;
                }
            }
        }

        println!("\n------------------------");
        println!("Passed: {}/{}", passed, passed + failed);
        println!("Failed: {}/{}", failed, passed + failed);
    }
}
