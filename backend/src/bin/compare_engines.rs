use chess_engine_api::engine::{
    movegen::MoveGen,
    position::{Color, Position},
    search::Search,
    search_v2::SearchV2,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

struct MatchStats {
    v1_wins: u32,
    v2_wins: u32,
    draws: u32,
    games_played: u32,
}

impl MatchStats {
    fn new() -> Self {
        Self {
            v1_wins: 0,
            v2_wins: 0,
            draws: 0,
            games_played: 0,
        }
    }

    fn v1_score(&self) -> f64 {
        self.v1_wins as f64 + 0.5 * self.draws as f64
    }

    fn v2_score(&self) -> f64 {
        self.v2_wins as f64 + 0.5 * self.draws as f64
    }

    fn v1_percentage(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.v1_score() / self.games_played as f64) * 100.0
        }
    }

    fn print_summary(&self) {
        println!("\n=== Match Summary ===");
        println!("Games played: {}", self.games_played);
        println!("V1.0 (Baseline) wins: {} ({:.1}%)", self.v1_wins, self.v1_percentage());
        println!("V2.0 (Improved) wins: {} ({:.1}%)", self.v2_wins, 100.0 - self.v1_percentage());
        println!("Draws: {}", self.draws);
        println!("Score: {:.1} - {:.1}", self.v1_score(), self.v2_score());

        if self.v2_score() > self.v1_score() {
            let improvement = ((self.v2_score() - self.v1_score()) / self.games_played as f64) * 100.0;
            println!("\n🎉 V2.0 is BETTER! Improvement: +{:.1}%", improvement);
        } else if self.v1_score() > self.v2_score() {
            println!("\n⚠️  V1.0 is still better");
        } else {
            println!("\n= They are equal");
        }
    }
}

fn play_game(
    use_v2_as_white: bool,
    depth: u8,
    max_moves: u32,
) -> GameResult {
    let mut pos = Position::startpos();
    let mut v1_search = Search::new(64);
    let mut v2_search = SearchV2::new(64);

    let mut move_count = 0;
    let mut position_history = Vec::new();

    loop {
        // Check for threefold repetition
        let current_hash = pos.hash;
        let repetition_count = position_history.iter().filter(|&&h| h == current_hash).count();
        if repetition_count >= 2 {
            return GameResult::Draw;
        }
        position_history.push(current_hash);

        if pos.halfmove_clock >= 100 || move_count >= max_moves {
            return GameResult::Draw;
        }

        let moves = MoveGen::generate_legal_moves(&pos);
        if moves.is_empty() {
            let result = if pos.is_check() {
                if pos.side_to_move == Color::White {
                    GameResult::BlackWin
                } else {
                    GameResult::WhiteWin
                }
            } else {
                GameResult::Draw
            };
            return result;
        }

        // Determine which engine to use
        let use_v2 = (pos.side_to_move == Color::White) == use_v2_as_white;

        let mv = if use_v2 {
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
                let result = if pos.side_to_move == Color::White {
                    GameResult::BlackWin
                } else {
                    GameResult::WhiteWin
                };
                return result;
            }
        } else {
            let result = if pos.side_to_move == Color::White {
                GameResult::BlackWin
            } else {
                GameResult::WhiteWin
            };
            return result;
        }

        move_count += 1;
    }
}

fn main() {
    println!("Chess Engine Comparison: V1.0 (Baseline) vs V2.0 (Improved)");
    println!("===========================================================\n");

    println!("V2.0 Improvements:");
    println!("  - Better evaluation (improved PST, rook placement, pawn structure)");
    println!("  - Proper MVV-LVA move ordering for captures");
    println!("  - Check extensions for tactical awareness");
    println!("  - Improved null move pruning (adaptive R)");
    println!("  - Better LMR conditions");
    println!("  - Rook on open file bonus");
    println!("  - Connected pawns bonus");
    println!("  - Stronger king safety evaluation");
    println!();

    let num_games = 10;
    let depth = 5;
    let max_moves = 150;

    println!("Configuration:");
    println!("  Games: {} (alternating colors)", num_games);
    println!("  Search depth: {}", depth);
    println!("  Max moves per game: {}", max_moves);
    println!();

    let mut stats = MatchStats::new();

    for game_num in 0..num_games {
        println!("Playing game {}/{}...", game_num + 1, num_games);

        // Alternate colors
        let use_v2_as_white = game_num % 2 == 0;
        let result = play_game(use_v2_as_white, depth, max_moves);

        // Convert result based on color assignment
        let final_result = if use_v2_as_white {
            match result {
                GameResult::WhiteWin => {
                    println!("  Result: V2.0 wins (as White)");
                    stats.v2_wins += 1;
                    result
                }
                GameResult::BlackWin => {
                    println!("  Result: V1.0 wins (as Black)");
                    stats.v1_wins += 1;
                    result
                }
                GameResult::Draw => {
                    println!("  Result: Draw");
                    stats.draws += 1;
                    result
                }
            }
        } else {
            match result {
                GameResult::WhiteWin => {
                    println!("  Result: V1.0 wins (as White)");
                    stats.v1_wins += 1;
                    result
                }
                GameResult::BlackWin => {
                    println!("  Result: V2.0 wins (as Black)");
                    stats.v2_wins += 1;
                    result
                }
                GameResult::Draw => {
                    println!("  Result: Draw");
                    stats.draws += 1;
                    result
                }
            }
        };

        stats.games_played += 1;

        // Print current score
        println!("  Current score: V1.0 {:.1} - {:.1} V2.0\n",
                 stats.v1_score(), stats.v2_score());
    }

    stats.print_summary();
}
