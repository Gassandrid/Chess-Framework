use ndarray::{Array1, Array2};
use rand_distr::{Distribution, Normal};
use crate::engine::position::{Position, PieceType, Color};
use crate::engine::nn_eval::position_to_input;
use std::fs::File;
use std::io::{Write as IoWrite, Read};

const INPUT_SIZE: usize = 768;
const HIDDEN1_SIZE: usize = 512;  // Larger first layer
const HIDDEN2_SIZE: usize = 256;
const HIDDEN3_SIZE: usize = 128;  // Extra hidden layer
const HIDDEN4_SIZE: usize = 64;   // Another layer
const OUTPUT_SIZE: usize = 1;

/// Deeper neural network with more layers for complex pattern recognition
pub struct DeepNeuralNetwork {
    w1: Array2<f32>,
    b1: Array1<f32>,
    w2: Array2<f32>,
    b2: Array1<f32>,
    w3: Array2<f32>,
    b3: Array1<f32>,
    w4: Array2<f32>,
    b4: Array1<f32>,
    w5: Array2<f32>,
    b5: Array1<f32>,
}

impl DeepNeuralNetwork {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        // He initialization
        let he1 = (2.0 / INPUT_SIZE as f32).sqrt();
        let he2 = (2.0 / HIDDEN1_SIZE as f32).sqrt();
        let he3 = (2.0 / HIDDEN2_SIZE as f32).sqrt();
        let he4 = (2.0 / HIDDEN3_SIZE as f32).sqrt();
        let he5 = (2.0 / HIDDEN4_SIZE as f32).sqrt();

        Self {
            w1: Array2::from_shape_fn((INPUT_SIZE, HIDDEN1_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he1
            }),
            b1: Array1::zeros(HIDDEN1_SIZE),
            w2: Array2::from_shape_fn((HIDDEN1_SIZE, HIDDEN2_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he2
            }),
            b2: Array1::zeros(HIDDEN2_SIZE),
            w3: Array2::from_shape_fn((HIDDEN2_SIZE, HIDDEN3_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he3
            }),
            b3: Array1::zeros(HIDDEN3_SIZE),
            w4: Array2::from_shape_fn((HIDDEN3_SIZE, HIDDEN4_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he4
            }),
            b4: Array1::zeros(HIDDEN4_SIZE),
            w5: Array2::from_shape_fn((HIDDEN4_SIZE, OUTPUT_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he5
            }),
            b5: Array1::zeros(OUTPUT_SIZE),
        }
    }

    pub fn forward(&self, input: &Array1<f32>) -> f32 {
        // Layer 1
        let z1 = input.dot(&self.w1) + &self.b1;
        let a1 = z1.mapv(Self::relu);

        // Layer 2
        let z2 = a1.dot(&self.w2) + &self.b2;
        let a2 = z2.mapv(Self::relu);

        // Layer 3
        let z3 = a2.dot(&self.w3) + &self.b3;
        let a3 = z3.mapv(Self::relu);

        // Layer 4
        let z4 = a3.dot(&self.w4) + &self.b4;
        let a4 = z4.mapv(Self::relu);

        // Output layer
        let z5 = a4.dot(&self.w5) + &self.b5;
        z5[0]
    }

    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        // Save dimensions
        file.write_all(&INPUT_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN1_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN2_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN3_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN4_SIZE.to_le_bytes())?;
        file.write_all(&OUTPUT_SIZE.to_le_bytes())?;

        // Save all weights and biases
        for &val in self.w1.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.b1.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.w2.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.b2.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.w3.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.b3.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.w4.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.b4.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.w5.iter() { file.write_all(&val.to_le_bytes())?; }
        for &val in self.b5.iter() { file.write_all(&val.to_le_bytes())?; }

        Ok(())
    }
}

/// Evaluate position using deep neural network
pub fn evaluate_deep_nn(pos: &Position, nn: &DeepNeuralNetwork) -> i32 {
    let input = position_to_input(pos);
    let score = nn.forward(&input);
    (score * 100.0) as i32
}
