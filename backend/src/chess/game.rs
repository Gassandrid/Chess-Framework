use crate::chess::board::Board;
use crate::chess::moves::{Move, MoveResult};
use crate::chess::piece::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameStatus {
    Active,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub current_player: Color,
    pub status: GameStatus,
    pub move_history: Vec<(Move, MoveResult)>,
    // Additional state like castling rights, en passant target, etc. would be here
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_standard(),
            current_player: Color::White,
            status: GameStatus::Active,
            move_history: Vec::new(),
        }
    }

    // Make a move on the board
    pub fn make_move(&mut self, chess_move: &Move) -> Result<MoveResult, &'static str> {
        // In a real implementation, we would:
        // 1. Validate the move
        // 2. Check if it's the current player's turn
        // 3. Apply the move to the board
        // 4. Update game state (check, checkmate, etc.)
        // 5. Switch players

        // For this skeleton, we'll just do a simple implementation

        let from_piece = self
            .board
            .get_piece(chess_move.from)
            .ok_or("No piece at source position")?;

        if from_piece.color != self.current_player {
            return Err("Not your turn");
        }

        let to_piece = self.board.get_piece(chess_move.to);
        let result = if let Some(captured) = to_piece {
            if captured.color == self.current_player {
                return Err("Cannot capture your own piece");
            }
            MoveResult::Capture(captured)
        } else {
            MoveResult::Normal
        };

        // Apply the move
        self.board.set_piece(chess_move.to, Some(from_piece));
        self.board.set_piece(chess_move.from, None);

        // Handle promotion
        if let Some(promotion_type) = chess_move.promotion {
            self.board.set_piece(
                chess_move.to,
                Some(crate::chess::piece::Piece::new(
                    promotion_type,
                    self.current_player,
                )),
            );
        }

        // Record the move
        self.move_history.push((chess_move.clone(), result.clone()));

        // Switch players
        self.current_player = self.current_player.opposite();

        // In a real implementation, we would check for check, checkmate, etc. here
        // and update self.status accordingly

        Ok(result)
    }

    // Undo the last move
    pub fn undo_move(&mut self) -> Result<(), &'static str> {
        let (last_move, _) = self.move_history.pop().ok_or("No moves to undo")?;

        // Switch back to the previous player
        self.current_player = self.current_player.opposite();

        // In a real implementation, we would restore the board state
        // For this skeleton, we'll just reset the board to the initial position if there are no moves left
        if self.move_history.is_empty() {
            self.board = Board::new_standard();
        } else {
            // In a real implementation, we would restore the previous board state
            // This is a simplified approach
            let mut piece = self
                .board
                .get_piece(last_move.to)
                .ok_or("No piece at destination")?;

            // If it was a promotion, restore the pawn
            if last_move.promotion.is_some() {
                piece = crate::chess::piece::Piece::new(
                    crate::chess::piece::PieceType::Pawn,
                    self.current_player,
                );
            }

            // Move the piece back
            self.board.set_piece(last_move.from, Some(piece));
            self.board.set_piece(last_move.to, None);

            // If it was a capture, restore the captured piece
            // (In a real implementation, we would store this information)
        }

        // Reset game status
        self.status = GameStatus::Active;

        Ok(())
    }
}
