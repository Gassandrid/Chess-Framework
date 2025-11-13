use chess_engine_api::engine::{perft::Perft, position::Position};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  {} test           - Run perft test suite", args[0]);
        println!("  {} <depth>        - Run perft on starting position", args[0]);
        println!("  {} <depth> <fen>  - Run perft on FEN position", args[0]);
        return;
    }

    if args[1] == "test" {
        Perft::test_suite();
        return;
    }

    let depth: u8 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            println!("Invalid depth");
            return;
        }
    };

    let position = if args.len() > 2 {
        let fen = args[2..].join(" ");
        match Position::from_fen(&fen) {
            Ok(pos) => pos,
            Err(e) => {
                println!("Error parsing FEN: {}", e);
                return;
            }
        }
    } else {
        Position::startpos()
    };

    println!("Position: {}", position.to_fen());
    Perft::divide(&position, depth);
}
