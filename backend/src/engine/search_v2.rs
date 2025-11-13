use crate::engine::evaluation_v2::{EvaluatorV2, INFINITY, MATE_SCORE};
use crate::engine::movegen::{Move, MoveGen, MoveType};
use crate::engine::position::Position;
use crate::engine::transposition::{NodeType, TranspositionTable};
use crate::engine::move_ordering::MoveOrdering;
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

pub struct SearchV2 {
    tt: TranspositionTable,
    killer_moves: [[Option<Move>; MAX_KILLER_MOVES]; MAX_DEPTH as usize],
    history: [[i32; 64]; 64],
    stats: SearchStats,
    start_time: Instant,
    limits: SearchLimits,
    stop_search: bool,
}

impl SearchV2 {
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

    pub fn search(&mut self, pos: &Position, limits: SearchLimits) -> Option<Move> {
        self.limits = limits;
        self.start_time = Instant::now();
        self.stats = SearchStats::new();
        self.stop_search = false;
        self.tt.new_search();

        let max_depth = self.limits.max_depth.unwrap_or(MAX_DEPTH).min(MAX_DEPTH);
        let mut best_move = None;
        let mut prev_score = 0;

        for depth in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            let (mut alpha, mut beta) = if depth > 3 {
                (prev_score - ASPIRATION_WINDOW, prev_score + ASPIRATION_WINDOW)
            } else {
                (-INFINITY, INFINITY)
            };

            let mut score;
            let mut search_count = 0;

            loop {
                score = self.negamax(pos, depth, alpha, beta, 0, true);

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

            if let Some(entry) = self.tt.probe(pos.hash) {
                if let Some(mv) = entry.best_move {
                    best_move = Some(mv);
                }
            }

            self.print_info(depth, score, best_move);
        }

        self.stats.time_elapsed = self.start_time.elapsed();
        best_move
    }

    fn negamax(&mut self, pos: &Position, depth: u8, mut alpha: i32, beta: i32, ply: u8, do_null: bool) -> i32 {
        if self.should_stop() {
            return 0;
        }

        if pos.halfmove_clock >= 100 {
            return 0;
        }

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

        // Check extension - search deeper when in check
        let in_check = pos.is_check();
        let adjusted_depth = if in_check && depth < MAX_DEPTH - 1 {
            depth + 1
        } else {
            depth
        };

        if adjusted_depth == 0 {
            return self.quiescence(pos, alpha, beta);
        }

        self.stats.nodes += 1;

        // Improved null move pruning with adaptive reduction
        if do_null && !in_check && depth >= 3 && !self.is_endgame(pos) {
            let mut null_pos = pos.clone();
            null_pos.side_to_move = !null_pos.side_to_move;
            null_pos.en_passant_square = None;
            null_pos.update_hash();

            // Adaptive null move reduction: R=3 for deeper searches
            let r = if depth >= 6 { 3 } else { 2 };
            let null_score = -self.negamax(&null_pos, depth.saturating_sub(1 + r), -beta, -beta + 1, ply + 1, false);

            if null_score >= beta {
                return beta;
            }
        }

        let mut moves: Vec<Move> = MoveGen::generate_legal_moves(pos).into_iter().collect();
        if moves.is_empty() {
            if in_check {
                return -MATE_SCORE + ply as i32;
            } else {
                return 0;
            }
        }

        // Use improved move ordering
        let ply_idx = (ply as usize).min(MAX_DEPTH as usize - 1);
        let tt_move = tt_entry.and_then(|e| e.best_move);
        MoveOrdering::order_moves(pos, &mut moves, tt_move, &self.killer_moves[ply_idx], &self.history);

        let mut best_score = -INFINITY;
        let mut best_move = None;
        let mut move_count = 0;

        for mv in moves {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
                continue;
            }

            move_count += 1;
            let mut score;

            if move_count == 1 {
                score = -self.negamax(&new_pos, adjusted_depth - 1, -beta, -alpha, ply + 1, true);
            } else {
                // Improved LMR conditions
                let reduction = if move_count > 4
                    && depth > 2
                    && !mv.is_capture()
                    && !mv.is_promotion()
                    && !in_check
                    && !new_pos.is_check()
                {
                    1
                } else {
                    0
                };

                score = -self.negamax(&new_pos, adjusted_depth - 1 - reduction, -alpha - 1, -alpha, ply + 1, true);

                if score > alpha && (score < beta || reduction > 0) {
                    score = -self.negamax(&new_pos, adjusted_depth - 1, -beta, -alpha, ply + 1, true);
                }
            }

            if score > best_score {
                best_score = score;
                best_move = Some(mv);

                if score > alpha {
                    alpha = score;

                    if alpha >= beta {
                        self.update_history(mv, depth);
                        self.update_killers(mv, ply);
                        break;
                    }
                }
            }
        }

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

    fn quiescence(&mut self, pos: &Position, mut alpha: i32, beta: i32) -> i32 {
        if self.should_stop() {
            return 0;
        }

        self.stats.qnodes += 1;

        let stand_pat = EvaluatorV2::evaluate(pos);

        if stand_pat >= beta {
            return beta;
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        const QUEEN_VALUE: i32 = 975;
        if stand_pat + QUEEN_VALUE + 200 < alpha {
            return alpha;
        }

        let mut moves: Vec<Move> = MoveGen::generate_legal_moves(pos)
            .into_iter()
            .filter(|mv| mv.is_capture() || mv.is_promotion())
            .collect();

        // Order captures by MVV-LVA
        MoveOrdering::order_moves(pos, &mut moves, None, &[None, None], &self.history);

        for mv in moves {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
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

    fn update_killers(&mut self, mv: Move, ply: u8) {
        if mv.is_capture() {
            return;
        }

        let ply_idx = (ply as usize).min(MAX_DEPTH as usize - 1);

        if Some(mv) != self.killer_moves[ply_idx][0] {
            self.killer_moves[ply_idx][1] = self.killer_moves[ply_idx][0];
            self.killer_moves[ply_idx][0] = Some(mv);
        }
    }

    fn update_history(&mut self, mv: Move, depth: u8) {
        if mv.is_capture() {
            return;
        }

        let bonus = (depth as i32).pow(2);
        self.history[mv.from() as usize][mv.to() as usize] += bonus;

        if self.history[mv.from() as usize][mv.to() as usize] > 10000 {
            self.age_history();
        }
    }

    fn age_history(&mut self) {
        for i in 0..64 {
            for j in 0..64 {
                self.history[i][j] /= 2;
            }
        }
    }

    fn is_endgame(&self, pos: &Position) -> bool {
        use crate::engine::position::{Color, PieceType};
        let queens = pos.piece_bb(PieceType::Queen, Color::White).count()
            + pos.piece_bb(PieceType::Queen, Color::Black).count();
        queens == 0
    }

    fn should_stop(&self) -> bool {
        if self.stop_search {
            return true;
        }

        if let Some(max_time) = self.limits.max_time {
            if self.start_time.elapsed() >= max_time {
                return true;
            }
        }

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

impl Default for SearchV2 {
    fn default() -> Self {
        Self::new(64)
    }
}
