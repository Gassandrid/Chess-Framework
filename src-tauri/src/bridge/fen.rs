// for translating to/from FEN strings
// FEN strings are used to represent the state of a chess game
use crate::models::architecture;

impl Model {
    pub fn apply_fen_to_board(fen: &str) {
        let mut board = Board::new();
        let mut moves = Vec::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = Piece::from_fen_char(c);
                board.set_piece(Position::new(rank, file), piece);
                file += 1;
            }
        }
        self.board = board;
    }
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let piece = self.board.get_piece(Position::new(rank, file));
                if piece.piece_type == PieceType::None {
                    empty += 1;
                } else {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(piece.to_fen_char());
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen
    }
}
