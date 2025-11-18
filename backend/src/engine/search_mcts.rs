/// Monte Carlo Tree Search (MCTS) engine
/// Uses random playouts with UCT (Upper Confidence Bound for Trees) selection
use crate::engine::{
    movegen::{Move, MoveGen},
    position::{Position, Color},
    evaluation_v2::EvaluatorV2,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;

const EXPLORATION_CONSTANT: f64 = 1.41; // sqrt(2), standard UCT constant
const MAX_PLAYOUT_DEPTH: usize = 200;

#[derive(Clone, Debug)]
pub struct MCTSLimits {
    pub max_iterations: Option<usize>,
    pub max_time: Option<Duration>,
}

impl Default for MCTSLimits {
    fn default() -> Self {
        Self {
            max_iterations: Some(10000),
            max_time: Some(Duration::from_secs(5)),
        }
    }
}

#[derive(Debug)]
pub struct MCTSStats {
    pub iterations: usize,
    pub nodes_created: usize,
    pub total_playouts: usize,
    pub time_elapsed: Duration,
}

#[derive(Clone)]
struct MCTSNode {
    position_hash: u64,
    move_from_parent: Option<Move>,
    visits: usize,
    wins: f64,
    children: Vec<usize>, // Indices into node pool
    untried_moves: Vec<Move>,
    player_to_move: Color,
}

impl MCTSNode {
    fn new(pos: &Position, move_from_parent: Option<Move>) -> Self {
        let legal_moves = MoveGen::generate_legal_moves(pos);
        Self {
            position_hash: pos.hash,
            move_from_parent,
            visits: 0,
            wins: 0.0,
            children: Vec::new(),
            untried_moves: legal_moves.into_iter().collect(),
            player_to_move: pos.side_to_move,
        }
    }

    fn is_fully_expanded(&self) -> bool {
        self.untried_moves.is_empty()
    }

    fn is_terminal(&self) -> bool {
        self.untried_moves.is_empty() && self.children.is_empty()
    }

    fn uct_value(&self, parent_visits: usize, exploration: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }

        let exploitation = self.wins / self.visits as f64;
        let exploration_term = exploration * ((parent_visits as f64).ln() / self.visits as f64).sqrt();

        exploitation + exploration_term
    }
}

pub struct MCTSSearch {
    nodes: Vec<MCTSNode>,
    start_time: Instant,
    limits: MCTSLimits,
    iterations: usize,
    total_playouts: usize,
}

impl MCTSSearch {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            start_time: Instant::now(),
            limits: MCTSLimits::default(),
            iterations: 0,
            total_playouts: 0,
        }
    }

    pub fn search(&mut self, pos: &Position, limits: MCTSLimits) -> Option<Move> {
        self.start_time = Instant::now();
        self.limits = limits;
        self.iterations = 0;
        self.total_playouts = 0;
        self.nodes = Vec::new();

        // Create root node
        let root = MCTSNode::new(pos, None);
        self.nodes.push(root);

        // MCTS main loop
        while !self.should_stop() {
            self.iterations += 1;

            // 1. Selection - traverse tree using UCT
            let (selected_idx, selected_pos) = self.select(pos, 0);

            // 2. Expansion - add new child node
            let (child_idx, child_pos) = if !self.nodes[selected_idx].is_fully_expanded() {
                self.expand(selected_idx, &selected_pos)
            } else {
                (selected_idx, selected_pos)
            };

            // 3. Simulation - random playout
            let result = self.simulate(&child_pos);

            // 4. Backpropagation - update statistics
            self.backpropagate(child_idx, result);

            // Print progress every 1000 iterations
            if self.iterations % 1000 == 0 {
                let best_move = self.get_best_move();
                let elapsed = self.start_time.elapsed();
                println!(
                    "info nodes {} time {} iters {} pv {}",
                    self.nodes.len(),
                    elapsed.as_millis(),
                    self.iterations,
                    best_move.map(|m| m.to_uci()).unwrap_or_else(|| "none".to_string())
                );
            }
        }

        self.get_best_move()
    }

    fn select(&self, root_pos: &Position, mut node_idx: usize) -> (usize, Position) {
        let mut current_pos = root_pos.clone();

        // Traverse down the tree using UCT
        while !self.nodes[node_idx].is_terminal() && self.nodes[node_idx].is_fully_expanded() {
            let parent_visits = self.nodes[node_idx].visits;

            // Select best child using UCT
            let mut best_uct = f64::NEG_INFINITY;
            let mut best_child_idx = None;

            for &child_idx in &self.nodes[node_idx].children {
                let uct = self.nodes[child_idx].uct_value(parent_visits, EXPLORATION_CONSTANT);

                if uct > best_uct {
                    best_uct = uct;
                    best_child_idx = Some(child_idx);
                }
            }

            if let Some(child_idx) = best_child_idx {
                if let Some(mv) = self.nodes[child_idx].move_from_parent {
                    if current_pos.make_move(mv).is_err() {
                        break;
                    }
                    node_idx = child_idx;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        (node_idx, current_pos)
    }

    fn expand(&mut self, parent_idx: usize, parent_pos: &Position) -> (usize, Position) {
        if self.nodes[parent_idx].untried_moves.is_empty() {
            return (parent_idx, parent_pos.clone());
        }

        // Select random untried move
        let move_idx = fastrand::usize(..self.nodes[parent_idx].untried_moves.len());
        let mv = self.nodes[parent_idx].untried_moves.remove(move_idx);

        // Make move
        let mut child_pos = parent_pos.clone();
        if child_pos.make_move(mv).is_err() {
            return (parent_idx, parent_pos.clone());
        }

        // Create child node
        let child = MCTSNode::new(&child_pos, Some(mv));
        let child_idx = self.nodes.len();
        self.nodes.push(child);

        // Add child to parent
        self.nodes[parent_idx].children.push(child_idx);

        (child_idx, child_pos)
    }

    fn simulate(&mut self, start_pos: &Position) -> f64 {
        self.total_playouts += 1;

        let mut pos = start_pos.clone();
        let root_color = start_pos.side_to_move;

        // Random playout until terminal or max depth
        for _ in 0..MAX_PLAYOUT_DEPTH {
            let moves = MoveGen::generate_legal_moves(&pos);

            if moves.is_empty() {
                // Terminal position
                if pos.is_check() {
                    // Checkmate - opponent wins
                    return if pos.side_to_move == root_color {
                        0.0 // We lost
                    } else {
                        1.0 // We won
                    };
                } else {
                    // Stalemate
                    return 0.5;
                }
            }

            // Select random move
            let move_idx = fastrand::usize(..moves.len());
            let mv = moves.iter().nth(move_idx).unwrap();
            if pos.make_move(*mv).is_err() {
                break;
            }
        }

        // Evaluate position heuristically
        let eval = EvaluatorV2::evaluate(&pos);
        let normalized = self.sigmoid(eval as f64 / 100.0);

        // Return result from root player's perspective
        if root_color == Color::White {
            normalized
        } else {
            1.0 - normalized
        }
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn backpropagate(&mut self, mut node_idx: usize, result: f64) {
        // Propagate result up the tree
        loop {
            self.nodes[node_idx].visits += 1;
            self.nodes[node_idx].wins += result;

            // Find parent
            let mut parent_idx = None;
            for (idx, node) in self.nodes.iter().enumerate() {
                if node.children.contains(&node_idx) {
                    parent_idx = Some(idx);
                    break;
                }
            }

            if let Some(idx) = parent_idx {
                node_idx = idx;
            } else {
                break; // Reached root
            }
        }
    }

    fn get_best_move(&self) -> Option<Move> {
        if self.nodes.is_empty() {
            return None;
        }

        let root = &self.nodes[0];
        if root.children.is_empty() {
            return None;
        }

        // Select child with most visits (most robust)
        let mut best_visits = 0;
        let mut best_move = None;

        for &child_idx in &root.children {
            let visits = self.nodes[child_idx].visits;
            if visits > best_visits {
                best_visits = visits;
                best_move = self.nodes[child_idx].move_from_parent;
            }
        }

        best_move
    }

    fn should_stop(&self) -> bool {
        if let Some(max_iterations) = self.limits.max_iterations {
            if self.iterations >= max_iterations {
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

    pub fn get_stats(&self) -> MCTSStats {
        MCTSStats {
            iterations: self.iterations,
            nodes_created: self.nodes.len(),
            total_playouts: self.total_playouts,
            time_elapsed: self.start_time.elapsed(),
        }
    }

    /// Print statistics about the search tree
    pub fn print_tree_stats(&self) {
        if self.nodes.is_empty() {
            return;
        }

        let root = &self.nodes[0];
        println!("\n=== MCTS Tree Statistics ===");
        println!("Total nodes: {}", self.nodes.len());
        println!("Total iterations: {}", self.iterations);
        println!("Total playouts: {}", self.total_playouts);
        println!("Root visits: {}", root.visits);
        println!("\nTop moves:");

        let mut children: Vec<_> = root.children.iter()
            .map(|&idx| (idx, &self.nodes[idx]))
            .collect();

        children.sort_by(|a, b| b.1.visits.cmp(&a.1.visits));

        for (i, (_idx, node)) in children.iter().take(10).enumerate() {
            if let Some(mv) = node.move_from_parent {
                let win_rate = if node.visits > 0 {
                    node.wins / node.visits as f64
                } else {
                    0.0
                };

                println!(
                    "  {}. {} - visits: {}, win rate: {:.1}%",
                    i + 1,
                    mv.to_uci(),
                    node.visits,
                    win_rate * 100.0
                );
            }
        }
    }
}

impl Default for MCTSSearch {
    fn default() -> Self {
        Self::new()
    }
}
