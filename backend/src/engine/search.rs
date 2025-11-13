use crate::engine::evaluation::{Evaluator, INFINITY, MATE_SCORE};
use crate::engine::movegen::{Move, MoveGen, MoveList, MoveType};
use crate::engine::position::{Position, PieceType};
use crate::engine::transposition::{NodeType, TranspositionTable};
use std::time::{Duration, Instant};

const MAX_DEPTH: u8 = 64;
const MAX_KILLER_MOVES: usize = 2;
const ASPIRATION_WINDOW: i32 = 50;

pub struct SearchStats {
    pub nodes: u64,
    pub qnodes: u64,
    pub tt_hits: u64,
    pub time_elapsed: Duration,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            nodes: 0,
            qnodes: 0,
            tt_hits: 0,
            time_elapsed: Duration::ZERO,
        }
    }

    pub fn nps(&self) -> u64 {
        let secs = self.time_elapsed.as_secs_f64();
        if secs > 0.0 {
            ((self.nodes + self.qnodes) as f64 / secs) as u64
        } else {
            0
        }
    }
}

pub struct SearchLimits {
    pub max_depth: Option<u8>,
    pub max_time: Option<Duration>,
    pub max_nodes: Option<u64>,
    pub infinite: bool,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            max_depth: Some(10),
            max_time: None,
            max_nodes: None,
            infinite: false,
        }
    }
}

pub struct Search {
    tt: TranspositionTable,
    killer_moves: [[Option<Move>; MAX_KILLER_MOVES]; MAX_DEPTH as usize],
    history: [[i32; 64]; 64], // [from][to]
    stats: SearchStats,
    start_time: Instant,
    limits: SearchLimits,
    stop_search: bool,
}

impl Search {
    pub fn new(tt_size_mb: usize) -> Self {
        Self {
            tt: TranspositionTable::new(tt_size_mb),
            killer_moves: [[None; MAX_KILLER_MOVES]; MAX_DEPTH as usize],
            history: [[0; 64]; 64],
            stats: SearchStats::new(),
            start_time: Instant::now(),
            limits: SearchLimits::default(),
            stop_search: false,
        }
    }

    /// Main search function with iterative deepening
    pub fn search(&mut self, pos: &Position, limits: SearchLimits) -> Option<Move> {
        self.limits = limits;
        self.start_time = Instant::now();
        self.stats = SearchStats::new();
        self.stop_search = false;
        self.tt.new_search();

        let max_depth = self.limits.max_depth.unwrap_or(MAX_DEPTH).min(MAX_DEPTH);
        let mut best_move = None;
        let mut prev_score = 0;

        // Iterative deepening
        for depth in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            // Aspiration windows for depths > 3
            let (mut alpha, mut beta) = if depth > 3 {
                (
                    prev_score - ASPIRATION_WINDOW,
                    prev_score + ASPIRATION_WINDOW,
                )
            } else {
                (-INFINITY, INFINITY)
            };

            let mut score;
            let mut search_count = 0;

            loop {
                score = self.negamax(pos, depth, alpha, beta, 0, true);

                // Re-search if outside window
                if score <= alpha {
                    alpha = -INFINITY;
                    search_count += 1;
                } else if score >= beta {
                    beta = INFINITY;
                    search_count += 1;
                } else {
                    break;
                }

                if search_count > 2 || self.should_stop() {
                    break;
                }
            }

            if self.should_stop() {
                break;
            }

            prev_score = score;

            // Get best move from TT
            if let Some(entry) = self.tt.probe(pos.hash) {
                if let Some(mv) = entry.best_move {
                    best_move = Some(mv);
                }
            }

            // Print info
            self.print_info(depth, score, best_move);
        }

        self.stats.time_elapsed = self.start_time.elapsed();
        best_move
    }

    /// Negamax with alpha-beta pruning
    fn negamax(&mut self, pos: &Position, depth: u8, mut alpha: i32, beta: i32, ply: u8, do_null: bool) -> i32 {
        if self.should_stop() {
            return 0;
        }

        // Check for draw by repetition or 50-move rule
        if pos.halfmove_clock >= 100 {
            return 0;
        }

        // Probe transposition table
        let tt_entry = self.tt.probe(pos.hash);
        if let Some(entry) = tt_entry {
            if entry.depth >= depth {
                self.stats.tt_hits += 1;
                match entry.node_type {
                    NodeType::Exact => return entry.score,
                    NodeType::LowerBound => {
                        if entry.score >= beta {
                            return entry.score;
                        }
                    }
                    NodeType::UpperBound => {
                        if entry.score <= alpha {
                            return entry.score;
                        }
                    }
                }
            }
        }

        // Drop into quiescence search at leaf nodes
        if depth == 0 {
            return self.quiescence(pos, alpha, beta);
        }

        self.stats.nodes += 1;

        let in_check = pos.is_check();

        // Null move pruning (if not in check and not in endgame)
        if do_null && !in_check && depth >= 3 && !self.is_endgame(pos) {
            let mut null_pos = pos.clone();
            null_pos.side_to_move = !null_pos.side_to_move;
            null_pos.en_passant_square = None;
            null_pos.update_hash();

            let r = 2; // Reduction depth
            let null_score = -self.negamax(&null_pos, depth.saturating_sub(1 + r), -beta, -beta + 1, ply + 1, false);

            if null_score >= beta {
                return beta;
            }
        }

        // Generate and order moves
        let mut moves = MoveGen::generate_legal_moves(pos);
        if moves.is_empty() {
            if in_check {
                return -MATE_SCORE + ply as i32; // Checkmate
            } else {
                return 0; // Stalemate
            }
        }

        self.order_moves(pos, &mut moves, ply, tt_entry);

        let mut best_score = -INFINITY;
        let mut best_move = None;
        let mut move_count = 0;

        for mv in moves.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
                continue;
            }

            move_count += 1;
            let mut score;

            // Principal Variation Search (PVS)
            if move_count == 1 {
                score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, true);
            } else {
                // Late Move Reduction (LMR)
                let reduction = if move_count > 4 && depth > 2 && !mv.is_capture() && !in_check {
                    1
                } else {
                    0
                };

                // Search with null window
                score = -self.negamax(&new_pos, depth - 1 - reduction, -alpha - 1, -alpha, ply + 1, true);

                // Re-search if needed
                if score > alpha && (score < beta || reduction > 0) {
                    score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, true);
                }
            }

            if score > best_score {
                best_score = score;
                best_move = Some(mv);

                if score > alpha {
                    alpha = score;

                    if alpha >= beta {
                        // Beta cutoff
                        self.update_history(mv, depth);
                        self.update_killers(mv, ply);
                        break;
                    }
                }
            }
        }

        // Store in transposition table
        let node_type = if best_score >= beta {
            NodeType::LowerBound
        } else if best_score <= alpha {
            NodeType::UpperBound
        } else {
            NodeType::Exact
        };

        self.tt.store(pos.hash, depth, best_score, node_type, best_move);

        best_score
    }

    /// Quiescence search to avoid horizon effect
    fn quiescence(&mut self, pos: &Position, mut alpha: i32, beta: i32) -> i32 {
        if self.should_stop() {
            return 0;
        }

        self.stats.qnodes += 1;

        // Stand pat
        let stand_pat = Evaluator::evaluate(pos);

        if stand_pat >= beta {
            return beta;
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        // Delta pruning
        const QUEEN_VALUE: i32 = 900;
        if stand_pat + QUEEN_VALUE + 200 < alpha {
            return alpha;
        }

        // Generate only captures and promotions
        let moves = self.generate_tactical_moves(pos);

        for mv in moves.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
                continue;
            }

            // SEE pruning (simplified)
            if !self.see_ge(&new_pos, mv, 0) {
                continue;
            }

            let score = -self.quiescence(&new_pos, -beta, -alpha);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    /// Generate tactical moves (captures and promotions)
    fn generate_tactical_moves(&self, pos: &Position) -> MoveList {
        let all_moves = MoveGen::generate_legal_moves(pos);
        let mut tactical = MoveList::new();

        for mv in all_moves.into_iter() {
            if mv.is_capture() || mv.is_promotion() {
                tactical.push(mv);
            }
        }

        tactical
    }

    /// Static Exchange Evaluation (simplified)
    fn see_ge(&self, _pos: &Position, _mv: Move, _threshold: i32) -> bool {
        // Simplified: assume all captures are good
        // Full implementation would calculate the value exchange
        true
    }

    /// Order moves for better alpha-beta pruning
    fn order_moves(&self, pos: &Position, moves: &mut MoveList, ply: u8, tt_entry: Option<crate::engine::transposition::TTEntry>) {
        // This is a simplified move ordering
        // Full implementation would sort the move list

        // Priority:
        // 1. TT move (from transposition table)
        // 2. Captures (MVV-LVA)
        // 3. Killer moves
        // 4. History heuristic
        // 5. Other moves

        // For now, we just rely on the natural order plus history
        // A full implementation would sort the MoveList
    }

    /// Update killer moves
    fn update_killers(&mut self, mv: Move, ply: u8) {
        if mv.is_capture() {
            return;
        }

        let ply_idx = (ply as usize).min(MAX_DEPTH as usize - 1);

        // Shift killers
        if Some(mv) != self.killer_moves[ply_idx][0] {
            self.killer_moves[ply_idx][1] = self.killer_moves[ply_idx][0];
            self.killer_moves[ply_idx][0] = Some(mv);
        }
    }

    /// Update history heuristic
    fn update_history(&mut self, mv: Move, depth: u8) {
        if mv.is_capture() {
            return;
        }

        let bonus = (depth as i32).pow(2);
        self.history[mv.from() as usize][mv.to() as usize] += bonus;

        // Prevent overflow
        if self.history[mv.from() as usize][mv.to() as usize] > 10000 {
            self.age_history();
        }
    }

    /// Age history table
    fn age_history(&mut self) {
        for i in 0..64 {
            for j in 0..64 {
                self.history[i][j] /= 2;
            }
        }
    }

    fn is_endgame(&self, pos: &Position) -> bool {
        let queens = pos.piece_bb(PieceType::Queen, crate::engine::position::Color::White).count()
            + pos.piece_bb(PieceType::Queen, crate::engine::position::Color::Black).count();
        queens == 0
    }

    fn should_stop(&self) -> bool {
        if self.stop_search {
            return true;
        }

        // Check time limit
        if let Some(max_time) = self.limits.max_time {
            if self.start_time.elapsed() >= max_time {
                return true;
            }
        }

        // Check node limit
        if let Some(max_nodes) = self.limits.max_nodes {
            if self.stats.nodes + self.stats.qnodes >= max_nodes {
                return true;
            }
        }

        false
    }

    fn print_info(&self, depth: u8, score: i32, best_move: Option<Move>) {
        let elapsed = self.start_time.elapsed();
        let nodes = self.stats.nodes + self.stats.qnodes;
        let nps = if elapsed.as_secs_f64() > 0.0 {
            (nodes as f64 / elapsed.as_secs_f64()) as u64
        } else {
            0
        };

        println!(
            "info depth {} score cp {} nodes {} nps {} time {} pv {}",
            depth,
            score,
            nodes,
            nps,
            elapsed.as_millis(),
            best_move.map(|m| m.to_uci()).unwrap_or_else(|| "none".to_string())
        );
    }

    pub fn stop(&mut self) {
        self.stop_search = true;
    }

    pub fn clear(&mut self) {
        self.tt.clear();
        self.killer_moves = [[None; MAX_KILLER_MOVES]; MAX_DEPTH as usize];
        self.history = [[0; 64]; 64];
    }

    pub fn get_stats(&self) -> &SearchStats {
        &self.stats
    }
}

impl Default for Search {
    fn default() -> Self {
        Self::new(64) // 64 MB TT
    }
}
