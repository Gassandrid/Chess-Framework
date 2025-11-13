use chess_engine_api::engine::uci::UciEngine;

fn main() {
    let mut engine = UciEngine::new();
    engine.run();
}
