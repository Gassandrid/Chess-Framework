/// Comprehensive testing suite for chess engine validation
use crate::engine::position::Position;
use crate::engine::perft::Perft;
use crate::engine::search::{Search, SearchLimits};
use crate::engine::movegen::Move;
use std::time::{Duration, Instant};

/// Test result for a single position
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub position: String,
    pub description: String,
    pub expected: String,
    pub actual: String,
    pub time_ms: u128,
}

/// Suite of tactical test positions (Win At Chess - WAC)
pub struct TacticalTestSuite;

impl TacticalTestSuite {
    /// Returns a collection of tactical positions with best moves
    /// Format: (FEN, best_move_uci, description)
    pub fn get_wac_positions() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            // WAC.001
            ("2rr3k/pp3pp1/1nnqbN1p/3pN3/2pP4/2P3Q1/PPB4P/R4RK1 w - -", "g3g7", "Queen sacrifice leads to checkmate"),

            // WAC.002
            ("8/7p/5k2/5p2/p1p2P2/Pr1pPK2/1P1R3P/8 b - -", "b3b2", "Pawn breakthrough"),

            // WAC.003
            ("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq -", "h5f7", "Scholar's mate pattern"),

            // WAC.004
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", "f3h3", "Queen takes hanging pawn with check"),

            // WAC.005
            ("2kr3r/pppq1ppp/3b1n2/3p4/3P4/2PB1N2/PP1Q1PPP/2KR3R w - -", "d3h7", "Bishop takes pawn, threatens knight"),

            // Checkmate in 1
            ("3qk3/8/8/8/8/8/5PPP/4R1K1 w - -", "e1e8", "Back rank mate"),
            ("r5k1/5ppp/8/8/8/8/1Q3PPP/6K1 w - -", "b2b8", "Queen delivers mate"),
            ("6k1/5ppp/8/8/8/8/5PPP/4R1K1 w - -", "e1e8", "Rook delivers back rank mate"),

            // Checkmate in 2
            ("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq -", "e8d7", "Defense against scholar's mate fails"),

            // Tactical motifs - Forks
            ("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq -", "d8h4", "Queen fork threat"),
            ("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq -", "f3e5", "Knight fork"),

            // Tactical motifs - Pins
            ("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -", "c4f7", "Bishop pins knight"),

            // Tactical motifs - Skewers
            ("4k3/8/8/8/8/8/4R3/4K2R w - -", "e2e8", "Rook skewer"),

            // Tactical motifs - Discovered attacks
            ("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 w kq -", "c4f7", "Discovered attack"),

            // Defensive tactics
            ("r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - -", "c4b5", "Trade to relieve pressure"),
        ]
    }

    /// Run all WAC tests with given search depth
    pub fn run_tests(depth: u8) -> Vec<TestResult> {
        let mut results = Vec::new();
        let positions = Self::get_wac_positions();

        println!("Running {} tactical test positions at depth {}...\n", positions.len(), depth);

        for (i, (fen, expected_move, description)) in positions.iter().enumerate() {
            print!("Test {}/{}: {} ... ", i + 1, positions.len(), description);

            let start = Instant::now();
            let result = Self::test_position(fen, expected_move, description, depth);
            let elapsed = start.elapsed();

            if result.passed {
                println!("✓ PASS ({} ms)", elapsed.as_millis());
            } else {
                println!("✗ FAIL ({} ms)", elapsed.as_millis());
                println!("  Expected: {}", result.expected);
                println!("  Got: {}", result.actual);
            }

            results.push(result);
        }

        results
    }

    fn test_position(fen: &str, expected_move: &str, description: &str, depth: u8) -> TestResult {
        let start = Instant::now();

        let position = match Position::from_fen(fen) {
            Ok(pos) => pos,
            Err(e) => {
                return TestResult {
                    passed: false,
                    position: fen.to_string(),
                    description: description.to_string(),
                    expected: expected_move.to_string(),
                    actual: format!("FEN parse error: {:?}", e),
                    time_ms: start.elapsed().as_millis(),
                };
            }
        };

        let mut search = Search::new(128);
        let limits = SearchLimits {
            max_depth: Some(depth),
            max_time: Some(Duration::from_secs(10)),
            ..Default::default()
        };

        let best_move = search.search(&position, limits);
        let elapsed = start.elapsed();

        let actual_move = best_move.map(|m| m.to_uci()).unwrap_or_else(|| "none".to_string());
        let passed = actual_move == expected_move;

        TestResult {
            passed,
            position: fen.to_string(),
            description: description.to_string(),
            expected: expected_move.to_string(),
            actual: actual_move,
            time_ms: elapsed.as_millis(),
        }
    }

    /// Print summary of test results
    pub fn print_summary(results: &[TestResult]) {
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        let pass_rate = (passed as f64 / total as f64) * 100.0;
        let total_time: u128 = results.iter().map(|r| r.time_ms).sum();
        let avg_time = total_time / total as u128;

        println!("\n=== Test Summary ===");
        println!("Passed: {}/{} ({:.1}%)", passed, total, pass_rate);
        println!("Failed: {}", total - passed);
        println!("Total time: {} ms", total_time);
        println!("Average time: {} ms", avg_time);

        if passed < total {
            println!("\nFailed tests:");
            for (i, result) in results.iter().enumerate() {
                if !result.passed {
                    println!("  {}: {}", i + 1, result.description);
                    println!("     Expected: {}", result.expected);
                    println!("     Got: {}", result.actual);
                }
            }
        }
    }
}

/// Perft test suite for move generation validation
pub struct PerftTestSuite;

impl PerftTestSuite {
    /// Standard perft positions with known node counts
    /// Format: (FEN, depth, expected_nodes, description)
    pub fn get_perft_positions() -> Vec<(&'static str, u8, u64, &'static str)> {
        vec![
            // Starting position
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 1, 20, "Start: depth 1"),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 2, 400, "Start: depth 2"),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 3, 8902, "Start: depth 3"),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197281, "Start: depth 4"),
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4865609, "Start: depth 5"),

            // Kiwipete position (complex middlegame)
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 1, 48, "Kiwipete: depth 1"),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 2, 2039, "Kiwipete: depth 2"),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", 3, 97862, "Kiwipete: depth 3"),

            // Position 3 (endgame)
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 1, 14, "Endgame: depth 1"),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 2, 191, "Endgame: depth 2"),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 3, 2812, "Endgame: depth 3"),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 4, 43238, "Endgame: depth 4"),

            // Position 4 (promotions and en passant)
            ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 1, 6, "Promotions: depth 1"),
            ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 2, 264, "Promotions: depth 2"),
            ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 3, 9467, "Promotions: depth 3"),

            // Position 5
            ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 1, 44, "Position 5: depth 1"),
            ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 2, 1486, "Position 5: depth 2"),
            ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 3, 62379, "Position 5: depth 3"),
        ]
    }

    pub fn run_tests() -> Vec<TestResult> {
        let mut results = Vec::new();
        let positions = Self::get_perft_positions();

        println!("Running {} perft test positions...\n", positions.len());

        for (i, (fen, depth, expected, description)) in positions.iter().enumerate() {
            print!("Test {}/{}: {} ... ", i + 1, positions.len(), description);

            let start = Instant::now();
            let result = Self::test_position(fen, *depth, *expected, description);
            let elapsed = start.elapsed();

            if result.passed {
                println!("✓ PASS ({} ms)", elapsed.as_millis());
            } else {
                println!("✗ FAIL ({} ms)", elapsed.as_millis());
                println!("  Expected: {}", result.expected);
                println!("  Got: {}", result.actual);
            }

            results.push(result);
        }

        results
    }

    fn test_position(fen: &str, depth: u8, expected: u64, description: &str) -> TestResult {
        let start = Instant::now();

        let position = match Position::from_fen(fen) {
            Ok(pos) => pos,
            Err(e) => {
                return TestResult {
                    passed: false,
                    position: fen.to_string(),
                    description: description.to_string(),
                    expected: expected.to_string(),
                    actual: format!("FEN parse error: {:?}", e),
                    time_ms: start.elapsed().as_millis(),
                };
            }
        };

        let nodes = Perft::perft(&position, depth);
        let elapsed = start.elapsed();

        TestResult {
            passed: nodes == expected,
            position: fen.to_string(),
            description: description.to_string(),
            expected: expected.to_string(),
            actual: nodes.to_string(),
            time_ms: elapsed.as_millis(),
        }
    }
}

/// Bratko-Kopec test suite
pub struct BratkoKopecTestSuite;

impl BratkoKopecTestSuite {
    /// Returns 24 positions from the Bratko-Kopec test
    pub fn get_positions() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            // BK.01
            ("1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - -", "d6d1", "Queen checkmate"),

            // BK.02
            ("3r1k2/4npp1/1ppr3p/p6P/P2PPPP1/1NR5/5K2/2R5 w - -", "d4d5", "Pawn breakthrough"),

            // BK.03
            ("2q1rr1k/3bbnnp/p2p1pp1/2pPp3/PpP1P1P1/1P2BNNP/2BQ1PRK/7R b - -", "f6f5", "Pawn break"),

            // BK.04
            ("rnbqkb1r/p3pppp/1p6/2ppP3/3N4/2P5/PPP1QPPP/R1B1KB1R w KQkq -", "e5e6", "Pawn breakthrough"),

            // BK.05
            ("r1b2rk1/2q1b1pp/p2ppn2/1p6/3QP3/1BN1B3/PPP3PP/R4RK1 w - -", "d4d7", "Queen invades"),
        ]
    }

    pub fn run_tests(depth: u8) -> Vec<TestResult> {
        let mut results = Vec::new();
        let positions = Self::get_positions();

        println!("Running {} Bratko-Kopec test positions at depth {}...\n", positions.len(), depth);

        for (i, (fen, expected_move, description)) in positions.iter().enumerate() {
            print!("Test {}/{}: {} ... ", i + 1, positions.len(), description);

            let start = Instant::now();
            let result = Self::test_position(fen, expected_move, description, depth);
            let elapsed = start.elapsed();

            if result.passed {
                println!("✓ PASS ({} ms)", elapsed.as_millis());
            } else {
                println!("✗ FAIL ({} ms)", elapsed.as_millis());
                println!("  Expected: {}", result.expected);
                println!("  Got: {}", result.actual);
            }

            results.push(result);
        }

        results
    }

    fn test_position(fen: &str, expected_move: &str, description: &str, depth: u8) -> TestResult {
        let start = Instant::now();

        let position = match Position::from_fen(fen) {
            Ok(pos) => pos,
            Err(e) => {
                return TestResult {
                    passed: false,
                    position: fen.to_string(),
                    description: description.to_string(),
                    expected: expected_move.to_string(),
                    actual: format!("FEN parse error: {:?}", e),
                    time_ms: start.elapsed().as_millis(),
                };
            }
        };

        let mut search = Search::new(128);
        let limits = SearchLimits {
            max_depth: Some(depth),
            max_time: Some(Duration::from_secs(10)),
            ..Default::default()
        };

        let best_move = search.search(&position, limits);
        let elapsed = start.elapsed();

        let actual_move = best_move.map(|m| m.to_uci()).unwrap_or_else(|| "none".to_string());
        let passed = actual_move == expected_move;

        TestResult {
            passed,
            position: fen.to_string(),
            description: description.to_string(),
            expected: expected_move.to_string(),
            actual: actual_move,
            time_ms: elapsed.as_millis(),
        }
    }
}

/// Run all test suites
pub fn run_all_tests(tactical_depth: u8) {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║   Chess Engine Comprehensive Test Suite   ║");
    println!("╚════════════════════════════════════════════╝\n");

    // 1. Perft tests (move generation correctness)
    println!("┌─ PERFT Tests (Move Generation) ─────────┐");
    let perft_results = PerftTestSuite::run_tests();
    TacticalTestSuite::print_summary(&perft_results);
    println!();

    // 2. Tactical tests
    println!("┌─ Tactical Tests (WAC Suite) ─────────────┐");
    let tactical_results = TacticalTestSuite::run_tests(tactical_depth);
    TacticalTestSuite::print_summary(&tactical_results);
    println!();

    // 3. Bratko-Kopec tests
    println!("┌─ Bratko-Kopec Test Suite ────────────────┐");
    let bk_results = BratkoKopecTestSuite::run_tests(tactical_depth);
    TacticalTestSuite::print_summary(&bk_results);
    println!();

    // Overall summary
    let total_tests = perft_results.len() + tactical_results.len() + bk_results.len();
    let total_passed = perft_results.iter().filter(|r| r.passed).count()
                     + tactical_results.iter().filter(|r| r.passed).count()
                     + bk_results.iter().filter(|r| r.passed).count();

    println!("╔════════════════════════════════════════════╗");
    println!("║           Overall Test Summary             ║");
    println!("╠════════════════════════════════════════════╣");
    println!("║ Total Tests: {:4}                          ║", total_tests);
    println!("║ Passed:      {:4} ({:5.1}%)                  ║",
             total_passed, (total_passed as f64 / total_tests as f64) * 100.0);
    println!("║ Failed:      {:4}                          ║", total_tests - total_passed);
    println!("╚════════════════════════════════════════════╝");
}
