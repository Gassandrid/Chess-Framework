use crate::architecture::board::Board;
// since board is a single vector, moving up/down/left/right is either 8, -8, 1, or -1
// we also have diagonals, which are 7, -7, 9, and -9
// enum to hold constant values for the directions
pub enum DirectionOffsets {
    UP = 8,
    DOWN = -8,
    LEFT = -1,
    RIGHT = 1,
    UP_LEFT = 7,
    UP_RIGHT = 9,
    DOWN_LEFT = -9,
    DOWN_RIGHT = -7,
}

// a 2d array of for holding the number of squares to the edge of the board
impl Board {
    pub fn get_edge_distances(&self) -> [[i8; 64]; 64] {
        let mut edge_distances = [[0; 64]; 64];
        for i in 0..64 {
            for j in 0..64 {
                edge_distances[i][j] = 0;
                let mut i_diff = i as i8 % 8 - j as i8 % 8;
                let mut j_diff = i as i8 / 8 - j as i8 / 8;
                if i_diff < 0 {
                    i_diff = -i_diff;
                }
                if j_diff < 0 {
                    j_diff = -j_diff;
                }
                edge_distances[i][j] = i_diff + j_diff;
            }
        }
        edge_distances
    }
}

// declare a constant 2d array

// // this will return a list of all legal moves for the model to use
// // might make it take no argments
// pub fn get_legal_moves(board: Board) -> Vec<Move> {}
//
// // frontend binding to get the best move the AI has come up with to that point
// pub fn get_best_move() -> Move {}
//
// // bindings to tell backend if game is over
// pub fn model_checkmate() -> bool {}
//
// pub fn model_stalemate() -> bool {}
//
// pub fn enemy_checkmate() -> bool {}
//
// pub fn enemy_stalemate() -> bool {}
//
// pub fn draw() -> bool {}
//
