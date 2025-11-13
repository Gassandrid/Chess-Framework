use crate::engine::{
    movegen::MoveGen,
    position::Position,
    search::{Search, SearchLimits},
};
use std::time::Duration;

/// Elo rating system for engine tournaments
#[derive(Debug, Clone)]
pub struct EloRating {
    pub rating: f64,
    pub games_played: u32,
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
}

impl EloRating {
    pub fn new(initial_rating: f64) -> Self {
        Self {
            rating: initial_rating,
            games_played: 0,
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    /// Calculate expected score against opponent
    pub fn expected_score(&self, opponent_rating: f64) -> f64 {
        1.0 / (1.0 + 10.0_f64.powf((opponent_rating - self.rating) / 400.0))
    }

    /// Update rating after a game
    /// result: 1.0 for win, 0.5 for draw, 0.0 for loss
    pub fn update_rating(&mut self, opponent_rating: f64, result: f64, k_factor: f64) {
        let expected = self.expected_score(opponent_rating);
        self.rating += k_factor * (result - expected);
        self.games_played += 1;

        if result == 1.0 {
            self.wins += 1;
        } else if result == 0.0 {
            self.losses += 1;
        } else {
            self.draws += 1;
        }
    }

    pub fn win_rate(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.wins as f64 + 0.5 * self.draws as f64) / self.games_played as f64
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

/// Engine configuration for testing
#[derive(Clone)]
pub struct EngineConfig {
    pub name: String,
    pub search_depth: u8,
    pub time_per_move: Duration,
    pub tt_size_mb: usize,
}

impl EngineConfig {
    pub fn new(name: String, search_depth: u8) -> Self {
        Self {
            name,
            search_depth,
            time_per_move: Duration::from_secs(1),
            tt_size_mb: 64,
        }
    }
}

/// Match between two engines
pub struct EngineMatch {
    pub white_config: EngineConfig,
    pub black_config: EngineConfig,
    pub games: Vec<GameResult>,
}

impl EngineMatch {
    pub fn new(white_config: EngineConfig, black_config: EngineConfig) -> Self {
        Self {
            white_config,
            black_config,
            games: Vec::new(),
        }
    }

    /// Play a single game
    pub fn play_game(&mut self, max_moves: u32) -> GameResult {
        let mut pos = Position::startpos();
        let mut white_search = Search::new(self.white_config.tt_size_mb);
        let mut black_search = Search::new(self.black_config.tt_size_mb);

        let mut move_count = 0;

        loop {
            // Check for draw conditions
            if pos.halfmove_clock >= 100 || move_count >= max_moves {
                self.games.push(GameResult::Draw);
                return GameResult::Draw;
            }

            // Check if game is over
            let moves = MoveGen::generate_legal_moves(&pos);
            if moves.is_empty() {
                let result = if pos.is_check() {
                    // Checkmate
                    if pos.side_to_move == crate::engine::position::Color::White {
                        GameResult::BlackWin
                    } else {
                        GameResult::WhiteWin
                    }
                } else {
                    // Stalemate
                    GameResult::Draw
                };
                self.games.push(result);
                return result;
            }

            // Get move from appropriate engine
            let search = if pos.side_to_move == crate::engine::position::Color::White {
                &mut white_search
            } else {
                &mut black_search
            };

            let limits = SearchLimits {
                max_depth: Some(if pos.side_to_move == crate::engine::position::Color::White {
                    self.white_config.search_depth
                } else {
                    self.black_config.search_depth
                }),
                max_time: Some(if pos.side_to_move == crate::engine::position::Color::White {
                    self.white_config.time_per_move
                } else {
                    self.black_config.time_per_move
                }),
                ..Default::default()
            };

            if let Some(mv) = search.search(&pos, limits) {
                if pos.make_move(mv).is_err() {
                    // Illegal move - opponent wins
                    let result = if pos.side_to_move == crate::engine::position::Color::White {
                        GameResult::BlackWin
                    } else {
                        GameResult::WhiteWin
                    };
                    self.games.push(result);
                    return result;
                }
            } else {
                // No move found - opponent wins
                let result = if pos.side_to_move == crate::engine::position::Color::White {
                    GameResult::BlackWin
                } else {
                    GameResult::WhiteWin
                };
                self.games.push(result);
                return result;
            }

            move_count += 1;
        }
    }

    /// Play multiple games, alternating colors
    pub fn play_match(&mut self, num_games: u32, max_moves: u32) -> MatchStats {
        let mut white_wins = 0;
        let mut black_wins = 0;
        let mut draws = 0;

        for game_num in 0..num_games {
            println!("Playing game {}/{}...", game_num + 1, num_games);

            let result = if game_num % 2 == 0 {
                // Normal colors
                self.play_game(max_moves)
            } else {
                // Swap colors
                std::mem::swap(&mut self.white_config, &mut self.black_config);
                let result = self.play_game(max_moves);
                std::mem::swap(&mut self.white_config, &mut self.black_config);

                // Flip result since colors were swapped
                match result {
                    GameResult::WhiteWin => GameResult::BlackWin,
                    GameResult::BlackWin => GameResult::WhiteWin,
                    GameResult::Draw => GameResult::Draw,
                }
            };

            match result {
                GameResult::WhiteWin => white_wins += 1,
                GameResult::BlackWin => black_wins += 1,
                GameResult::Draw => draws += 1,
            }

            println!("Result: {:?}", result);
        }

        MatchStats {
            white_name: self.white_config.name.clone(),
            black_name: self.black_config.name.clone(),
            games_played: num_games,
            white_wins,
            black_wins,
            draws,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchStats {
    pub white_name: String,
    pub black_name: String,
    pub games_played: u32,
    pub white_wins: u32,
    pub black_wins: u32,
    pub draws: u32,
}

impl MatchStats {
    pub fn white_score(&self) -> f64 {
        self.white_wins as f64 + 0.5 * self.draws as f64
    }

    pub fn black_score(&self) -> f64 {
        self.black_wins as f64 + 0.5 * self.draws as f64
    }

    pub fn white_percentage(&self) -> f64 {
        if self.games_played == 0 {
            0.0
        } else {
            (self.white_score() / self.games_played as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== Match Summary ===");
        println!("{} vs {}", self.white_name, self.black_name);
        println!("Games played: {}", self.games_played);
        println!(
            "{} wins: {} ({:.1}%)",
            self.white_name,
            self.white_wins,
            self.white_percentage()
        );
        println!(
            "{} wins: {} ({:.1}%)",
            self.black_name,
            self.black_wins,
            100.0 - self.white_percentage()
        );
        println!("Draws: {}", self.draws);
        println!(
            "Score: {:.1} - {:.1}",
            self.white_score(),
            self.black_score()
        );
    }
}

/// Tournament system for testing multiple engine versions
pub struct Tournament {
    pub engines: Vec<(EngineConfig, EloRating)>,
    pub games_per_match: u32,
    pub max_moves: u32,
}

impl Tournament {
    pub fn new(games_per_match: u32, max_moves: u32) -> Self {
        Self {
            engines: Vec::new(),
            games_per_match,
            max_moves,
        }
    }

    pub fn add_engine(&mut self, config: EngineConfig, initial_rating: f64) {
        self.engines.push((config, EloRating::new(initial_rating)));
    }

    /// Run a round-robin tournament
    pub fn run_round_robin(&mut self) {
        let num_engines = self.engines.len();
        if num_engines < 2 {
            println!("Need at least 2 engines for a tournament");
            return;
        }

        println!("\n=== Starting Round-Robin Tournament ===");
        println!("Engines: {}", num_engines);
        println!("Games per match: {}", self.games_per_match);
        println!("Max moves per game: {}\n", self.max_moves);

        // Play each pair
        for i in 0..num_engines {
            for j in (i + 1)..num_engines {
                let engine1 = &self.engines[i].0;
                let engine2 = &self.engines[j].0;

                println!(
                    "\nMatch: {} vs {}",
                    engine1.name, engine2.name
                );

                let mut match_obj =
                    EngineMatch::new(engine1.clone(), engine2.clone());
                let stats = match_obj.play_match(self.games_per_match, self.max_moves);

                stats.print_summary();

                // Update Elo ratings
                let rating1 = self.engines[i].1.rating;
                let rating2 = self.engines[j].1.rating;

                let score1 = stats.white_score() / self.games_per_match as f64;
                let score2 = stats.black_score() / self.games_per_match as f64;

                self.engines[i].1.update_rating(rating2, score1, 32.0);
                self.engines[j].1.update_rating(rating1, score2, 32.0);
            }
        }

        self.print_standings();
    }

    pub fn print_standings(&self) {
        println!("\n=== Tournament Standings ===");
        println!("{:<20} {:>8} {:>8} {:>5} {:>5} {:>5} {:>6}",
                 "Engine", "Rating", "Games", "W", "L", "D", "Win%");
        println!("{:-<70}", "");

        let mut sorted_engines = self.engines.clone();
        sorted_engines.sort_by(|a, b| b.1.rating.partial_cmp(&a.1.rating).unwrap());

        for (config, elo) in sorted_engines {
            println!(
                "{:<20} {:>8.0} {:>8} {:>5} {:>5} {:>5} {:>6.1}%",
                config.name,
                elo.rating,
                elo.games_played,
                elo.wins,
                elo.losses,
                elo.draws,
                elo.win_rate() * 100.0
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elo_calculation() {
        let mut player1 = EloRating::new(1500.0);
        let player2_rating = 1500.0;

        // Win should increase rating
        player1.update_rating(player2_rating, 1.0, 32.0);
        assert!(player1.rating > 1500.0);

        // Loss should decrease rating
        let mut player3 = EloRating::new(1500.0);
        player3.update_rating(player2_rating, 0.0, 32.0);
        assert!(player3.rating < 1500.0);
    }
}
