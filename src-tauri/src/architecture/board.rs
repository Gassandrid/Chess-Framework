use crate::architecture::piece::Piece;

//board struct, contains the board and the pieces
pub struct Board {
    pub pieces: [u8; 64],
}

// example starter board
impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [
                Piece::ROOK | Piece::WHITE,
                Piece::KNIGHT | Piece::WHITE,
                Piece::BISHOP | Piece::WHITE,
                Piece::QUEEN | Piece::WHITE,
                Piece::KING | Piece::WHITE,
                Piece::BISHOP | Piece::WHITE,
                Piece::KNIGHT | Piece::WHITE,
                Piece::ROOK | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                // empty squares
                for _ in 16..48 {
                    Piece::NONE
                },
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::ROOK | Piece::BLACK,
                Piece::KNIGHT | Piece::BLACK,
                Piece::BISHOP | Piece::BLACK,
                Piece::QUEEN | Piece::BLACK,
                Piece::KING | Piece::BLACK,
                Piece::BISHOP | Piece::BLACK,
                Piece::KNIGHT | Piece::BLACK,
                Piece::ROOK | Piece::BLACK,
            ],
        }
    }
}
