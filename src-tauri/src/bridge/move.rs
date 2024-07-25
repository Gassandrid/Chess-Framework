// this will return a list of all legal moves for the model to use
// might make it take no argments
pub fn get_legal_moves(board: Board) -> Vec<Move> {}

// frontend binding to get the best move the AI has come up with to that point
pub fn get_best_move() -> Move {}

// bindings to tell backend if game is over
pub fn model_checkmate() -> bool {}

pub fn model_stalemate() -> bool {}

pub fn enemy_checkmate() -> bool {}

pub fn enemy_stalemate() -> bool {}

pub fn draw() -> bool {}
