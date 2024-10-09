// gettign a certain pice is done by getting the bitwise OR of the type and the color
// for example, to get a white king, you would do Piece::KING | Piece::white
// for an empty square, you would do Piece::NONE
pub struct Piece {
    pub value: u8,
}
impl Piece {
    pub const NONE: u8 = 0;
    pub const KING: u8 = 1;
    pub const PAWN: u8 = 2;
    pub const KNIGHT: u8 = 3;
    pub const BISHOP: u8 = 4;
    pub const ROOK: u8 = 5;
    pub const QUEEN: u8 = 6;

    pub const WHITE: u8 = 8;
    pub const BLACK: u8 = 16;
}
