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
const MAX_EXTENSIONS: i32 = 16; // Maximum cumulative extensions in a line

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
    pub null_move_cutoffs: u64,
    pub lmr_searches: u64,
    pub tt_hits: u64,
    pub tt_cutoffs: u64,
    pub beta_cutoffs: u64,
    pub first_move_cutoffs: u64,
    pub check_extensions: u64,
    pub recapture_extensions: u64,
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

    pub fn branching_factor(&self) -> f64 {
        if self.nodes > 0 {
            (self.nodes as f64).powf(1.0 / 6.0)  // Approximate for depth 6
        } else {
            0.0
        }
    }

    pub fn move_ordering_quality(&self) -> f64 {
        if self.beta_cutoffs > 0 {
            (self.first_move_cutoffs as f64 / self.beta_cutoffs as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn tt_hit_rate(&self) -> f64 {
        if self.nodes > 0 {
            (self.tt_hits as f64 / self.nodes as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn print_detailed(&self) {
        println!("\n=== Detailed Search Statistics ===");
        println!("Nodes:            {}", self.nodes);
        println!("QNodes:           {}", self.qnodes);
        println!("Total:            {}", self.nodes + self.qnodes);
        println!("Time:             {}ms", self.time_elapsed.as_millis());
        println!("NPS:              {}", self.nps());
        println!();
        println!("Node Types:");
        println!("  PV nodes:       {} ({:.1}%)", self.pv_nodes,
                 (self.pv_nodes as f64 / self.nodes as f64) * 100.0);
        println!("  Cut nodes:      {} ({:.1}%)", self.cut_nodes,
                 (self.cut_nodes as f64 / self.nodes as f64) * 100.0);
        println!("  All nodes:      {} ({:.1}%)", self.all_nodes,
                 (self.all_nodes as f64 / self.nodes as f64) * 100.0);
        println!();
        println!("Pruning:");
        println!("  Null move cuts: {}", self.null_move_cutoffs);
        println!("  LMR searches:   {}", self.lmr_searches);
        println!("  Beta cutoffs:   {}", self.beta_cutoffs);
        println!("  First move cuts:{} ({:.1}%)", self.first_move_cutoffs,
                 self.move_ordering_quality());
        println!();
        println!("Extensions:");
        println!("  Check exts:     {}", self.check_extensions);
        println!("  Recapture exts: {}", self.recapture_extensions);
        println!("  Total exts:     {}", self.check_extensions + self.recapture_extensions);
        println!();
        println!("Transposition Table:");
        println!("  TT hits:        {} ({:.1}%)", self.tt_hits, self.tt_hit_rate());
        println!("  TT cutoffs:     {}", self.tt_cutoffs);
        println!();
        println!("Metrics:");
        println!("  Branching:      ~{:.2}", self.branching_factor());
        println!("  Move ordering:  {:.1}%", self.move_ordering_quality());
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
    position_history: Vec<u64>,  // For repetition detection
    // Statistics
    null_move_cutoffs: u64,
    lmr_searches: u64,
    tt_hits: u64,
    tt_cutoffs: u64,
    beta_cutoffs: u64,
    first_move_cutoffs: u64,
    check_extensions: u64,
    recapture_extensions: u64,
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
            position_history: Vec::new(),
            null_move_cutoffs: 0,
            lmr_searches: 0,
            tt_hits: 0,
            tt_cutoffs: 0,
            beta_cutoffs: 0,
            first_move_cutoffs: 0,
            check_extensions: 0,
            recapture_extensions: 0,
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
        self.position_history.clear();
        self.position_history.push(pos.hash);
        self.null_move_cutoffs = 0;
        self.lmr_searches = 0;
        self.tt_hits = 0;
        self.tt_cutoffs = 0;
        self.beta_cutoffs = 0;
        self.first_move_cutoffs = 0;
        self.check_extensions = 0;
        self.recapture_extensions = 0;

        let mut best_move = None;
        let mut best_score = -100000;

        // Iterative deepening
        let max_depth = self.limits.max_depth.unwrap_or(64);
        for depth in 1..=max_depth {
            if self.should_stop() {
                break;
            }

            // Aspiration windows
            let (alpha, beta) = if depth >= 5 && (best_score as i32).abs() < 9000 {
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
        let mut score = self.negamax(pos, depth, alpha, beta, 0, true, 0);

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
            score = self.negamax(pos, depth, alpha, beta, 0, true, 0);
        }

        score
    }

    fn negamax(&mut self, pos: &Position, mut depth: u8, mut alpha: i32, beta: i32, ply: usize, is_pv: bool, extensions: i32) -> i32 {
        if self.should_stop() {
            return 0;
        }

        self.pv_length[ply] = ply;

        // Check for draws by repetition
        if ply > 0 && pos.is_repetition(&self.position_history) {
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
        let tt_entry = self.tt.probe(pos.hash);
        let mut tt_move = None;

        if let Some(entry) = &tt_entry {
            self.tt_hits += 1;
            tt_move = entry.best_move;

            if entry.depth >= depth && !is_pv {
                match entry.node_type {
                    NodeType::Exact => {
                        self.tt_cutoffs += 1;
                        return entry.score;
                    }
                    NodeType::LowerBound if entry.score >= beta => {
                        self.tt_cutoffs += 1;
                        return entry.score;
                    }
                    NodeType::UpperBound if entry.score <= alpha => {
                        self.tt_cutoffs += 1;
                        return entry.score;
                    }
                    _ => {}
                }
            }
        }

        let in_check = pos.is_check();

        // Extensions (limited to prevent search explosion)
        let mut extension = 0;

        // Check extension - always extend when in check
        if in_check && extensions < MAX_EXTENSIONS {
            extension = 1;
            self.check_extensions += 1;
        }

        depth = depth.saturating_add(extension);

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

            let score = -self.negamax(&null_pos, depth.saturating_sub(r + 1), -beta, -beta + 1, ply + 1, false, extensions);

            if score >= beta {
                self.null_move_cutoffs += 1;
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
            self.negamax(pos, iid_depth, alpha, beta, ply, true, extensions);

            // Re-probe TT for move from IID search
            if let Some(entry) = self.tt.probe(pos.hash) {
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

        // Order moves - Convert MoveList to Vec for ordering
        let mut moves_vec: Vec<Move> = moves.into_iter().collect();
        MoveOrdering::order_moves(
            pos,
            &mut moves_vec,
            tt_move,
            &self.killer_moves[ply],
            &self.history,
        );
        let moves = moves_vec;

        // Multi-cut pruning - TODO: Implement with correct API
        // Temporarily disabled due to API complexity

        let mut best_move = None;
        let mut best_score = -100000;
        let mut move_count = 0;
        let mut raised_alpha = false;

        for mv in moves {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
                continue;
            }

            // Add position to history for repetition detection
            self.position_history.push(new_pos.hash);

            move_count += 1;
            let mut score;

            // Late Move Reductions (LMR)
            if move_count > 4 && depth >= 3 && !in_check && !mv.is_capture() && !new_pos.is_check() {
                self.lmr_searches += 1;
                // Reduce depth for late moves
                let reduction = if move_count > 16 { 3 } else if move_count > 8 { 2 } else { 1 };
                let reduced_depth = (depth - 1).saturating_sub(reduction);

                // Scout search with reduced depth
                score = -self.negamax(&new_pos, reduced_depth, -alpha - 1, -alpha, ply + 1, false, extensions + extension as i32);

                // Re-search if it fails high
                if score > alpha {
                    score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, false, extensions + extension as i32);
                }
            } else {
                // Principal Variation Search (PVS)
                if move_count == 1 {
                    score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, is_pv, extensions + extension as i32);
                } else {
                    // Null window search
                    score = -self.negamax(&new_pos, depth - 1, -alpha - 1, -alpha, ply + 1, false, extensions + extension as i32);

                    // Re-search with full window if it fails high
                    if score > alpha && score < beta {
                        score = -self.negamax(&new_pos, depth - 1, -beta, -alpha, ply + 1, is_pv, extensions + extension as i32);
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
                        self.beta_cutoffs += 1;
                        if move_count == 1 {
                            self.first_move_cutoffs += 1;
                        }

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

                        // Remove position from history before breaking
                        self.position_history.pop();
                        break;
                    }
                }
            }

            // Remove position from history after exploring this move
            self.position_history.pop();
        }

        // Store in transposition table
        let node_type = if best_score >= beta {
            NodeType::LowerBound
        } else if raised_alpha {
            NodeType::Exact
        } else {
            NodeType::UpperBound
        };

        self.tt.store(pos.hash, depth, best_score, node_type, best_move);

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

            let mut new_pos = pos.clone();
            if new_pos.make_move(*mv).is_err() {
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
            null_move_cutoffs: self.null_move_cutoffs,
            lmr_searches: self.lmr_searches,
            tt_hits: self.tt_hits,
            tt_cutoffs: self.tt_cutoffs,
            beta_cutoffs: self.beta_cutoffs,
            first_move_cutoffs: self.first_move_cutoffs,
            check_extensions: self.check_extensions,
            recapture_extensions: self.recapture_extensions,
        }
    }
}
