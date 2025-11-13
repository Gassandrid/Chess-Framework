use crate::engine::{
    position::Position,
    evaluation_v2::EvaluatorV2,
    nn_eval::{NeuralNetwork, evaluate_nn},
    nn_eval_deep::{DeepNeuralNetwork, evaluate_deep_nn},
};

/// Ensemble evaluation combining multiple approaches
pub struct EnsembleEvaluator {
    classical_weight: f32,
    nn_weight: f32,
    deep_nn_weight: f32,
    nn: Option<NeuralNetwork>,
    deep_nn: Option<DeepNeuralNetwork>,
}

impl EnsembleEvaluator {
    pub fn new() -> Self {
        Self {
            classical_weight: 0.5,
            nn_weight: 0.3,
            deep_nn_weight: 0.2,
            nn: None,
            deep_nn: None,
        }
    }

    pub fn with_weights(classical: f32, nn: f32, deep_nn: f32) -> Self {
        let total = classical + nn + deep_nn;
        Self {
            classical_weight: classical / total,
            nn_weight: nn / total,
            deep_nn_weight: deep_nn / total,
            nn: None,
            deep_nn: None,
        }
    }

    pub fn load_nn(&mut self, path: &str) -> std::io::Result<()> {
        self.nn = Some(NeuralNetwork::load(path)?);
        Ok(())
    }

    pub fn set_nn(&mut self, nn: NeuralNetwork) {
        self.nn = Some(nn);
    }

    pub fn set_deep_nn(&mut self, nn: DeepNeuralNetwork) {
        self.deep_nn = Some(nn);
    }

    /// Evaluate position using ensemble of methods
    pub fn evaluate(&self, pos: &Position) -> i32 {
        let mut total_score = 0.0;

        // Classical evaluation
        let classical_score = EvaluatorV2::evaluate(pos) as f32;
        total_score += classical_score * self.classical_weight;

        // Shallow NN evaluation
        if let Some(ref nn) = self.nn {
            let nn_score = evaluate_nn(pos, nn) as f32;
            total_score += nn_score * self.nn_weight;
        } else {
            // Fall back to classical if NN not loaded
            total_score += classical_score * self.nn_weight;
        }

        // Deep NN evaluation
        if let Some(ref deep_nn) = self.deep_nn {
            let deep_score = evaluate_deep_nn(pos, deep_nn) as f32;
            total_score += deep_score * self.deep_nn_weight;
        } else {
            // Fall back to classical if deep NN not loaded
            total_score += classical_score * self.deep_nn_weight;
        }

        total_score as i32
    }

    /// Get breakdown of evaluation components
    pub fn evaluate_breakdown(&self, pos: &Position) -> EvaluationBreakdown {
        let classical = EvaluatorV2::evaluate(pos);
        let nn = self.nn.as_ref().map(|n| evaluate_nn(pos, n));
        let deep_nn = self.deep_nn.as_ref().map(|n| evaluate_deep_nn(pos, n));

        let final_score = self.evaluate(pos);

        EvaluationBreakdown {
            classical_score: classical,
            nn_score: nn,
            deep_nn_score: deep_nn,
            final_score,
            weights: (self.classical_weight, self.nn_weight, self.deep_nn_weight),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvaluationBreakdown {
    pub classical_score: i32,
    pub nn_score: Option<i32>,
    pub deep_nn_score: Option<i32>,
    pub final_score: i32,
    pub weights: (f32, f32, f32),
}

impl EvaluationBreakdown {
    pub fn to_string(&self) -> String {
        let mut result = format!("Final Score: {} cp\n", self.final_score);
        result.push_str(&format!("  Classical (weight {:.2}): {} cp\n",
                                 self.weights.0, self.classical_score));

        if let Some(nn_score) = self.nn_score {
            result.push_str(&format!("  Shallow NN (weight {:.2}): {} cp\n",
                                     self.weights.1, nn_score));
        }

        if let Some(deep_score) = self.deep_nn_score {
            result.push_str(&format!("  Deep NN (weight {:.2}): {} cp\n",
                                     self.weights.2, deep_score));
        }

        result
    }
}
