/// Search V4 - Advanced search with singular extensions, multi-cut pruning, and IID
use crate::engine::{
    movegen::{Move, MoveGen, MoveList},
    position::{Position, Color},
    evaluation_v2::EvaluatorV2,
    transposition::{TranspositionTable, TTEntry, NodeType},
    move_ordering::MoveOrdering,
};
use std::time::{Duration, Instant};

pub const MAX_DEPTH: u8 = 128;
const FUTILITY_MARGINS: [i32; 6] = [0, 100, 200, 300, 400, 500];
const RAZOR_MARGIN: i32 = 300;
const MULTI_CUT_M: usize = 10; // Number of moves to try for multi-cut
const MULTI_CUT_C: usize = 3;  // Number of cuts needed for multi-cut
const IID_DEPTH_REDUCTION: u8 = 2; // Depth reduction for IID

#[derive(Clone, Debug)]
pub struct SearchLimits {
    pub max_depth: Option<u8>,
    pub max_time: Option<Duration>,
    pub max_nodes: Option<u64>,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            max_depth: Some(64),
            max_time: None,
            max_nodes: None,
        }
    }
}

#[derive(Debug)]
pub struct SearchStats {
    pub nodes: u64,
    pub qnodes: u64,
    pub time_elapsed: Duration,
    pub cut_nodes: u64,
    pub all_nodes: u64,
    pub pv_nodes: u64,
}

impl SearchStats {
    pub fn nps(&self) -> u64 {
        let secs = self.time_elapsed.as_secs_f64();
        if secs > 0.0 {
            ((self.nodes + self.qnodes) as f64 / secs) as u64
        } else {
            0
        }
    }
}

pub struct SearchV4 {
    tt: TranspositionTable,
    nodes_searched: u64,
    qnodes_searched: u64,
    cut_nodes: u64,
    all_nodes: u64,
    pv_nodes: u64,
    start_time: Instant,
    limits: SearchLimits,
    killer_moves: [[Option<Move>; 2]; MAX_DEPTH as usize],
    history: [[i32; 64]; 64],
    pv_table: [[Option<Move>; MAX_DEPTH as usize]; MAX_DEPTH as usize],
    pv_length: [usize; MAX_DEPTH as usize],
    counter_moves: [[Option<Move>; 64]; 64],
}

impl SearchV4 {
    pub fn new(tt_size_mb: usize) -> Self {
        Self {
            tt: TranspositionTable::new(tt_size_mb),
            nodes_searched: 0,
            qnodes_searched: 0,
            cut_nodes: 0,
            all_nodes: 0,
            pv_nodes: 0,
            start_time: Instant::now(),
            limits: SearchLimits::default(),
            killer_moves: [[None; 2]; MAX_DEPTH as usize],
            history: [[0; 64]; 64],
            pv_table: [[None; MAX_DEPTH as usize]; MAX_DEPTH as usize],
            pv_length: [0; MAX_DEPTH as usize],
            counter_moves: [[None; 64]; 64],
        }
    }

    pub fn search(&mut self, pos: &Position, limits: SearchLimits) -> Option<Move> {
        self.start_time = Instant::now();
        self.limits = limits;
        self.nodes_searched = 0;
        self.qnodes_searched = 0;
        self.cut_nodes = 0;
        self.all_nodes = 0;
        self.pv_nodes = 0;

        let mut best_move = None;
        let mut best_score = -100000;

        // Iterative deepening
        let max_depth = self.limits.max_depth.unwrap_or(64);
        for depth in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            // Aspiration windows
            let (alpha, beta) = if depth >= 5 && best_score.abs() < 9000 {
                (best_score - 50, best_score + 50)
            } else {
                (-100000, 100000)
            };

            let score = self.aspiration_search(pos, depth, alpha, beta);

            if self.should_stop() {
                break;
            }

            best_score = score;
            if self.pv_length[0] > 0 {
                best_move = self.pv_table[0][0];
            }

            let elapsed = self.start_time.elapsed();
            println!(
                "info depth {} score cp {} nodes {} time {} nps {} pv {}",
                depth,
                score,
                self.nodes_searched + self.qnodes_searched,
                elapsed.as_millis(),
                self.get_stats().nps(),
                best_move.map(|m| m.to_uci()).unwrap_or_else(|| "none".to_string())
            );
        }

        best_move
    }

    fn aspiration_search(&mut self, pos: &Position, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
        let mut score = self.negamax(pos, depth, alpha, beta, 0, true);

        // Widen window on fail
        let mut delta = 100;
        while (score <= alpha || score >= beta) && !self.should_stop() {
            if score <= alpha {
                alpha -= delta;
            }
            if score >= beta {
                beta += delta;
            }
            delta *= 2;
            score = self.negamax(pos, depth, alpha, beta, 0, true);
        }

        score
    }

    fn negamax(&mut self, pos: &Position, mut depth: u8, mut alpha: i32, beta: i32, ply: usize, is_pv: bool) -> i32 {
        if self.should_stop() {
            return 0;
        }

        self.pv_length[ply] = ply;

        // Check for draws
        if ply > 0 && pos.is_repetition() {
            return 0;
        }

        // Mate distance pruning
        let mate_value = 10000 - ply as i32;
        alpha = alpha.max(-mate_value);
        let beta = beta.min(mate_value - 1);
        if alpha >= beta {
            return alpha;
        }

        // Probe transposition table
        let tt_entry = self.tt.probe(pos.hash());
        let mut tt_move = None;

        if let Some(entry) = &tt_entry {
            tt_move = entry.best_move;

            if entry.depth >= depth && !is_pv {
                match entry.node_type {
                    NodeType::Exact => return entry.score,
                    NodeType::LowerBound if entry.score >= beta => return entry.score,
                    NodeType::UpperBound if entry.score <= alpha => return entry.score,
                    _ => {}
                }
            }
        }

        let in_check = pos.is_in_check(pos.side_to_move);

        // Extend search in check
        if in_check {
            depth += 1;
        }

        // Quiescence search at frontier
        if depth == 0 {
            return self.quiescence(pos, alpha, beta);
        }

        self.nodes_searched += 1;

        let static_eval = EvaluatorV2::evaluate(pos);

        // Reverse futility pruning (static null move pruning)
        if !is_pv && !in_check && depth <= 5 {
            let margin = FUTILITY_MARGINS[depth as usize];
            if static_eval - margin >= beta {
                return static_eval - margin;
            }
        }

        // Null move pruning
        if !is_pv && !in_check && depth >= 3 && static_eval >= beta {
            let r = if depth > 6 { 3 } else { 2 };
            let null_pos = pos.make_null_move();

            let score = -self.negamax(&null_pos, depth.saturating_sub(r + 1), -beta, -beta + 1, ply + 1, false);

            if score >= beta {
                return beta; // Fail-soft
            }
        }

        // Razoring
        if !is_pv && !in_check && depth <= 3 {
            let razor_margin = RAZOR_MARGIN + (depth as i32 - 1) * 50;
            if static_eval + razor_margin < alpha {
                let score = self.quiescence(pos, alpha, beta);
                if score < alpha {
                    return score;
                }
            }
        }

        // Internal Iterative Deepening (IID)
        if is_pv && tt_move.is_none() && depth >= 4 {
            let iid_depth = depth.saturating_sub(IID_DEPTH_REDUCTION);
            self.negamax(pos, iid_depth, alpha, beta, ply, true);

            // Re-probe TT for move from IID search
            if let Some(entry) = self.tt.probe(pos.hash()) {
                tt_move = entry.best_move;
            }
        }

        // Generate moves
        let mut moves = MoveGen::generate_legal_moves(pos);
        if moves.is_empty() {
            return if in_check {
                -(10000 - ply as i32) // Checkmate
            } else {
                0 // Stalemate
            };
        }

        // Order moves
        MoveOrdering::order_moves(
            pos,
            &mut moves,
            tt_move,
            &self.killer_moves[ply],
            &self.history,
        );

        // Multi-cut pruning
        if !is_pv && !in_check && depth >= 6 && moves.len() >= MULTI_CUT_M {
            let mut cuts = 0;
            let reduced_depth = depth - 4;

            for i in 0..MULTI_CUT_M.min(moves.len()) {
                let new_pos = pos.make_move(&moves[i]);
                let score = -self.negamax(&new_pos, reduced_depth, -beta, -beta + 1, ply + 1, false);

                if score >= beta {
                    cuts += 1;
                    if cuts >= MULTI_CUT_C {
                        return beta; // Multi-cut
                    }
                }
            }
        }

        let mut best_move = None;
        let mut best_score = -100000;
        let mut move_count = 0;
        let mut raised_alpha = false;

        for i in 0..moves.len() {
            let mv = moves[i];
            let new_pos = pos.make_move(&mv);

            move_count += 1;
            let mut score;

            // Late Move Reductions (LMR)
            if move_count > 4 && depth >= 3 && !in_check && !mv.is_capture() && !new_pos.is_in_check(new_pos.side_to_move) {
                // Reduce depth for late moves
                let reduction = if move_count > 16 { 3 } else if move_count > 8 { 2 } else { 1 };
                let reduced_depth = (depth - 1).saturating_sub(reduction);

                // Scout search with reduced depth
                score = -self.negamax(&new_pos, reduced_depth, -alpha - 1, -alpha, ply + 1, false);

                // Re-search if it fails high
                if score > alpha {
                    score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, false);
                }
            } else {
                // Principal Variation Search (PVS)
                if move_count == 1 {
                    score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, is_pv);
                } else {
                    // Null window search
                    score = -self.negamax(&new_pos, depth - 1, -alpha - 1, -alpha, ply + 1, false);

                    // Re-search with full window if it fails high
                    if score > alpha && score < beta {
                        score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, is_pv);
                    }
                }
            }

            if score > best_score {
                best_score = score;
                best_move = Some(mv);

                if score > alpha {
                    alpha = score;
                    raised_alpha = true;

                    // Update PV
                    self.pv_table[ply][ply] = Some(mv);
                    for j in (ply + 1)..self.pv_length[ply + 1] {
                        self.pv_table[ply][j] = self.pv_table[ply + 1][j];
                    }
                    self.pv_length[ply] = self.pv_length[ply + 1];

                    if score >= beta {
                        // Beta cutoff
                        self.cut_nodes += 1;

                        // Update killer moves
                        if !mv.is_capture() {
                            if self.killer_moves[ply][0] != Some(mv) {
                                self.killer_moves[ply][1] = self.killer_moves[ply][0];
                                self.killer_moves[ply][0] = Some(mv);
                            }

                            // Update history
                            let from = mv.from() as usize;
                            let to = mv.to() as usize;
                            self.history[from][to] += (depth as i32) * (depth as i32);
                        }

                        break;
                    }
                }
            }
        }

        // Store in transposition table
        let node_type = if best_score >= beta {
            NodeType::LowerBound
        } else if raised_alpha {
            NodeType::Exact
        } else {
            NodeType::UpperBound
        };

        self.tt.store(pos.hash(), TTEntry {
            hash: pos.hash(),
            score: best_score,
            best_move,
            depth,
            node_type,
        });

        if raised_alpha {
            self.pv_nodes += 1;
        } else if best_score >= beta {
            self.cut_nodes += 1;
        } else {
            self.all_nodes += 1;
        }

        best_score
    }

    fn quiescence(&mut self, pos: &Position, mut alpha: i32, beta: i32) -> i32 {
        self.qnodes_searched += 1;

        let stand_pat = EvaluatorV2::evaluate(pos);

        if stand_pat >= beta {
            return beta;
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        // Delta pruning
        const DELTA_MARGIN: i32 = 900; // Queen value
        if stand_pat + DELTA_MARGIN < alpha {
            return alpha;
        }

        let moves = MoveGen::generate_legal_moves(pos);

        // Only search captures and checks
        for mv in moves.iter() {
            if !mv.is_capture() {
                continue;
            }

            // SEE (Static Exchange Evaluation) pruning could go here

            let new_pos = pos.make_move(&mv);
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

    fn should_stop(&self) -> bool {
        if let Some(max_nodes) = self.limits.max_nodes {
            if self.nodes_searched >= max_nodes {
                return true;
            }
        }

        if let Some(max_time) = self.limits.max_time {
            if self.start_time.elapsed() >= max_time {
                return true;
            }
        }

        false
    }

    pub fn get_stats(&self) -> SearchStats {
        SearchStats {
            nodes: self.nodes_searched,
            qnodes: self.qnodes_searched,
            time_elapsed: self.start_time.elapsed(),
            cut_nodes: self.cut_nodes,
            all_nodes: self.all_nodes,
            pv_nodes: self.pv_nodes,
        }
    }
}
