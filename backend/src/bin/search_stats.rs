/// Demonstration of detailed search statistics from V4 engine
use chess_engine_api::engine::{
    Position,
    search_v4::{SearchV4, SearchLimits},
};
use std::time::Duration;

fn main() {
    println!("Chess Engine V4.0 - Detailed Search Statistics Demo");
    println!("====================================================\n");

    // Test position: starting position
    let pos = Position::startpos();

    // Create V4 search engine
    let mut search = SearchV4::new(64);  // 64MB TT

    println!("Position: Starting position");
    println!("Search depth: 6\n");

    // Search to depth 6
    let limits = SearchLimits {
        max_depth: Some(6),
        max_time: Some(Duration::from_secs(10)),
        ..Default::default()
    };

    println!("Searching...\n");
    if let Some(best_move) = search.search(&pos, limits) {
        println!("Best move: {}", best_move.to_uci());
    }

    // Get and print detailed statistics
    let stats = search.get_stats();
    stats.print_detailed();

    println!("\n=== Analysis ===");
    println!("Effective branching factor: ~{:.2}", stats.branching_factor());
    println!("Move ordering effectiveness: {:.1}%", stats.move_ordering_quality());
    println!("TT hit rate: {:.1}%", stats.tt_hit_rate());

    // Test on tactical position
    println!("\n\n=== Tactical Position Test ===");
    let tactical_fen = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
    println!("Position: Italian Game");

    if let Ok(pos2) = Position::from_fen(tactical_fen) {
        let mut search2 = SearchV4::new(64);

        let limits2 = SearchLimits {
            max_depth: Some(7),
            max_time: Some(Duration::from_secs(10)),
            ..Default::default()
        };

        if let Some(best_move) = search2.search(&pos2, limits2) {
            println!("Best move: {}", best_move.to_uci());
        }

        let stats2 = search2.get_stats();
        stats2.print_detailed();
    }

    println!("\n=== Statistics Legend ===");
    println!("PV nodes: Principal variation nodes (best line)");
    println!("Cut nodes: Nodes that caused beta cutoff");
    println!("All nodes: Nodes where all moves were searched");
    println!("Null move cuts: Positions where null move caused beta cutoff");
    println!("LMR searches: Late moves that were searched with reduced depth");
    println!("TT hits: Positions found in transposition table");
    println!("First move cuts: Beta cutoffs on the first move (best move ordering)");
    println!("\n Branching factor: Average number of moves considered per position");
    println!("Move ordering: Percentage of cutoffs on first move (higher is better)");
    println!("TT hit rate: Percentage of positions found in hash table");
}
