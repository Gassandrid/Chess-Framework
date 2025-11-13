use crate::engine::{
    position::Position,
    movegen::{Move, MoveGen},
    search_v2::SearchV2,
    search_v3::SearchV3,
    evaluation_v2::EvaluatorV2,
    ensemble_eval::{EnsembleEvaluator, EvaluationBreakdown},
};
use std::time::Duration;

/// Comprehensive position analysis
#[derive(Debug, Clone)]
pub struct PositionAnalysis {
    pub static_eval: i32,
    pub search_eval: Option<i32>,
    pub best_move: Option<Move>,
    pub best_line: Vec<Move>,
    pub top_moves: Vec<(Move, i32)>,
    pub evaluation_breakdown: Option<EvaluationBreakdown>,
    pub tactical_features: TacticalFeatures,
    pub positional_features: PositionalFeatures,
}

#[derive(Debug, Clone)]
pub struct TacticalFeatures {
    pub checks_available: usize,
    pub captures_available: usize,
    pub threatened_pieces: Vec<String>,
    pub hanging_pieces: Vec<String>,
    pub pins: Vec<String>,
    pub forks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PositionalFeatures {
    pub material_balance: i32,
    pub piece_activity: i32,
    pub pawn_structure_score: i32,
    pub king_safety: i32,
    pub center_control: i32,
}

pub struct Analyzer {
    search_v2: SearchV2,
    search_v3: SearchV3,
    ensemble: Option<EnsembleEvaluator>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            search_v2: SearchV2::new(128),
            search_v3: SearchV3::new(128),
            ensemble: None,
        }
    }

    pub fn with_ensemble(mut self, ensemble: EnsembleEvaluator) -> Self {
        self.ensemble = Some(ensemble);
        self
    }

    /// Analyze position comprehensively
    pub fn analyze_position(&mut self, pos: &Position, depth: u8) -> PositionAnalysis {
        // Static evaluation
        let static_eval = EvaluatorV2::evaluate(pos);

        // Search for best move
        let limits = crate::engine::search_v3::SearchLimits {
            max_depth: Some(depth),
            max_time: Some(Duration::from_secs(5)),
            ..Default::default()
        };

        let best_move = self.search_v3.search(pos, limits);
        let search_eval = best_move.map(|_| static_eval); // Simplified for now

        // Get top moves
        let top_moves = self.get_top_moves(pos, 5, depth);

        // Evaluation breakdown
        let evaluation_breakdown = self.ensemble.as_ref().map(|e| e.evaluate_breakdown(pos));

        // Tactical features
        let tactical_features = self.analyze_tactical_features(pos);

        // Positional features
        let positional_features = self.analyze_positional_features(pos);

        PositionAnalysis {
            static_eval,
            search_eval,
            best_move,
            best_line: Vec::new(), // TODO: Extract from PV
            top_moves,
            evaluation_breakdown,
            tactical_features,
            positional_features,
        }
    }

    /// Get top N moves with their evaluations
    fn get_top_moves(&mut self, pos: &Position, n: usize, _depth: u8) -> Vec<(Move, i32)> {
        let moves = MoveGen::generate_legal_moves(pos);
        let mut scored_moves = Vec::new();

        for mv in moves.into_iter().take(n * 2) {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_ok() {
                let score = -EvaluatorV2::evaluate(&new_pos);
                scored_moves.push((mv, score));
            }
        }

        scored_moves.sort_by(|a, b| b.1.cmp(&a.1));
        scored_moves.truncate(n);
        scored_moves
    }

    /// Analyze tactical features
    fn analyze_tactical_features(&self, pos: &Position) -> TacticalFeatures {
        let moves = MoveGen::generate_legal_moves(pos);

        let checks_available = moves.iter()
            .filter(|mv| {
                let mut new_pos = pos.clone();
                new_pos.make_move(**mv).ok().map_or(false, |_| new_pos.is_check())
            })
            .count();

        let captures_available = moves.iter()
            .filter(|mv| mv.is_capture())
            .count();

        TacticalFeatures {
            checks_available,
            captures_available,
            threatened_pieces: Vec::new(), // TODO: Implement
            hanging_pieces: Vec::new(),     // TODO: Implement
            pins: Vec::new(),               // TODO: Implement
            forks: Vec::new(),              // TODO: Implement
        }
    }

    /// Analyze positional features
    fn analyze_positional_features(&self, pos: &Position) -> PositionalFeatures {
        // Use evaluator components
        let static_eval = EvaluatorV2::evaluate(pos);

        PositionalFeatures {
            material_balance: static_eval / 10, // Rough estimate
            piece_activity: 0,    // TODO: Calculate from mobility
            pawn_structure_score: 0, // TODO: Extract from evaluator
            king_safety: 0,       // TODO: Extract from evaluator
            center_control: 0,    // TODO: Calculate
        }
    }

    /// Compare move quality against best move
    pub fn analyze_move_quality(&mut self, pos: &Position, chosen_move: Move, depth: u8) -> MoveQuality {
        let analysis = self.analyze_position(pos, depth);

        let best_eval = analysis.search_eval.unwrap_or(analysis.static_eval);

        // Evaluate the chosen move
        let mut new_pos = pos.clone();
        let chosen_eval = if new_pos.make_move(chosen_move).is_ok() {
            -EvaluatorV2::evaluate(&new_pos)
        } else {
            -10000 // Illegal move
        };

        let eval_loss = best_eval - chosen_eval;

        let classification = if eval_loss <= 10 {
            MoveClassification::Excellent
        } else if eval_loss <= 50 {
            MoveClassification::Good
        } else if eval_loss <= 100 {
            MoveClassification::Inaccuracy
        } else if eval_loss <= 300 {
            MoveClassification::Mistake
        } else {
            MoveClassification::Blunder
        };

        MoveQuality {
            chosen_move,
            chosen_eval,
            best_move: analysis.best_move,
            best_eval,
            eval_loss,
            classification,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveQuality {
    pub chosen_move: Move,
    pub chosen_eval: i32,
    pub best_move: Option<Move>,
    pub best_eval: i32,
    pub eval_loss: i32,
    pub classification: MoveClassification,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveClassification {
    Excellent,    // <= 10cp loss
    Good,         // <= 50cp loss
    Inaccuracy,   // <= 100cp loss
    Mistake,      // <= 300cp loss
    Blunder,      // > 300cp loss
}

impl MoveQuality {
    pub fn to_string(&self) -> String {
        let mut result = format!("Move: {}\n", self.chosen_move.to_uci());
        result.push_str(&format!("Classification: {:?}\n", self.classification));
        result.push_str(&format!("Eval: {} cp (loss: {} cp)\n",
                                 self.chosen_eval, self.eval_loss));

        if let Some(best) = self.best_move {
            result.push_str(&format!("Best move: {} ({} cp)\n",
                                     best.to_uci(), self.best_eval));
        }

        result
    }
}

/// Game analysis
pub struct GameAnalysis {
    pub moves: Vec<Move>,
    pub move_qualities: Vec<MoveQuality>,
    pub average_cpl: f32,  // Average centipawn loss
    pub accuracy: f32,      // Percentage
    pub blunders: usize,
    pub mistakes: usize,
    pub inaccuracies: usize,
}

impl GameAnalysis {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            move_qualities: Vec::new(),
            average_cpl: 0.0,
            accuracy: 0.0,
            blunders: 0,
            mistakes: 0,
            inaccuracies: 0,
        }
    }

    pub fn add_move_quality(&mut self, quality: MoveQuality) {
        match quality.classification {
            MoveClassification::Blunder => self.blunders += 1,
            MoveClassification::Mistake => self.mistakes += 1,
            MoveClassification::Inaccuracy => self.inaccuracies += 1,
            _ => {},
        }

        self.move_qualities.push(quality);
        self.recalculate();
    }

    fn recalculate(&mut self) {
        if self.move_qualities.is_empty() {
            return;
        }

        let total_loss: i32 = self.move_qualities.iter()
            .map(|q| q.eval_loss)
            .sum();

        self.average_cpl = total_loss as f32 / self.move_qualities.len() as f32;

        // Simple accuracy calculation: 100% - (average_cpl / 3)
        self.accuracy = (100.0 - (self.average_cpl / 3.0)).max(0.0);
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("Game Analysis:\n");
        result.push_str(&format!("  Moves analyzed: {}\n", self.move_qualities.len()));
        result.push_str(&format!("  Average CPL: {:.1}\n", self.average_cpl));
        result.push_str(&format!("  Accuracy: {:.1}%\n", self.accuracy));
        result.push_str(&format!("  Blunders: {}\n", self.blunders));
        result.push_str(&format!("  Mistakes: {}\n", self.mistakes));
        result.push_str(&format!("  Inaccuracies: {}\n", self.inaccuracies));
        result
    }
}
