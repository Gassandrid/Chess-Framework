/// Tournament manager for engine vs engine matches
use crate::engine::{
    position::Position,
    movegen::MoveGen,
    search::{Search, SearchLimits as SearchLimitsV1},
    search_v2::{SearchV2, SearchLimits as SearchLimitsV2},
    search_v3::{SearchV3, SearchLimits as SearchLimitsV3},
    search_v4::{SearchV4, SearchLimits as SearchLimitsV4},
    search_mcts::{MCTSSearch, MCTSLimits},
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineType {
    V1,
    V2,
    V3,
    V4,
    MCTS,
}

impl EngineType {
    pub fn name(&self) -> &'static str {
        match self {
            EngineType::V1 => "V1.0 (Alpha-Beta)",
            EngineType::V2 => "V2.0 (Null Move + LMR)",
            EngineType::V3 => "V3.0 (Aspiration + Razoring)",
            EngineType::V4 => "V4.0 (Multi-Cut + IID)",
            EngineType::MCTS => "V5.0 (MCTS)",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(Debug)]
pub struct GameRecord {
    pub white: EngineType,
    pub black: EngineType,
    pub result: GameResult,
    pub moves: usize,
    pub opening_moves: Vec<String>,
}

pub struct Tournament {
    results: Vec<GameRecord>,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Play a match between two engines
    pub fn play_match(
        &mut self,
        white: EngineType,
        black: EngineType,
        depth: u8,
        time_per_move_ms: u64,
        max_moves: usize,
    ) -> GameResult {
        println!("\n┌─────────────────────────────────────────┐");
        println!("│ {} vs {}                    ", white.name(), black.name());
        println!("└─────────────────────────────────────────┘");

        let mut pos = Position::startpos();
        let mut move_count = 0;
        let mut opening_moves = Vec::new();
        let mut position_history = Vec::new();

        loop {
            move_count += 1;

            if move_count > max_moves {
                println!("Draw by move limit");
                let result = GameResult::Draw;
                self.results.push(GameRecord {
                    white,
                    black,
                    result,
                    moves: move_count,
                    opening_moves: opening_moves.clone(),
                });
                return result;
            }

            // Check for threefold repetition
            let current_hash = pos.hash;
            position_history.push(current_hash);
            if position_history.iter().filter(|&&h| h == current_hash).count() >= 3 {
                println!("Draw by threefold repetition");
                let result = GameResult::Draw;
                self.results.push(GameRecord {
                    white,
                    black,
                    result,
                    moves: move_count,
                    opening_moves: opening_moves.clone(),
                });
                return result;
            }

            // Check for stalemate/checkmate
            let legal_moves = MoveGen::generate_legal_moves(&pos);
            if legal_moves.is_empty() {
                let result = if pos.is_check() {
                    // Checkmate
                    if pos.side_to_move == crate::engine::position::Color::White {
                        println!("Black wins by checkmate!");
                        GameResult::BlackWin
                    } else {
                        println!("White wins by checkmate!");
                        GameResult::WhiteWin
                    }
                } else {
                    // Stalemate
                    println!("Draw by stalemate");
                    GameResult::Draw
                };

                self.results.push(GameRecord {
                    white,
                    black,
                    result,
                    moves: move_count,
                    opening_moves: opening_moves.clone(),
                });
                return result;
            }

            // Get move from appropriate engine
            let engine = if pos.side_to_move == crate::engine::position::Color::White {
                white
            } else {
                black
            };

            let best_move = Self::get_engine_move(&pos, engine, depth, time_per_move_ms);

            if let Some(mv) = best_move {
                if move_count <= 10 {
                    opening_moves.push(mv.to_uci());
                }

                print!("{}. {} ", move_count, mv.to_uci());
                if pos.make_move(mv).is_err() {
                    println!("\nFailed to make move");
                    let result = if pos.side_to_move == crate::engine::position::Color::White {
                        GameResult::BlackWin
                    } else {
                        GameResult::WhiteWin
                    };
                    self.results.push(GameRecord {
                        white,
                        black,
                        result,
                        moves: move_count,
                        opening_moves: opening_moves.clone(),
                    });
                    return result;
                }

                if move_count % 10 == 0 {
                    println!();
                }
            } else {
                println!("\nEngine failed to find a move");
                let result = if pos.side_to_move == crate::engine::position::Color::White {
                    GameResult::BlackWin
                } else {
                    GameResult::WhiteWin
                };

                self.results.push(GameRecord {
                    white,
                    black,
                    result,
                    moves: move_count,
                    opening_moves: opening_moves.clone(),
                });
                return result;
            }
        }
    }

    fn get_engine_move(
        pos: &Position,
        engine: EngineType,
        depth: u8,
        time_ms: u64,
    ) -> Option<crate::engine::movegen::Move> {
        match engine {
            EngineType::V1 => {
                let mut search = Search::new(64);
                let limits = SearchLimitsV1 {
                    max_depth: Some(depth),
                    max_time: Some(Duration::from_millis(time_ms)),
                    ..Default::default()
                };
                search.search(pos, limits)
            }
            EngineType::V2 => {
                let mut search = SearchV2::new(64);
                let limits = SearchLimitsV2 {
                    max_depth: Some(depth),
                    max_time: Some(Duration::from_millis(time_ms)),
                    ..Default::default()
                };
                search.search(pos, limits)
            }
            EngineType::V3 => {
                let mut search = SearchV3::new(64);
                let limits = SearchLimitsV3 {
                    max_depth: Some(depth),
                    max_time: Some(Duration::from_millis(time_ms)),
                    ..Default::default()
                };
                search.search(pos, limits)
            }
            EngineType::V4 => {
                let mut search = SearchV4::new(64);
                let limits = SearchLimitsV4 {
                    max_depth: Some(depth),
                    max_time: Some(Duration::from_millis(time_ms)),
                    ..Default::default()
                };
                search.search(pos, limits)
            }
            EngineType::MCTS => {
                let mut search = MCTSSearch::new();
                let limits = MCTSLimits {
                    max_iterations: Some(10000),
                    max_time: Some(Duration::from_millis(time_ms)),
                };
                search.search(pos, limits)
            }
        }
    }

    /// Run a full round-robin tournament
    pub fn run_round_robin(&mut self, engines: &[EngineType], games_per_pairing: usize, depth: u8, time_ms: u64) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║       Round-Robin Tournament               ║");
        println!("╚════════════════════════════════════════════╝");
        println!("Engines: {}", engines.len());
        println!("Games per pairing: {}", games_per_pairing);
        println!("Depth: {}", depth);
        println!("Time per move: {}ms\n", time_ms);

        let total_pairings = engines.len() * (engines.len() - 1);
        let total_games = total_pairings * games_per_pairing;
        let mut game_num = 0;

        for i in 0..engines.len() {
            for j in 0..engines.len() {
                if i == j {
                    continue;
                }

                for game in 0..games_per_pairing {
                    game_num += 1;
                    println!("\n[Game {}/{}]", game_num, total_games);

                    self.play_match(engines[i], engines[j], depth, time_ms, 200);
                }
            }
        }

        self.print_standings(engines);
    }

    /// Calculate and print tournament standings
    pub fn print_standings(&self, engines: &[EngineType]) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                  Tournament Standings                      ║");
        println!("╠════════════════════════════════════════════════════════════╣");

        let mut scores: std::collections::HashMap<EngineType, (f32, usize, usize, usize)> =
            std::collections::HashMap::new();

        // Initialize scores
        for &engine in engines {
            scores.insert(engine, (0.0, 0, 0, 0)); // (points, wins, draws, losses)
        }

        // Calculate scores
        for record in &self.results {
            let (white_pts, black_pts) = match record.result {
                GameResult::WhiteWin => (1.0, 0.0),
                GameResult::BlackWin => (0.0, 1.0),
                GameResult::Draw => (0.5, 0.5),
            };

            // Update white
            let white_entry = scores.get_mut(&record.white).unwrap();
            white_entry.0 += white_pts;
            match record.result {
                GameResult::WhiteWin => white_entry.1 += 1,
                GameResult::Draw => white_entry.2 += 1,
                GameResult::BlackWin => white_entry.3 += 1,
            }

            // Update black
            let black_entry = scores.get_mut(&record.black).unwrap();
            black_entry.0 += black_pts;
            match record.result {
                GameResult::BlackWin => black_entry.1 += 1,
                GameResult::Draw => black_entry.2 += 1,
                GameResult::WhiteWin => black_entry.3 += 1,
            }
        }

        // Sort by points
        let mut standings: Vec<_> = scores.iter().collect();
        standings.sort_by(|a, b| b.1 .0.partial_cmp(&a.1 .0).unwrap());

        // Print standings
        println!("║ Rank │ Engine                    │ Score  │ W-D-L      ║");
        println!("╠══════╪═══════════════════════════╪════════╪════════════╣");

        for (rank, (engine, (points, wins, draws, losses))) in standings.iter().enumerate() {
            let games = wins + draws + losses;
            println!(
                "║  {:2}  │ {:25} │ {:4.1}/{:2} │ {}-{}-{}    ║",
                rank + 1,
                engine.name(),
                points,
                games,
                wins,
                draws,
                losses
            );
        }

        println!("╚══════╧═══════════════════════════╧════════╧════════════╝");
    }

    /// Calculate Elo ratings based on results
    pub fn calculate_elo_ratings(&self, engines: &[EngineType]) -> std::collections::HashMap<EngineType, i32> {
        const K_FACTOR: f64 = 32.0;
        let mut ratings: std::collections::HashMap<EngineType, i32> = std::collections::HashMap::new();

        // Initialize all engines at 1500
        for &engine in engines {
            ratings.insert(engine, 1500);
        }

        // Process each game
        for record in &self.results {
            let white_rating = *ratings.get(&record.white).unwrap() as f64;
            let black_rating = *ratings.get(&record.black).unwrap() as f64;

            // Expected scores
            let white_expected = 1.0 / (1.0 + 10.0_f64.powf((black_rating - white_rating) / 400.0));
            let black_expected = 1.0 - white_expected;

            // Actual scores
            let (white_score, black_score) = match record.result {
                GameResult::WhiteWin => (1.0, 0.0),
                GameResult::BlackWin => (0.0, 1.0),
                GameResult::Draw => (0.5, 0.5),
            };

            // Update ratings
            let white_new = white_rating + K_FACTOR * (white_score - white_expected);
            let black_new = black_rating + K_FACTOR * (black_score - black_expected);

            ratings.insert(record.white, white_new.round() as i32);
            ratings.insert(record.black, black_new.round() as i32);
        }

        ratings
    }

    pub fn print_elo_ratings(&self, engines: &[EngineType]) {
        let ratings = self.calculate_elo_ratings(engines);

        println!("\n╔════════════════════════════════════════════╗");
        println!("║           Estimated Elo Ratings            ║");
        println!("╠════════════════════════════════════════════╣");

        let mut sorted_ratings: Vec<_> = ratings.iter().collect();
        sorted_ratings.sort_by(|a, b| b.1.cmp(a.1));

        for (engine, rating) in sorted_ratings {
            println!("║ {:25} │ {:4} Elo      ║", engine.name(), rating);
        }

        println!("╚════════════════════════════════════════════╝");
    }
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}
