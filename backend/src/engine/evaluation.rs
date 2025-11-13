use crate::engine::position::{Color, PieceType, Position};
use crate::engine::bitboard::Bitboard;

pub const INFINITY: i32 = 30000;
pub const MATE_SCORE: i32 = 29000;

// Material values (centipawns)
const PIECE_VALUES: [i32; 6] = [
    100,  // Pawn
    320,  // Knight
    330,  // Bishop
    500,  // Rook
    900,  // Queen
    20000 // King
];

// Piece-square tables (from white's perspective)
// Values are in centipawns and encourage good piece placement

const PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  25,  25,  10,   5,   5,
     0,   0,   0,  20,  20,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -20, -20,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0,
];

const KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0,
];

const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,   5,   5,   5,   0, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

const KING_MIDDLE_GAME_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];

const KING_END_GAME_TABLE: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -50, -30, -30, -30, -30, -30, -30, -50,
];

pub struct Evaluator;

impl Evaluator {
    /// Evaluate the position from the side to move's perspective
    pub fn evaluate(pos: &Position) -> i32 {
        let mut score = 0;

        // Material and piece-square tables
        score += Self::evaluate_material_and_pst(pos, Color::White);
        score -= Self::evaluate_material_and_pst(pos, Color::Black);

        // Pawn structure
        score += Self::evaluate_pawn_structure(pos, Color::White);
        score -= Self::evaluate_pawn_structure(pos, Color::Black);

        // Mobility
        score += Self::evaluate_mobility(pos, Color::White);
        score -= Self::evaluate_mobility(pos, Color::Black);

        // King safety
        score += Self::evaluate_king_safety(pos, Color::White);
        score -= Self::evaluate_king_safety(pos, Color::Black);

        // Bishop pair bonus
        if Self::has_bishop_pair(pos, Color::White) {
            score += 50;
        }
        if Self::has_bishop_pair(pos, Color::Black) {
            score -= 50;
        }

        // Return from the side to move's perspective
        if pos.side_to_move == Color::White {
            score
        } else {
            -score
        }
    }

    fn evaluate_material_and_pst(pos: &Position, color: Color) -> i32 {
        let mut score = 0;
        let is_endgame = Self::is_endgame(pos);

        for piece_type in PieceType::ALL {
            let pieces = pos.piece_bb(piece_type, color);
            let piece_value = PIECE_VALUES[piece_type.index()];

            for square in pieces {
                score += piece_value;

                // Add piece-square table bonus
                let sq_index = if color == Color::White {
                    square as usize
                } else {
                    (square ^ 56) as usize // Flip for black
                };

                let pst_bonus = match piece_type {
                    PieceType::Pawn => PAWN_TABLE[sq_index],
                    PieceType::Knight => KNIGHT_TABLE[sq_index],
                    PieceType::Bishop => BISHOP_TABLE[sq_index],
                    PieceType::Rook => ROOK_TABLE[sq_index],
                    PieceType::Queen => QUEEN_TABLE[sq_index],
                    PieceType::King => {
                        if is_endgame {
                            KING_END_GAME_TABLE[sq_index]
                        } else {
                            KING_MIDDLE_GAME_TABLE[sq_index]
                        }
                    }
                };

                score += pst_bonus;
            }
        }

        score
    }

    fn evaluate_pawn_structure(pos: &Position, color: Color) -> i32 {
        let mut score = 0;
        let pawns = pos.piece_bb(PieceType::Pawn, color);

        // Doubled pawns penalty
        for file in 0..8 {
            let file_bb = Bitboard::file(file);
            let pawns_on_file = (pawns & file_bb).count();
            if pawns_on_file > 1 {
                score -= 20 * (pawns_on_file - 1) as i32;
            }
        }

        // Isolated pawns penalty
        for square in pawns {
            let file = Bitboard::file_of(square);
            let neighbor_files = if file == 0 {
                Bitboard::file(1)
            } else if file == 7 {
                Bitboard::file(6)
            } else {
                Bitboard::file(file - 1) | Bitboard::file(file + 1)
            };

            if (pawns & neighbor_files).is_empty() {
                score -= 15;
            }
        }

        // Passed pawns bonus
        for square in pawns {
            if Self::is_passed_pawn(pos, square, color) {
                let rank = Bitboard::rank_of(square);
                let bonus = if color == Color::White {
                    (rank as i32) * 10
                } else {
                    (7 - rank as i32) * 10
                };
                score += bonus;
            }
        }

        score
    }

    fn is_passed_pawn(pos: &Position, square: u8, color: Color) -> bool {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);

        let opp_pawns = pos.piece_bb(PieceType::Pawn, !color);

        // Get the files to check (this file and adjacent files)
        let mut files_to_check = Bitboard::file(file);
        if file > 0 {
            files_to_check |= Bitboard::file(file - 1);
        }
        if file < 7 {
            files_to_check |= Bitboard::file(file + 1);
        }

        // Get the ranks ahead
        let ranks_ahead = if color == Color::White {
            Bitboard::new(0xFFFF_FFFF_FFFF_FFFF << ((rank + 1) * 8))
        } else {
            Bitboard::new(0xFFFF_FFFF_FFFF_FFFF >> ((8 - rank) * 8))
        };

        (opp_pawns & files_to_check & ranks_ahead).is_empty()
    }

    fn evaluate_mobility(pos: &Position, color: Color) -> i32 {
        // Simplified mobility evaluation
        // Count potential moves for minor and major pieces
        let mut mobility = 0;

        let knights = pos.piece_bb(PieceType::Knight, color);
        for square in knights {
            let attacks = Self::get_knight_attacks(square);
            let valid_squares = attacks & !pos.color_bb(color);
            mobility += valid_squares.count() as i32;
        }

        let bishops = pos.piece_bb(PieceType::Bishop, color);
        for square in bishops {
            let attacks = Self::get_bishop_attacks(square, pos.occupied);
            let valid_squares = attacks & !pos.color_bb(color);
            mobility += valid_squares.count() as i32;
        }

        let rooks = pos.piece_bb(PieceType::Rook, color);
        for square in rooks {
            let attacks = Self::get_rook_attacks(square, pos.occupied);
            let valid_squares = attacks & !pos.color_bb(color);
            mobility += valid_squares.count() as i32;
        }

        mobility * 2 // Weight mobility
    }

    fn evaluate_king_safety(pos: &Position, color: Color) -> i32 {
        if Self::is_endgame(pos) {
            return 0; // King safety doesn't matter in endgame
        }

        let mut safety = 0;
        let king_bb = pos.piece_bb(PieceType::King, color);
        if king_bb.is_empty() {
            return 0;
        }

        let king_square = king_bb.lsb();
        let king_file = Bitboard::file_of(king_square);

        // Pawn shield bonus
        let pawn_shield_squares = if color == Color::White {
            let king_bb_pos = Bitboard::from_square(king_square);
            king_bb_pos.north() | king_bb_pos.north_east() | king_bb_pos.north_west()
        } else {
            let king_bb_pos = Bitboard::from_square(king_square);
            king_bb_pos.south() | king_bb_pos.south_east() | king_bb_pos.south_west()
        };

        let pawns = pos.piece_bb(PieceType::Pawn, color);
        let shield_pawns = (pawn_shield_squares & pawns).count();
        safety += shield_pawns as i32 * 15;

        // Penalty for open files near king
        for file_offset in -1..=1 {
            let file = king_file as i32 + file_offset;
            if file >= 0 && file < 8 {
                let file_bb = Bitboard::file(file as u8);
                if (file_bb & pawns).is_empty() {
                    safety -= 25;
                }
            }
        }

        safety
    }

    fn has_bishop_pair(pos: &Position, color: Color) -> bool {
        pos.piece_bb(PieceType::Bishop, color).count() >= 2
    }

    fn is_endgame(pos: &Position) -> bool {
        // Simple endgame detection: few pieces left
        let queens = pos.piece_bb(PieceType::Queen, Color::White).count()
            + pos.piece_bb(PieceType::Queen, Color::Black).count();
        let minors = pos.piece_bb(PieceType::Knight, Color::White).count()
            + pos.piece_bb(PieceType::Knight, Color::Black).count()
            + pos.piece_bb(PieceType::Bishop, Color::White).count()
            + pos.piece_bb(PieceType::Bishop, Color::Black).count();
        let rooks = pos.piece_bb(PieceType::Rook, Color::White).count()
            + pos.piece_bb(PieceType::Rook, Color::Black).count();

        queens == 0 || (queens <= 2 && minors <= 2 && rooks <= 2)
    }

    // Helper attack functions (copied from movegen for evaluation purposes)
    fn get_knight_attacks(square: u8) -> Bitboard {
        let bb = Bitboard::from_square(square);
        let l1 = (bb >> 1) & Bitboard::NOT_H_FILE;
        let l2 = (bb >> 2) & Bitboard::NOT_GH_FILE;
        let r1 = (bb << 1) & Bitboard::NOT_A_FILE;
        let r2 = (bb << 2) & Bitboard::NOT_AB_FILE;
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
    }

    fn get_bishop_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        Self::get_diagonal_attacks(square, occupied) | Self::get_anti_diagonal_attacks(square, occupied)
    }

    fn get_rook_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        Self::get_file_attacks(square, occupied) | Self::get_rank_attacks(square, occupied)
    }

    fn get_file_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);
        let mut attacks = Bitboard::EMPTY;

        for r in (rank + 1)..8 {
            let sq = Bitboard::square_from_coords(file, r);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        for r in (0..rank).rev() {
            let sq = Bitboard::square_from_coords(file, r);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        attacks
    }

    fn get_rank_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);
        let mut attacks = Bitboard::EMPTY;

        for f in (file + 1)..8 {
            let sq = Bitboard::square_from_coords(f, rank);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        for f in (0..file).rev() {
            let sq = Bitboard::square_from_coords(f, rank);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        attacks
    }

    fn get_diagonal_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);
        let mut attacks = Bitboard::EMPTY;

        let mut f = file + 1;
        let mut r = rank + 1;
        while f < 8 && r < 8 {
            let sq = Bitboard::square_from_coords(f, r);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
            f += 1;
            r += 1;
        }

        let mut f = file as i8 - 1;
        let mut r = rank as i8 - 1;
        while f >= 0 && r >= 0 {
            let sq = Bitboard::square_from_coords(f as u8, r as u8);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
            f -= 1;
            r -= 1;
        }

        attacks
    }

    fn get_anti_diagonal_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);
        let mut attacks = Bitboard::EMPTY;

        let mut f = file as i8 - 1;
        let mut r = rank + 1;
        while f >= 0 && r < 8 {
            let sq = Bitboard::square_from_coords(f as u8, r);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
            f -= 1;
            r += 1;
        }

        let mut f = file + 1;
        let mut r = rank as i8 - 1;
        while f < 8 && r >= 0 {
            let sq = Bitboard::square_from_coords(f, r as u8);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
            f += 1;
            r -= 1;
        }

        attacks
    }
}
