// main parent struct

pub struct Model {}
    pub board: Board,
    pub moves: Vec<Move>,
}
// struct for a chess piece

// pub trait PieceFunctions {
//     fn get_color(&self) -> Color;
//     fn get_position(&self) -> Position;
//     fn get_type(&self) -> PieceType;
//     fn get_moves(&self) -> Vec<Move>;
//     fn move_piece(&self, end_pos: Position) -> Move;
// }
// 
// 
// 
// pub struct Move {
//     pub start_pos: Position,
//     pub end_pos: Position,
//     pub piece: Piece,
//     pub move_type: MoveType,
// }

// pieces on the board will be representend as an 8 bit integer, first 3 bits will be the color,
// next 2 will be the color
// only 3 of the bits wil be used

// enum for readablility, but under the hood it will be an 8 bit integer
pub enum PieceType {
    None,
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub enum Color {
    White,
    Black,
}

pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    // piece constants
    pub fn none() -> u8 {
        0
    }
    pub fn king() -> u8 {
        1
    }
    pub fn pawn() -> u8 {
        2
    }
    pub fn knight() -> u8 {
        3
    }
    pub fn bishop() -> u8 {
        4
    }
    pub fn rook() -> u8 {
        5
    }
    pub fn queen() -> u8 {
        6
    }

    // type constants
    pub fn white() -> u8 {
        8
    }
    pub fn black() -> u8 {
        16
    }

    // translation functions
    pub fn piece_to_int() -> u8 {

        let type: u8 = match self.piece_type {
            PieceType::None => 0,
            PieceType::King => 1,
            PieceType::Pawn => 2,
            PieceType::Knight => 3,
            PieceType::Bishop => 4,
            PieceType::Rook => 5,
            PieceType::Queen => 6,
        };

        let color: u8 = match self.color {
            Color::White => 8,
            Color::Black => 16,
        };

        (type + color)
    }

}


pub struct Board {
    pub board: [[u8; 8]; 8],
}
