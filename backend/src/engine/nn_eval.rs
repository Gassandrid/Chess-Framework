use ndarray::{Array1, Array2};
use rand_distr::{Distribution, Normal};
use crate::engine::position::{Position, PieceType, Color};
use std::fs::File;
use std::io::{Write as IoWrite, Read};

const INPUT_SIZE: usize = 768; // 12 piece types * 64 squares
const HIDDEN1_SIZE: usize = 256;
const HIDDEN2_SIZE: usize = 128;
const OUTPUT_SIZE: usize = 1;

pub struct NeuralNetwork {
    // Layer weights and biases
    w1: Array2<f32>,
    b1: Array1<f32>,
    w2: Array2<f32>,
    b2: Array1<f32>,
    w3: Array2<f32>,
    b3: Array1<f32>,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        // Initialize weights with He initialization
        let he_scale_1 = (2.0 / INPUT_SIZE as f32).sqrt();
        let he_scale_2 = (2.0 / HIDDEN1_SIZE as f32).sqrt();
        let he_scale_3 = (2.0 / HIDDEN2_SIZE as f32).sqrt();

        Self {
            w1: Array2::from_shape_fn((INPUT_SIZE, HIDDEN1_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he_scale_1
            }),
            b1: Array1::zeros(HIDDEN1_SIZE),
            w2: Array2::from_shape_fn((HIDDEN1_SIZE, HIDDEN2_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he_scale_2
            }),
            b2: Array1::zeros(HIDDEN2_SIZE),
            w3: Array2::from_shape_fn((HIDDEN2_SIZE, OUTPUT_SIZE), |_| {
                normal.sample(&mut rng) as f32 * he_scale_3
            }),
            b3: Array1::zeros(OUTPUT_SIZE),
        }
    }

    pub fn forward(&self, input: &Array1<f32>) -> f32 {
        // Layer 1: input -> hidden1
        let z1 = input.dot(&self.w1) + &self.b1;
        let a1 = z1.mapv(Self::relu);

        // Layer 2: hidden1 -> hidden2
        let z2 = a1.dot(&self.w2) + &self.b2;
        let a2 = z2.mapv(Self::relu);

        // Layer 3: hidden2 -> output
        let z3 = a2.dot(&self.w3) + &self.b3;

        // Output (linear activation)
        z3[0]
    }

    fn relu(x: f32) -> f32 {
        x.max(0.0)
    }

    fn relu_derivative(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else { 0.0 }
    }

    pub fn train(&mut self, inputs: &[Array1<f32>], targets: &[f32], epochs: usize, learning_rate: f32) {
        let batch_size = inputs.len();

        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for (input, &target) in inputs.iter().zip(targets.iter()) {
                // Forward pass
                let z1 = input.dot(&self.w1) + &self.b1;
                let a1 = z1.mapv(Self::relu);

                let z2 = a1.dot(&self.w2) + &self.b2;
                let a2 = z2.mapv(Self::relu);

                let z3 = a2.dot(&self.w3) + &self.b3;
                let output = z3[0];

                // Compute loss (MSE)
                let error = output - target;
                total_loss += error * error;

                // Backward pass
                // Output layer gradient
                let dz3 = Array1::from_vec(vec![error]);

                // Hidden layer 2 gradients
                let dw3 = a2.insert_axis(ndarray::Axis(1)).dot(&dz3.view().insert_axis(ndarray::Axis(0)));
                let db3 = dz3.to_owned();
                let da2 = dz3.dot(&self.w3.t());
                let dz2 = &da2 * &z2.mapv(Self::relu_derivative);

                // Hidden layer 1 gradients
                let dw2 = a1.insert_axis(ndarray::Axis(1)).dot(&dz2.view().insert_axis(ndarray::Axis(0)));
                let db2 = dz2.to_owned();
                let da1 = dz2.dot(&self.w2.t());
                let dz1 = &da1 * &z1.mapv(Self::relu_derivative);

                // Input layer gradients
                let dw1 = input.view().insert_axis(ndarray::Axis(1)).dot(&dz1.view().insert_axis(ndarray::Axis(0)));
                let db1 = dz1.to_owned();

                // Update weights and biases
                self.w3 -= &(dw3 * learning_rate);
                self.b3 -= &(db3 * learning_rate);
                self.w2 -= &(dw2 * learning_rate);
                self.b2 -= &(db2 * learning_rate);
                self.w1 -= &(dw1 * learning_rate);
                self.b1 -= &(db1 * learning_rate);
            }

            if epoch % 100 == 0 {
                println!("Epoch {}: Average loss = {}", epoch, total_loss / batch_size as f32);
            }
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        // Save dimensions
        file.write_all(&INPUT_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN1_SIZE.to_le_bytes())?;
        file.write_all(&HIDDEN2_SIZE.to_le_bytes())?;
        file.write_all(&OUTPUT_SIZE.to_le_bytes())?;

        // Save weights and biases
        for &val in self.w1.iter() {
            file.write_all(&val.to_le_bytes())?;
        }
        for &val in self.b1.iter() {
            file.write_all(&val.to_le_bytes())?;
        }
        for &val in self.w2.iter() {
            file.write_all(&val.to_le_bytes())?;
        }
        for &val in self.b2.iter() {
            file.write_all(&val.to_le_bytes())?;
        }
        for &val in self.w3.iter() {
            file.write_all(&val.to_le_bytes())?;
        }
        for &val in self.b3.iter() {
            file.write_all(&val.to_le_bytes())?;
        }

        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut offset = 0;

        // Read dimensions
        let _input_size = usize::from_le_bytes(buffer[offset..offset+8].try_into().unwrap());
        offset += 8;
        let _hidden1_size = usize::from_le_bytes(buffer[offset..offset+8].try_into().unwrap());
        offset += 8;
        let _hidden2_size = usize::from_le_bytes(buffer[offset..offset+8].try_into().unwrap());
        offset += 8;
        let _output_size = usize::from_le_bytes(buffer[offset..offset+8].try_into().unwrap());
        offset += 8;

        // Read weights and biases
        let mut w1_data = vec![0.0f32; INPUT_SIZE * HIDDEN1_SIZE];
        for val in &mut w1_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let w1 = Array2::from_shape_vec((INPUT_SIZE, HIDDEN1_SIZE), w1_data).unwrap();

        let mut b1_data = vec![0.0f32; HIDDEN1_SIZE];
        for val in &mut b1_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let b1 = Array1::from_vec(b1_data);

        let mut w2_data = vec![0.0f32; HIDDEN1_SIZE * HIDDEN2_SIZE];
        for val in &mut w2_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let w2 = Array2::from_shape_vec((HIDDEN1_SIZE, HIDDEN2_SIZE), w2_data).unwrap();

        let mut b2_data = vec![0.0f32; HIDDEN2_SIZE];
        for val in &mut b2_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let b2 = Array1::from_vec(b2_data);

        let mut w3_data = vec![0.0f32; HIDDEN2_SIZE * OUTPUT_SIZE];
        for val in &mut w3_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let w3 = Array2::from_shape_vec((HIDDEN2_SIZE, OUTPUT_SIZE), w3_data).unwrap();

        let mut b3_data = vec![0.0f32; OUTPUT_SIZE];
        for val in &mut b3_data {
            *val = f32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
            offset += 4;
        }
        let b3 = Array1::from_vec(b3_data);

        Ok(Self { w1, b1, w2, b2, w3, b3 })
    }
}

// Convert position to neural network input
pub fn position_to_input(pos: &Position) -> Array1<f32> {
    let mut input = Array1::zeros(INPUT_SIZE);

    // Encode each piece on each square
    for sq in 0..64 {
        if let Some((piece_type, color)) = pos.piece_at(sq) {
            let color_offset = if color == Color::White { 0 } else { 6 };
            let piece_idx = match piece_type {
                PieceType::Pawn => 0,
                PieceType::Knight => 1,
                PieceType::Bishop => 2,
                PieceType::Rook => 3,
                PieceType::Queen => 4,
                PieceType::King => 5,
            };
            let feature_idx = (color_offset + piece_idx) * 64 + sq as usize;
            input[feature_idx] = 1.0;
        }
    }

    input
}

// Evaluate position using neural network
pub fn evaluate_nn(pos: &Position, nn: &NeuralNetwork) -> i32 {
    let input = position_to_input(pos);
    let score = nn.forward(&input);

    // Convert from normalized score to centipawns
    (score * 100.0) as i32
}
