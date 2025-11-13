use chess_engine_api::engine::{
    position::Position,
    movegen::MoveGen,
    search::Search,
    search_v2::SearchV2,
    search_v3::SearchV3,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameResult {
    V1Win,
    V2Win,
    V3Win,
    Draw,
}

fn main() {
    println!("Chess Engine Comparison: V1.0 vs V2.0 vs V3.0");
    println!("==============================================\n");

    println!("V2.0 Improvements over V1.0:");
    println!("  - Better evaluation (improved PST, rook placement, pawn structure)");
    println!("  - Proper MVV-LVA move ordering");
    println!("  - Check extensions");
    println!("  - Improved null move pruning (adaptive R)");
    println!();

    println!("V3.0 Improvements over V2.0:");
    println!("  - Aspiration windows for faster search");
    println!("  - Razoring for early pruning of bad positions");
    println!("  - Futility pruning in late endgame");
    println!("  - More aggressive LMR (2-ply reduction for very late moves)");
    println!("  - Better PVS implementation");
    println!();

    let num_games = 6; // Each pair plays 6 games (3 as white, 3 as black)
    let depth = 5;

    println!("Configuration:");
    println!("  Games per matchup: {} (alternating colors)", num_games);
    println!("  Search depth: {}", depth);
    println!();

    // V1 vs V2
    println!("=== Matchup 1: V1.0 vs V2.0 ===");
    let (v1_wins_1, v2_wins, draws_1) = play_match_v1_v2(num_games, depth);
    print_match_result("V1.0", "V2.0", v1_wins_1, v2_wins, draws_1);

    // V1 vs V3
    println!("\n=== Matchup 2: V1.0 vs V3.0 ===");
    let (v1_wins_2, v3_wins_a, draws_2) = play_match_v1_v3(num_games, depth);
    print_match_result("V1.0", "V3.0", v1_wins_2, v3_wins_a, draws_2);

    // V2 vs V3
    println!("\n=== Matchup 3: V2.0 vs V3.0 ===");
    let (v2_wins_a, v3_wins_b, draws_3) = play_match_v2_v3(num_games, depth);
    print_match_result("V2.0", "V3.0", v2_wins_a, v3_wins_b, draws_3);

    // Final standings
    println!("\n=== Final Standings ===");
    let v1_score = v1_wins_1 as f32 + v1_wins_2 as f32 + draws_1 as f32 * 0.5 + draws_2 as f32 * 0.5;
    let v2_score = v2_wins as f32 + v2_wins_a as f32 + draws_1 as f32 * 0.5 + draws_3 as f32 * 0.5;
    let v3_score = v3_wins_a as f32 + v3_wins_b as f32 + draws_2 as f32 * 0.5 + draws_3 as f32 * 0.5;

    println!("V1.0: {:.1} points", v1_score);
    println!("V2.0: {:.1} points", v2_score);
    println!("V3.0: {:.1} points", v3_score);

    let total = num_games as f32 * 2.0; // Each engine plays 2 matchups
    println!("\nPercentages:");
    println!("V1.0: {:.1}%", (v1_score / total) * 100.0);
    println!("V2.0: {:.1}%", (v2_score / total) * 100.0);
    println!("V3.0: {:.1}%", (v3_score / total) * 100.0);

    if v3_score > v2_score && v3_score > v1_score {
        println!("\n🎉 V3.0 is the strongest!");
    } else if v2_score > v1_score && v2_score >= v3_score {
        println!("\n🎉 V2.0 is the strongest!");
    } else if v1_score >= v2_score && v1_score >= v3_score {
        println!("\n😮 V1.0 is still competitive!");
    }
}

fn print_match_result(name1: &str, name2: &str, wins1: usize, wins2: usize, draws: usize) {
    let score1 = wins1 as f32 + draws as f32 * 0.5;
    let score2 = wins2 as f32 + draws as f32 * 0.5;
    println!("Results:");
    println!("  {} wins: {}", name1, wins1);
    println!("  {} wins: {}", name2, wins2);
    println!("  Draws: {}", draws);
    println!("  Score: {:.1} - {:.1}", score1, score2);
}

fn play_match_v1_v2(num_games: usize, depth: u8) -> (usize, usize, usize) {
    let mut v1_wins = 0;
    let mut v2_wins = 0;
    let mut draws = 0;

    for game_num in 0..num_games {
        let v2_plays_white = game_num % 2 == 0;
        println!("Playing game {}/{} (V2 as {})...",
                 game_num + 1,
                 num_games,
                 if v2_plays_white { "White" } else { "Black" });

        match play_game_v1_v2(v2_plays_white, depth) {
            GameResult::V1Win => v1_wins += 1,
            GameResult::V2Win => v2_wins += 1,
            GameResult::Draw => draws += 1,
            _ => {},
        }
    }

    (v1_wins, v2_wins, draws)
}

fn play_match_v1_v3(num_games: usize, depth: u8) -> (usize, usize, usize) {
    let mut v1_wins = 0;
    let mut v3_wins = 0;
    let mut draws = 0;

    for game_num in 0..num_games {
        let v3_plays_white = game_num % 2 == 0;
        println!("Playing game {}/{} (V3 as {})...",
                 game_num + 1,
                 num_games,
                 if v3_plays_white { "White" } else { "Black" });

        match play_game_v1_v3(v3_plays_white, depth) {
            GameResult::V1Win => v1_wins += 1,
            GameResult::V3Win => v3_wins += 1,
            GameResult::Draw => draws += 1,
            _ => {},
        }
    }

    (v1_wins, v3_wins, draws)
}

fn play_match_v2_v3(num_games: usize, depth: u8) -> (usize, usize, usize) {
    let mut v2_wins = 0;
    let mut v3_wins = 0;
    let mut draws = 0;

    for game_num in 0..num_games {
        let v3_plays_white = game_num % 2 == 0;
        println!("Playing game {}/{} (V3 as {})...",
                 game_num + 1,
                 num_games,
                 if v3_plays_white { "White" } else { "Black" });

        match play_game_v2_v3(v3_plays_white, depth) {
            GameResult::V2Win => v2_wins += 1,
            GameResult::V3Win => v3_wins += 1,
            GameResult::Draw => draws += 1,
            _ => {},
        }
    }

    (v2_wins, v3_wins, draws)
}

fn play_game_v1_v2(v2_plays_white: bool, depth: u8) -> GameResult {
    let mut pos = Position::startpos();
    let mut v1_search = Search::new(64);
    let mut v2_search = SearchV2::new(64);

    let mut position_history = Vec::new();
    let max_moves = 150;
    let mut move_count = 0;

    loop {
        // Threefold repetition
        let current_hash = pos.hash;
        if position_history.iter().filter(|&&h| h == current_hash).count() >= 2 {
            return GameResult::Draw;
        }
        position_history.push(current_hash);

        if pos.halfmove_clock >= 100 || move_count >= max_moves {
            return GameResult::Draw;
        }

        let moves = MoveGen::generate_legal_moves(&pos);
        if moves.is_empty() {
            return if pos.is_check() {
                if pos.side_to_move == chess_engine_api::engine::position::Color::White {
                    GameResult::V2Win
                } else {
                    GameResult::V1Win
                }
            } else {
                GameResult::Draw
            };
        }

        let is_v2_turn = (pos.side_to_move == chess_engine_api::engine::position::Color::White) == v2_plays_white;

        let mv = if is_v2_turn {
            let limits = chess_engine_api::engine::search_v2::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v2_search.search(&pos, limits)
        } else {
            let limits = chess_engine_api::engine::search::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v1_search.search(&pos, limits)
        };

        if let Some(mv) = mv {
            if pos.make_move(mv).is_err() {
                return if is_v2_turn { GameResult::V1Win } else { GameResult::V2Win };
            }
        } else {
            return if is_v2_turn { GameResult::V1Win } else { GameResult::V2Win };
        }

        move_count += 1;
    }
}

fn play_game_v1_v3(v3_plays_white: bool, depth: u8) -> GameResult {
    let mut pos = Position::startpos();
    let mut v1_search = Search::new(64);
    let mut v3_search = SearchV3::new(64);

    let mut position_history = Vec::new();
    let max_moves = 150;
    let mut move_count = 0;

    loop {
        let current_hash = pos.hash;
        if position_history.iter().filter(|&&h| h == current_hash).count() >= 2 {
            return GameResult::Draw;
        }
        position_history.push(current_hash);

        if pos.halfmove_clock >= 100 || move_count >= max_moves {
            return GameResult::Draw;
        }

        let moves = MoveGen::generate_legal_moves(&pos);
        if moves.is_empty() {
            return if pos.is_check() {
                if pos.side_to_move == chess_engine_api::engine::position::Color::White {
                    GameResult::V3Win
                } else {
                    GameResult::V1Win
                }
            } else {
                GameResult::Draw
            };
        }

        let is_v3_turn = (pos.side_to_move == chess_engine_api::engine::position::Color::White) == v3_plays_white;

        let mv = if is_v3_turn {
            let limits = chess_engine_api::engine::search_v3::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v3_search.search(&pos, limits)
        } else {
            let limits = chess_engine_api::engine::search::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v1_search.search(&pos, limits)
        };

        if let Some(mv) = mv {
            if pos.make_move(mv).is_err() {
                return if is_v3_turn { GameResult::V1Win } else { GameResult::V3Win };
            }
        } else {
            return if is_v3_turn { GameResult::V1Win } else { GameResult::V3Win };
        }

        move_count += 1;
    }
}

fn play_game_v2_v3(v3_plays_white: bool, depth: u8) -> GameResult {
    let mut pos = Position::startpos();
    let mut v2_search = SearchV2::new(64);
    let mut v3_search = SearchV3::new(64);

    let mut position_history = Vec::new();
    let max_moves = 150;
    let mut move_count = 0;

    loop {
        let current_hash = pos.hash;
        if position_history.iter().filter(|&&h| h == current_hash).count() >= 2 {
            return GameResult::Draw;
        }
        position_history.push(current_hash);

        if pos.halfmove_clock >= 100 || move_count >= max_moves {
            return GameResult::Draw;
        }

        let moves = MoveGen::generate_legal_moves(&pos);
        if moves.is_empty() {
            return if pos.is_check() {
                if pos.side_to_move == chess_engine_api::engine::position::Color::White {
                    GameResult::V3Win
                } else {
                    GameResult::V2Win
                }
            } else {
                GameResult::Draw
            };
        }

        let is_v3_turn = (pos.side_to_move == chess_engine_api::engine::position::Color::White) == v3_plays_white;

        let mv = if is_v3_turn {
            let limits = chess_engine_api::engine::search_v3::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v3_search.search(&pos, limits)
        } else {
            let limits = chess_engine_api::engine::search_v2::SearchLimits {
                max_depth: Some(depth),
                max_time: Some(Duration::from_millis(1000)),
                ..Default::default()
            };
            v2_search.search(&pos, limits)
        };

        if let Some(mv) = mv {
            if pos.make_move(mv).is_err() {
                return if is_v3_turn { GameResult::V2Win } else { GameResult::V3Win };
            }
        } else {
            return if is_v3_turn { GameResult::V2Win } else { GameResult::V3Win };
        }

        move_count += 1;
    }
}
