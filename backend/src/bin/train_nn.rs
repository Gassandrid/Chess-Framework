use chess_engine_api::engine::{
    position::Position,
    movegen::MoveGen,
    search_v2::SearchV2,
    search_v2::SearchLimits,
    nn_eval::{NeuralNetwork, position_to_input},
    evaluation_v2::EvaluatorV2,
};
use std::time::Duration;
use ndarray::Array1;

fn main() {
    println!("Neural Network Training for Chess Evaluation");
    println!("============================================\n");

    // Generate training data from self-play
    println!("Generating training data from self-play...");
    let (inputs, targets) = generate_training_data(500, 5); // 500 positions, depth 5

    println!("Generated {} training positions", inputs.len());

    // Create and train neural network
    println!("\nTraining neural network...");
    let mut nn = NeuralNetwork::new();

    let learning_rate = 0.001;
    let epochs = 1000;

    nn.train(&inputs, &targets, epochs, learning_rate);

    // Save the trained network
    println!("\nSaving trained network...");
    if let Err(e) = nn.save("chess_nn.bin") {
        eprintln!("Error saving network: {}", e);
    } else {
        println!("Network saved to chess_nn.bin");
    }

    // Test the network on a few positions
    println!("\nTesting network on sample positions:");
    test_network(&nn);

    println!("\nTraining complete!");
}

fn generate_training_data(num_positions: usize, search_depth: u8) -> (Vec<Array1<f32>>, Vec<f32>) {
    let mut inputs = Vec::new();
    let mut targets = Vec::new();

    let mut search = SearchV2::new(64);
    let num_games = (num_positions / 30).max(1); // Average 30 positions per game

    for game_num in 0..num_games {
        if game_num % 10 == 0 {
            println!("  Playing game {}/{}...", game_num + 1, num_games);
        }

        let mut pos = Position::startpos();
        let mut move_count = 0;

        loop {
            if move_count >= 60 || inputs.len() >= num_positions {
                break;
            }

            let moves = MoveGen::generate_legal_moves(&pos);
            if moves.is_empty() {
                break;
            }

            // Get evaluation from V2.0 engine
            let eval = EvaluatorV2::evaluate(&pos);

            // Only save positions from non-trivial parts of the game
            if move_count > 5 && move_count < 50 {
                let input = position_to_input(&pos);
                let target = (eval as f32) / 100.0; // Normalize to roughly -10 to +10 range
                inputs.push(input);
                targets.push(target);
            }

            // Make a move using the search
            let limits = SearchLimits {
                max_depth: Some(search_depth),
                max_time: Some(Duration::from_millis(500)),
                ..Default::default()
            };

            if let Some(mv) = search.search(&pos, limits) {
                if pos.make_move(mv).is_err() {
                    break;
                }
            } else {
                break;
            }

            move_count += 1;

            if inputs.len() >= num_positions {
                break;
            }
        }
    }

    (inputs, targets)
}

fn test_network(nn: &NeuralNetwork) {
    let test_positions = vec![
        ("Starting position", Position::startpos()),
        ("After 1.e4", {
            let mut pos = Position::startpos();
            let moves = MoveGen::generate_legal_moves(&pos);
            // Find e2e4
            for mv in moves.into_iter() {
                if mv.from() == 12 && mv.to() == 28 {
                    pos.make_move(mv).ok();
                    break;
                }
            }
            pos
        }),
    ];

    for (name, pos) in test_positions {
        let input = position_to_input(&pos);
        let nn_eval = nn.forward(&input) * 100.0;
        let classical_eval = EvaluatorV2::evaluate(&pos);

        println!("  {}: NN={:.1}, Classical={}", name, nn_eval, classical_eval);
    }
}
