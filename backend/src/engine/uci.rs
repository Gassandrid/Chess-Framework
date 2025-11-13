use crate::engine::position::Position;
use crate::engine::search::{Search, SearchLimits};
use std::io::{self, BufRead};
use std::time::Duration;

pub struct UciEngine {
    search: Search,
    position: Position,
}

impl UciEngine {
    pub fn new() -> Self {
        Self {
            search: Search::default(),
            position: Position::startpos(),
        }
    }

    pub fn run(&mut self) {
        println!("Chess Engine v1.0");

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(command) = line {
                let trimmed = command.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if !self.handle_command(trimmed) {
                    break;
                }
            }
        }
    }

    fn handle_command(&mut self, command: &str) -> bool {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return true;
        }

        match parts[0] {
            "uci" => self.handle_uci(),
            "isready" => println!("readyok"),
            "ucinewgame" => self.handle_new_game(),
            "position" => self.handle_position(&parts[1..]),
            "go" => self.handle_go(&parts[1..]),
            "stop" => self.search.stop(),
            "quit" => return false,
            "d" | "display" => println!("{:?}", self.position),
            "eval" => {
                let eval = crate::engine::evaluation::Evaluator::evaluate(&self.position);
                println!("Evaluation: {} centipawns", eval);
            }
            _ => println!("Unknown command: {}", command),
        }

        true
    }

    fn handle_uci(&self) {
        println!("id name ChessEngine");
        println!("id author Rust Chess Framework");
        println!("option name Hash type spin default 64 min 1 max 4096");
        println!("uciok");
    }

    fn handle_new_game(&mut self) {
        self.search.clear();
        self.position = Position::startpos();
    }

    fn handle_position(&mut self, args: &[&str]) {
        if args.is_empty() {
            return;
        }

        let mut move_index = None;

        match args[0] {
            "startpos" => {
                self.position = Position::startpos();
                move_index = Some(1);
            }
            "fen" => {
                // Find where moves start
                for (i, &arg) in args.iter().enumerate() {
                    if arg == "moves" {
                        move_index = Some(i);
                        break;
                    }
                }

                let fen_end = move_index.unwrap_or(args.len());
                let fen = args[1..fen_end].join(" ");

                match Position::from_fen(&fen) {
                    Ok(pos) => self.position = pos,
                    Err(e) => println!("Error parsing FEN: {}", e),
                }
            }
            _ => {
                println!("Invalid position command");
                return;
            }
        }

        // Apply moves if any
        if let Some(idx) = move_index {
            if idx < args.len() && args[idx] == "moves" {
                for &move_str in &args[idx + 1..] {
                    if let Some(mv) = crate::engine::movegen::Move::from_uci(move_str, &self.position) {
                        if self.position.make_move(mv).is_err() {
                            println!("Illegal move: {}", move_str);
                            break;
                        }
                    } else {
                        println!("Invalid move: {}", move_str);
                        break;
                    }
                }
            }
        }
    }

    fn handle_go(&mut self, args: &[&str]) {
        let mut limits = SearchLimits::default();
        let mut i = 0;

        while i < args.len() {
            match args[i] {
                "infinite" => {
                    limits.infinite = true;
                    limits.max_depth = None;
                    limits.max_time = None;
                }
                "depth" => {
                    if i + 1 < args.len() {
                        if let Ok(depth) = args[i + 1].parse::<u8>() {
                            limits.max_depth = Some(depth);
                        }
                        i += 1;
                    }
                }
                "movetime" => {
                    if i + 1 < args.len() {
                        if let Ok(ms) = args[i + 1].parse::<u64>() {
                            limits.max_time = Some(Duration::from_millis(ms));
                        }
                        i += 1;
                    }
                }
                "nodes" => {
                    if i + 1 < args.len() {
                        if let Ok(nodes) = args[i + 1].parse::<u64>() {
                            limits.max_nodes = Some(nodes);
                        }
                        i += 1;
                    }
                }
                "wtime" | "btime" | "winc" | "binc" | "movestogo" => {
                    // Skip time control parameters for now
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }

        if let Some(best_move) = self.search.search(&self.position, limits) {
            println!("bestmove {}", best_move.to_uci());
        } else {
            println!("bestmove 0000");
        }
    }
}

impl Default for UciEngine {
    fn default() -> Self {
        Self::new()
    }
}
