// struct for a chess piece

pub trait PieceFunctions {
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Position;
    fn get_type(&self) -> PieceType;
    fn get_moves(&self) -> Vec<Move>;
    fn move_piece(&self, end_pos: Position) -> Move;
}

pub struct King {}

pub struct Queen {}

pub struct Rook {}

pub struct Bishop {}

pub struct Knight {}

pub struct Pawn {}

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

pub enum PieceColor {
    White,
    Black,
    None,
}

pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: Position,
}

pub enum MoveType {
    Capture,
    Move,
    Check,
    Checkmate,
    Stalemate,
}

pub struct Move {
    pub start_pos: Position,
    pub end_pos: Position,
    pub piece: Piece,
    pub move_type: MoveType,
}

pub struct Board {
    pub board: Vec<Vec<Piece>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Vec::new();

        // first row of black pieces
        board.push(vec![
            Piece {
                piece_type: PieceType::Rook,
                color: PieceColor::Black,
                position: Position { x: 0, y: 7 },
            },
            Piece {
                piece_type: PieceType::Knight,
                color: PieceColor::Black,
                position: Position { x: 1, y: 7 },
            },
            Piece {
                piece_type: PieceType::Bishop,
                color: PieceColor::Black,
                position: Position { x: 2, y: 7 },
            },
            Piece {
                piece_type: PieceType::Queen,
                color: PieceColor::Black,
                position: Position { x: 3, y: 7 },
            },
            Piece {
                piece_type: PieceType::King,
                color: PieceColor::Black,
                position: Position { x: 4, y: 7 },
            },
            Piece {
                piece_type: PieceType::Bishop,
                color: PieceColor::Black,
                position: Position { x: 5, y: 7 },
            },
            Piece {
                piece_type: PieceType::Knight,
                color: PieceColor::Black,
                position: Position { x: 6, y: 7 },
            },
            Piece {
                piece_type: PieceType::Rook,
                color: PieceColor::Black,
                position: Position { x: 7, y: 7 },
            },
        ]);

        // second row of black pieces
        board.push(vec![
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 0, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 1, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 2, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 3, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 4, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 5, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 6, y: 6 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
                position: Position { x: 7, y: 6 },
            },
        ]);

        // empty rows
        // row 3
        board.push(vec![
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 0, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 1, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 2, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 3, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 4, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 5, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 6, y: 5 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 7, y: 5 },
            },
        ]);

        // row 4
        board.push(vec![
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 0, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 1, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 2, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 3, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 4, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 5, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 6, y: 4 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 7, y: 4 },
            },
        ]);

        // row 5
        board.push(vec![
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 0, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 1, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 2, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 3, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 4, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 5, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 6, y: 3 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 7, y: 3 },
            },
        ]);

        // row 6
        board.push(vec![
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 0, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 1, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 2, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 3, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 4, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 5, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 6, y: 2 },
            },
            Piece {
                piece_type: PieceType::None,
                color: PieceColor::None,
                position: Position { x: 7, y: 2 },
            },
        ]);

        // whites pawns
        board.push(vec![
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 0, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 1, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 2, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 3, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 4, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 5, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 6, y: 1 },
            },
            Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
                position: Position { x: 7, y: 1 },
            },
        ]);

        // white pieces
        board.push(vec![
            Piece {
                piece_type: PieceType::Rook,
                color: PieceColor::White,
                position: Position { x: 0, y: 0 },
            },
            Piece {
                piece_type: PieceType::Knight,
                color: PieceColor::White,
                position: Position { x: 1, y: 0 },
            },
            Piece {
                piece_type: PieceType::Bishop,
                color: PieceColor::White,
                position: Position { x: 2, y: 0 },
            },
            Piece {
                piece_type: PieceType::Queen,
                color: PieceColor::White,
                position: Position { x: 3, y: 0 },
            },
            Piece {
                piece_type: PieceType::King,
                color: PieceColor::White,
                position: Position { x: 4, y: 0 },
            },
            Piece {
                piece_type: PieceType::Bishop,
                color: PieceColor::White,
                position: Position { x: 5, y: 0 },
            },
            Piece {
                piece_type: PieceType::Knight,
                color: PieceColor::White,
                position: Position { x: 6, y: 0 },
            },
            Piece {
                piece_type: PieceType::Rook,
                color: PieceColor::White,
                position: Position { x: 7, y: 0 },
            },
        ]);

        Board { board }
    }
}
