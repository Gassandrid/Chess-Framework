use crate::engine::{
    movegen::{Move, MoveGen},
    position::Position,
    evaluation_v2::EvaluatorV2,
    transposition::{TranspositionTable, NodeType},
    move_ordering::MoveOrdering,
};
use std::time::{Duration, Instant};

pub const MAX_DEPTH: u8 = 64;
const INFINITY: i32 = 30000;
const MATE_SCORE: i32 = 29000;

// Futility pruning margins (in centipawns)
const FUTILITY_MARGINS: [i32; 5] = [0, 100, 200, 300, 400];

// Razoring margins
const RAZOR_MARGIN: i32 = 300;

#[derive(Default)]
pub struct SearchLimits {
    pub max_depth: Option<u8>,
    pub max_time: Option<Duration>,
    pub max_nodes: Option<u64>,
}

pub struct SearchV3 {
    tt: TranspositionTable,
    nodes_searched: u64,
    start_time: Instant,
    limits: SearchLimits,
    killer_moves: [[Option<Move>; 2]; MAX_DEPTH as usize],
    history: [[i32; 64]; 64],
    pv_table: [[Option<Move>; MAX_DEPTH as usize]; MAX_DEPTH as usize],
}

impl SearchV3 {
    pub fn new(tt_size_mb: usize) -> Self {
        Self {
            tt: TranspositionTable::new(tt_size_mb),
            nodes_searched: 0,
            start_time: Instant::now(),
            limits: SearchLimits::default(),
            killer_moves: [[None; 2]; MAX_DEPTH as usize],
            history: [[0; 64]; 64],
            pv_table: [[None; MAX_DEPTH as usize]; MAX_DEPTH as usize],
        }
    }

    pub fn search(&mut self, pos: &Position, limits: SearchLimits) -> Option<Move> {
        self.limits = limits;
        self.start_time = Instant::now();
        self.nodes_searched = 0;

        let mut best_move = None;
        let mut prev_score = 0;

        // Iterative deepening
        for depth in 1..=self.limits.max_depth.unwrap_or(MAX_DEPTH) {
            if self.should_stop() {
                break;
            }

            // Aspiration windows - start with narrow window around previous score
            let (alpha, beta) = if depth >= 4 {
                let window = 50; // 50 centipawns
                (prev_score - window, prev_score + window)
            } else {
                (-INFINITY, INFINITY)
            };

            let mut score = self.aspiration_search(pos, depth, alpha, beta);

            // If we fail outside the window, research with full window
            if score <= alpha || score >= beta {
                score = self.negamax(pos, depth, -INFINITY, INFINITY, 0, true);
            }

            if self.should_stop() {
                break;
            }

            prev_score = score;
            best_move = self.pv_table[0][0];

            // Print search info
            let elapsed = self.start_time.elapsed().as_millis().max(1);
            let nps = (self.nodes_searched as u128 * 1000 / elapsed) as u64;

            print!("info depth {} score cp {} nodes {} nps {} time {}",
                   depth, score, self.nodes_searched, nps, elapsed);

            // Print PV
            print!(" pv");
            for i in 0..depth as usize {
                if let Some(mv) = self.pv_table[0][i] {
                    print!(" {}", mv.to_uci());
                } else {
                    break;
                }
            }
            println!();
        }

        best_move
    }

    fn aspiration_search(&mut self, pos: &Position, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
        let mut score = self.negamax(pos, depth, alpha, beta, 0, true);

        // Widen window if we fail
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

    fn negamax(
        &mut self,
        pos: &Position,
        depth: u8,
        mut alpha: i32,
        beta: i32,
        ply: u8,
        do_null: bool,
    ) -> i32 {
        if self.should_stop() {
            return 0;
        }

        let is_root = ply == 0;
        let in_check = pos.is_check();

        // Check extension
        let adjusted_depth = if in_check && depth < MAX_DEPTH - 1 {
            depth + 1
        } else {
            depth
        };

        if adjusted_depth == 0 {
            return self.quiescence(pos, alpha, beta);
        }

        self.nodes_searched += 1;

        // Probe transposition table
        let tt_entry = self.tt.probe(pos.hash);
        let tt_move = tt_entry.and_then(|e| e.best_move);

        if !is_root {
            if let Some(entry) = tt_entry {
                if entry.depth >= adjusted_depth {
                    match entry.node_type {
                        NodeType::Exact => return entry.score,
                        NodeType::LowerBound if entry.score >= beta => return entry.score,
                        NodeType::UpperBound if entry.score <= alpha => return entry.score,
                        _ => {}, // Don't use the TT entry
                    }
                }
            }
        }

        let static_eval = EvaluatorV2::evaluate(pos);

        // Razoring - if position is very bad, reduce depth
        if !in_check && !is_root && depth <= 3 {
            let razor_margin = RAZOR_MARGIN + (depth as i32 - 1) * 50;
            if static_eval + razor_margin < alpha {
                let score = self.quiescence(pos, alpha, beta);
                if score < alpha {
                    return score;
                }
            }
        }

        // Futility pruning - if we're close to leaf and position is bad
        let futility_prune = !in_check &&
                            !is_root &&
                            depth <= 4 &&
                            static_eval + FUTILITY_MARGINS[depth as usize] <= alpha;

        // Null move pruning
        if do_null && !in_check && depth >= 3 && static_eval >= beta {
            let mut null_pos = pos.clone();
            null_pos.side_to_move = !null_pos.side_to_move;
            null_pos.en_passant_square = None;

            let r = if depth >= 6 { 3 } else { 2 };
            let score = -self.negamax(&null_pos, depth - 1 - r, -beta, -beta + 1, ply + 1, false);

            if score >= beta {
                return beta;
            }
        }

        // Generate and order moves
        let moves = MoveGen::generate_legal_moves(pos);
        let mut move_vec: Vec<Move> = moves.into_iter().collect();
        MoveOrdering::order_moves(
            pos,
            &mut move_vec,
            tt_move,
            &self.killer_moves[ply as usize],
            &self.history,
        );
        let mut moves = crate::engine::movegen::MoveList::new();
        for mv in move_vec {
            moves.push(mv);
        }

        if moves.is_empty() {
            return if in_check {
                -MATE_SCORE + ply as i32
            } else {
                0 // Stalemate
            };
        }

        let mut best_score = -INFINITY;
        let mut best_move = None;
        let original_alpha = alpha;
        let mut move_count = 0;

        for mv in moves.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_err() {
                continue;
            }

            move_count += 1;

            // Futility pruning - skip quiet moves in hopeless positions
            if futility_prune &&
               move_count > 1 &&
               !mv.is_capture() &&
               !mv.is_promotion() &&
               !new_pos.is_check() {
                continue;
            }

            let score = if move_count == 1 {
                // Full window search for first move
                -self.negamax(&new_pos, adjusted_depth - 1, -beta, -alpha, ply + 1, true)
            } else {
                // Late move reductions
                let reduction = if move_count > 4
                    && adjusted_depth > 2
                    && !mv.is_capture()
                    && !mv.is_promotion()
                    && !in_check
                    && !new_pos.is_check()
                {
                    // More aggressive LMR
                    if move_count > 8 && adjusted_depth > 4 {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                };

                // Try with reduced depth first
                let mut score = if reduction > 0 {
                    -self.negamax(&new_pos, adjusted_depth - 1 - reduction, -alpha - 1, -alpha, ply + 1, true)
                } else {
                    alpha + 1 // Ensure we do a full search
                };

                // Re-search if reduction was too optimistic
                if score > alpha && reduction > 0 {
                    score = -self.negamax(&new_pos, adjusted_depth - 1, -alpha - 1, -alpha, ply + 1, true);
                }

                // PVS - if we beat alpha, do full window search
                if score > alpha && score < beta {
                    score = -self.negamax(&new_pos, adjusted_depth - 1, -beta, -alpha, ply + 1, true);
                }

                score
            };

            if score > best_score {
                best_score = score;
                best_move = Some(mv);

                // Update PV
                self.pv_table[ply as usize][0] = Some(mv);
                for i in 0..(MAX_DEPTH as usize - ply as usize - 1) {
                    self.pv_table[ply as usize][i + 1] = self.pv_table[ply as usize + 1][i];
                    if self.pv_table[ply as usize][i + 1].is_none() {
                        break;
                    }
                }
            }

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                // Beta cutoff - update killer moves and history
                if !mv.is_capture() {
                    // Update killer moves
                    let ply_idx = ply as usize;
                    if self.killer_moves[ply_idx][0] != Some(mv) {
                        self.killer_moves[ply_idx][1] = self.killer_moves[ply_idx][0];
                        self.killer_moves[ply_idx][0] = Some(mv);
                    }

                    // Update history heuristic
                    let from = mv.from() as usize;
                    let to = mv.to() as usize;
                    self.history[from][to] += (depth * depth) as i32;
                }
                break;
            }
        }

        // Store in transposition table
        let entry_type = if best_score >= beta {
            NodeType::LowerBound
        } else if best_score <= original_alpha {
            NodeType::UpperBound
        } else {
            NodeType::Exact
        };

        self.tt.store(pos.hash, adjusted_depth, best_score, entry_type, best_move);

        best_score
    }

    fn quiescence(&mut self, pos: &Position, mut alpha: i32, beta: i32) -> i32 {
        if self.should_stop() {
            return 0;
        }

        self.nodes_searched += 1;

        let stand_pat = EvaluatorV2::evaluate(pos);

        if stand_pat >= beta {
            return beta;
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let moves = MoveGen::generate_legal_moves(pos);
        let mut captures: Vec<Move> = moves.into_iter()
            .filter(|mv| mv.is_capture() || mv.is_promotion())
            .collect();

        // Order captures by MVV-LVA
        MoveOrdering::order_moves(
            pos,
            &mut captures,
            None,
            &[None, None],
            &self.history,
        );

        for mv in captures {
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
}
