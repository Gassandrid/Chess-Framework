use chess_engine_api::engine::testing::{EngineConfig, Tournament};
use std::time::Duration;

fn main() {
    println!("Chess Engine Tournament System");
    println!("==============================\n");

    // Create tournament
    let mut tournament = Tournament::new(10, 200); // 10 games per match, 200 max moves

    // Add different engine configurations to test
    let mut base_config = EngineConfig::new("Baseline v1.0".to_string(), 5);
    base_config.time_per_move = Duration::from_millis(1000);

    let mut deeper_config = EngineConfig::new("Deeper Search v1.1".to_string(), 6);
    deeper_config.time_per_move = Duration::from_millis(1000);

    let mut faster_config = EngineConfig::new("Fast Search v1.2".to_string(), 4);
    faster_config.time_per_move = Duration::from_millis(500);

    // Add engines with initial Elo of 1500
    tournament.add_engine(base_config, 1500.0);
    tournament.add_engine(deeper_config, 1500.0);
    tournament.add_engine(faster_config, 1500.0);

    // Run tournament
    tournament.run_round_robin();

    println!("\n=== Tournament Complete ===");
}
