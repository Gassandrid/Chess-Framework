/// Static Exchange Evaluation (SEE)
/// Evaluates the outcome of a capture sequence on a square
use crate::engine::{
    position::{Position, PieceType, Color},
    movegen::Move,
};

/// Piece values for SEE (in centipawns)
const PIECE_VALUES: [i32; 6] = [
    100,   // Pawn
    320,   // Knight
    330,   // Bishop
    500,   // Rook
    900,   // Queen
    20000, // King
];

impl Position {
    /// Static Exchange Evaluation
    /// Returns the material balance after a sequence of captures on the target square
    /// Positive value means the side making the initial capture gains material
    pub fn see(&self, mv: &Move) -> i32 {
        let to = mv.to();
        let from = mv.from();

        // Get the piece moving and the piece being captured
        let moving_piece = match self.piece_at(from as u8) {
            Some(p) => p,
            None => return 0,
        };

        let captured_piece = match self.piece_at(to as u8) {
            Some(p) => p,
            None => {
                // En passant or no capture
                if mv.is_capture() {
                    // En passant
                    (PieceType::Pawn, !self.side_to_move)
                } else {
                    return 0; // Not a capture
                }
            }
        };

        // Start with the value of the captured piece
        let value = Self::piece_value_from_tuple(&captured_piece);

        // Make the capture
        let balance = value - self.see_recursive(to, !self.side_to_move, &moving_piece);

        balance
    }

    /// Recursive SEE calculation
    /// Returns the best material balance the side to move can achieve
    fn see_recursive(&self, square: u8, side: Color, last_attacker: &(PieceType, Color)) -> i32 {
        // Find the least valuable attacker for this side
        let attacker = match self.find_least_valuable_attacker(square, side) {
            Some(a) => a,
            None => return 0, // No more attackers
        };

        // The side can choose to not capture if it would be losing
        let stand_pat = 0;

        // Value of capturing: get the last attacker's value minus what we'd lose
        let last_value = Self::piece_value_from_tuple(last_attacker);
        let capture_value = last_value - self.see_recursive(square, !side, &attacker.piece);

        // Return the better option
        stand_pat.max(capture_value)
    }

    /// Find the least valuable piece attacking a square for a given side
    fn find_least_valuable_attacker(&self, square: u8, side: Color) -> Option<AttackerInfo> {
        let attackers = self.get_attackers(square, side);

        // Find least valuable attacker
        let mut best: Option<AttackerInfo> = None;
        let mut best_value = i32::MAX;

        for attacker in attackers {
            let value = Self::piece_value_from_tuple(&attacker.piece);
            if value < best_value {
                best_value = value;
                best = Some(attacker);
            }
        }

        best
    }

    /// Get all pieces of a given color that attack a square
    fn get_attackers(&self, square: u8, side: Color) -> Vec<AttackerInfo> {
        let mut attackers = Vec::new();

        // Check all pieces of the given side
        for from in 0..64 {
            if let Some((piece_type, color)) = self.piece_at(from) {
                if color == side {
                    // Check if this piece attacks the target square
                    if self.attacks_square(from, square, piece_type, color) {
                        attackers.push(AttackerInfo {
                            from,
                            piece: (piece_type, color),
                        });
                    }
                }
            }
        }

        attackers
    }

    /// Check if a piece on a square attacks another square
    fn attacks_square(&self, from: u8, to: u8, piece_type: PieceType, color: Color) -> bool {
        let from_rank = from / 8;
        let from_file = from % 8;
        let to_rank = to / 8;
        let to_file = to % 8;

        let rank_diff = (to_rank as i8 - from_rank as i8).abs();
        let file_diff = (to_file as i8 - from_file as i8).abs();

        match piece_type {
            PieceType::Pawn => {
                // Pawns attack diagonally
                let direction = if color == Color::White { 1 } else { -1 };
                let expected_rank = from_rank as i8 + direction;

                to_rank as i8 == expected_rank && file_diff == 1
            }
            PieceType::Knight => {
                // Knight moves in L-shape
                (rank_diff == 2 && file_diff == 1) || (rank_diff == 1 && file_diff == 2)
            }
            PieceType::Bishop => {
                // Bishop moves diagonally
                rank_diff == file_diff && rank_diff > 0 && self.is_diagonal_clear(from, to)
            }
            PieceType::Rook => {
                // Rook moves horizontally or vertically
                (from_rank == to_rank || from_file == to_file) &&
                self.is_line_clear(from, to)
            }
            PieceType::Queen => {
                // Queen moves like bishop or rook
                ((rank_diff == file_diff && rank_diff > 0 && self.is_diagonal_clear(from, to)) ||
                 ((from_rank == to_rank || from_file == to_file) && self.is_line_clear(from, to)))
            }
            PieceType::King => {
                // King moves one square in any direction
                rank_diff <= 1 && file_diff <= 1 && (rank_diff > 0 || file_diff > 0)
            }
        }
    }

    /// Check if the diagonal path is clear
    fn is_diagonal_clear(&self, from: u8, to: u8) -> bool {
        let from_rank = from / 8;
        let from_file = from % 8;
        let to_rank = to / 8;
        let to_file = to % 8;

        let rank_dir = if to_rank > from_rank { 1 } else { -1 };
        let file_dir = if to_file > from_file { 1 } else { -1 };

        let mut rank = from_rank as i8 + rank_dir;
        let mut file = from_file as i8 + file_dir;

        while rank != to_rank as i8 {
            let sq = (rank * 8 + file) as u8;
            if self.piece_at(sq).is_some() {
                return false;
            }
            rank += rank_dir;
            file += file_dir;
        }

        true
    }

    /// Check if the line (rank or file) is clear
    fn is_line_clear(&self, from: u8, to: u8) -> bool {
        let from_rank = from / 8;
        let from_file = from % 8;
        let to_rank = to / 8;
        let to_file = to % 8;

        if from_rank == to_rank {
            // Same rank - check files
            let start = from_file.min(to_file) + 1;
            let end = from_file.max(to_file);

            for file in start..end {
                let sq = from_rank * 8 + file;
                if self.piece_at(sq).is_some() {
                    return false;
                }
            }
        } else if from_file == to_file {
            // Same file - check ranks
            let start = from_rank.min(to_rank) + 1;
            let end = from_rank.max(to_rank);

            for rank in start..end {
                let sq = rank * 8 + from_file;
                if self.piece_at(sq).is_some() {
                    return false;
                }
            }
        }

        true
    }

    /// Get the value of a piece from a tuple
    fn piece_value_from_tuple(piece: &(PieceType, Color)) -> i32 {
        PIECE_VALUES[piece.0 as usize]
    }
}

struct AttackerInfo {
    from: u8,
    piece: (PieceType, Color),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_see_simple_capture() {
        // Test SEE on a simple capture
        // This would need actual position setup
        // For now, this is a placeholder
    }

    #[test]
    fn test_see_exchange() {
        // Test SEE on a capture with recapture
        // Placeholder test
    }
}
