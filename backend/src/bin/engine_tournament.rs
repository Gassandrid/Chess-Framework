/// Comprehensive tournament testing all engine versions
use chess_engine_api::engine::{
    tournament::{Tournament, EngineType},
};

fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Chess Engine Framework Tournament System       ║");
    println!("║  Testing V1.0, V2.0, V3.0, V4.0, and MCTS       ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    let mut tournament = Tournament::new();

    // Tournament parameters
    let depth = 5;  // Search depth for alpha-beta engines
    let time_per_move_ms = 2000;  // 2 seconds per move
    let max_moves = 100;  // Maximum moves before declaring draw

    // List of engines to test
    let engines = vec![
        (EngineType::V1, "V1.0 (Basic Alpha-Beta)"),
        (EngineType::V2, "V2.0 (+ Null Move & LMR)"),
        (EngineType::V3, "V3.0 (+ Aspiration Windows)"),
        (EngineType::V4, "V4.0 (+ Multi-Cut & IID)"),
        (EngineType::MCTS, "V5.0 (Monte Carlo Tree Search)"),
    ];

    println!("Tournament Configuration:");
    println!("  Search depth:     {} plies", depth);
    println!("  Time per move:    {}ms", time_per_move_ms);
    println!("  Max moves/game:   {}", max_moves);
    println!("  Games per match:  2 (switch colors)\n");

    println!("Participating Engines:");
    for (i, (_, name)) in engines.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }
    println!();

    // Run round-robin tournament
    let mut total_games = 0;
    let mut results = std::collections::HashMap::new();

    for (_, name) in &engines {
        results.insert(name.to_string(), (0, 0, 0)); // wins, losses, draws
    }

    println!("═══════════════════════════════════════════════════");
    println!("Starting Round-Robin Tournament...");
    println!("═══════════════════════════════════════════════════\n");

    // Each engine plays against every other engine (with color swap)
    for i in 0..engines.len() {
        for j in (i+1)..engines.len() {
            let (engine1, name1) = engines[i];
            let (engine2, name2) = engines[j];

            println!("\n▶ Match {}: {} vs {}", total_games / 2 + 1, name1, name2);
            println!("  ├─ Game 1: {} (White) vs {} (Black)", name1, name2);

            // Game 1: engine1 as white
            let result1 = tournament.play_match(
                engine1,
                engine2,
                depth,
                time_per_move_ms,
                max_moves,
            );

            match result1 {
                chess_engine_api::engine::tournament::GameResult::WhiteWin => {
                    println!("  │  Result: 1-0 (White wins)");
                    let entry = results.get_mut(name1).unwrap();
                    entry.0 += 1;
                    let entry = results.get_mut(name2).unwrap();
                    entry.1 += 1;
                }
                chess_engine_api::engine::tournament::GameResult::BlackWin => {
                    println!("  │  Result: 0-1 (Black wins)");
                    let entry = results.get_mut(name1).unwrap();
                    entry.1 += 1;
                    let entry = results.get_mut(name2).unwrap();
                    entry.0 += 1;
                }
                chess_engine_api::engine::tournament::GameResult::Draw => {
                    println!("  │  Result: ½-½ (Draw)");
                    let entry = results.get_mut(name1).unwrap();
                    entry.2 += 1;
                    let entry = results.get_mut(name2).unwrap();
                    entry.2 += 1;
                }
            }
            total_games += 1;

            println!("  └─ Game 2: {} (White) vs {} (Black)", name2, name1);

            // Game 2: engine2 as white (color swap)
            let result2 = tournament.play_match(
                engine2,
                engine1,
                depth,
                time_per_move_ms,
                max_moves,
            );

            match result2 {
                chess_engine_api::engine::tournament::GameResult::WhiteWin => {
                    println!("     Result: 1-0 (White wins)");
                    let entry = results.get_mut(name2).unwrap();
                    entry.0 += 1;
                    let entry = results.get_mut(name1).unwrap();
                    entry.1 += 1;
                }
                chess_engine_api::engine::tournament::GameResult::BlackWin => {
                    println!("     Result: 0-1 (Black wins)");
                    let entry = results.get_mut(name2).unwrap();
                    entry.1 += 1;
                    let entry = results.get_mut(name1).unwrap();
                    entry.0 += 1;
                }
                chess_engine_api::engine::tournament::GameResult::Draw => {
                    println!("     Result: ½-½ (Draw)");
                    let entry = results.get_mut(name2).unwrap();
                    entry.2 += 1;
                    let entry = results.get_mut(name1).unwrap();
                    entry.2 += 1;
                }
            }
            total_games += 1;
        }
    }

    // Print final standings
    println!("\n═══════════════════════════════════════════════════");
    println!("FINAL STANDINGS");
    println!("═══════════════════════════════════════════════════\n");

    let mut standings: Vec<_> = engines.iter().map(|(_, name)| {
        let (wins, losses, draws) = results[&name.to_string()];
        let games = wins + losses + draws;
        let score = wins as f64 + draws as f64 * 0.5;
        let percentage = if games > 0 { (score / games as f64) * 100.0 } else { 0.0 };
        (name, wins, losses, draws, score, percentage)
    }).collect();

    // Sort by score (descending)
    standings.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap());

    println!("┌─────┬──────────────────────────────────┬──────┬────────┬───────┬──────┬───────┐");
    println!("│ Pos │ Engine                           │ Wins │ Losses │ Draws │ Score│  %    │");
    println!("├─────┼──────────────────────────────────┼──────┼────────┼───────┼──────┼───────┤");

    for (pos, (name, wins, losses, draws, score, percentage)) in standings.iter().enumerate() {
        println!("│ {:3} │ {:<32} │ {:4} │ {:6} │ {:5} │ {:4.1} │ {:5.1}%│",
                 pos + 1, name, wins, losses, draws, score, percentage);
    }

    println!("└─────┴──────────────────────────────────┴──────┴────────┴───────┴──────┴───────┘");

    println!("\n═══════════════════════════════════════════════════");
    println!("Tournament Statistics");
    println!("═══════════════════════════════════════════════════");
    println!("Total games played: {}", total_games);
    println!("Total engines:      {}", engines.len());
    println!("Match format:       Round-robin with color swap");

    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║  Tournament Complete!                            ║");
    println!("╚══════════════════════════════════════════════════╝");
}
