// struct for a chess piece

// piece structure
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position,
}

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub enum Color {
    White,
    Black,
}

pub struct Position {
    pub x: u8,
    pub y: u8,
}


// move structure
pub struct Move {}
    pub piece: Piece,
    pub start_pos: Position,
    pub end_pos: Position,
    pub is_capture: bool,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub is_draw: bool,
}

// board structure
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
}
