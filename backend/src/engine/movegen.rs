use crate::engine::bitboard::Bitboard;
use crate::engine::position::{Color, PieceType, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Capture,
    DoublePawnPush,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    PromotionKnight,
    PromotionBishop,
    PromotionRook,
    PromotionQueen,
    PromotionCaptureKnight,
    PromotionCaptureBishop,
    PromotionCaptureRook,
    PromotionCaptureQueen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: u8,
    to: u8,
    move_type: MoveType,
}

impl Move {
    #[inline]
    pub fn new(from: u8, to: u8, move_type: MoveType) -> Self {
        Self { from, to, move_type }
    }

    #[inline]
    pub fn from(&self) -> u8 {
        self.from
    }

    #[inline]
    pub fn to(&self) -> u8 {
        self.to
    }

    #[inline]
    pub fn move_type(&self) -> MoveType {
        self.move_type
    }

    pub fn is_capture(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::Capture
                | MoveType::EnPassant
                | MoveType::PromotionCaptureKnight
                | MoveType::PromotionCaptureBishop
                | MoveType::PromotionCaptureRook
                | MoveType::PromotionCaptureQueen
        )
    }

    pub fn is_promotion(&self) -> bool {
        matches!(
            self.move_type,
            MoveType::PromotionKnight
                | MoveType::PromotionBishop
                | MoveType::PromotionRook
                | MoveType::PromotionQueen
                | MoveType::PromotionCaptureKnight
                | MoveType::PromotionCaptureBishop
                | MoveType::PromotionCaptureRook
                | MoveType::PromotionCaptureQueen
        )
    }

    pub fn promotion_piece(&self) -> Option<PieceType> {
        match self.move_type {
            MoveType::PromotionKnight | MoveType::PromotionCaptureKnight => Some(PieceType::Knight),
            MoveType::PromotionBishop | MoveType::PromotionCaptureBishop => Some(PieceType::Bishop),
            MoveType::PromotionRook | MoveType::PromotionCaptureRook => Some(PieceType::Rook),
            MoveType::PromotionQueen | MoveType::PromotionCaptureQueen => Some(PieceType::Queen),
            _ => None,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut uci = format!(
            "{}{}",
            Bitboard::square_to_name(self.from),
            Bitboard::square_to_name(self.to)
        );
        if let Some(piece) = self.promotion_piece() {
            uci.push(match piece {
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                _ => unreachable!(),
            });
        }
        uci
    }

    pub fn from_uci(uci: &str, pos: &Position) -> Option<Self> {
        if uci.len() < 4 {
            return None;
        }

        let from = Bitboard::square_from_name(&uci[0..2])?;
        let to = Bitboard::square_from_name(&uci[2..4])?;

        // Generate all legal moves and find the matching one
        let moves = MoveGen::generate_legal_moves(pos);
        moves.into_iter().find(|m| m.from == from && m.to == to)
    }
}

pub struct MoveList {
    moves: Vec<Move>,
}

impl MoveList {
    pub fn new() -> Self {
        Self { moves: Vec::with_capacity(256) }
    }

    #[inline]
    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves.iter()
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

pub struct MoveGen;

impl MoveGen {
    /// Generate all pseudo-legal moves (may leave king in check)
    pub fn generate_pseudo_legal_moves(pos: &Position) -> MoveList {
        let mut moves = MoveList::new();
        let us = pos.side_to_move;
        let them = !us;

        Self::generate_pawn_moves(pos, &mut moves, us);
        Self::generate_knight_moves(pos, &mut moves, us);
        Self::generate_bishop_moves(pos, &mut moves, us);
        Self::generate_rook_moves(pos, &mut moves, us);
        Self::generate_queen_moves(pos, &mut moves, us);
        Self::generate_king_moves(pos, &mut moves, us);
        Self::generate_castling_moves(pos, &mut moves, us);

        moves
    }

    /// Generate all legal moves
    pub fn generate_legal_moves(pos: &Position) -> MoveList {
        let pseudo_legal = Self::generate_pseudo_legal_moves(pos);
        let mut legal = MoveList::new();

        for mv in pseudo_legal.into_iter() {
            let mut new_pos = pos.clone();
            if new_pos.make_move(mv).is_ok() {
                if !new_pos.is_opponent_in_check() {
                    legal.push(mv);
                }
            }
        }

        legal
    }

    fn generate_pawn_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let pawns = pos.piece_bb(PieceType::Pawn, us);
        let occupied = pos.occupied;
        let them_bb = pos.color_bb(!us);

        let (up, start_rank, promo_rank) = if us == Color::White {
            (8i8, 1u8, 6u8)
        } else {
            (-8i8, 6u8, 1u8)
        };

        // Single pushes
        let mut single_pushes = if us == Color::White {
            pawns.north()
        } else {
            pawns.south()
        } & !occupied;

        // Promotions from single pushes
        let promos = single_pushes & Bitboard::rank(if us == Color::White { 7 } else { 0 });
        single_pushes &= !promos;

        for to in single_pushes {
            let from = (to as i8 - up) as u8;
            moves.push(Move::new(from, to, MoveType::Normal));
        }

        for to in promos {
            let from = (to as i8 - up) as u8;
            moves.push(Move::new(from, to, MoveType::PromotionQueen));
            moves.push(Move::new(from, to, MoveType::PromotionRook));
            moves.push(Move::new(from, to, MoveType::PromotionBishop));
            moves.push(Move::new(from, to, MoveType::PromotionKnight));
        }

        // Double pushes
        let double_push_start = pawns & Bitboard::rank(start_rank);
        let double_pushes = if us == Color::White {
            double_push_start.north().north()
        } else {
            double_push_start.south().south()
        } & !occupied
            & if us == Color::White {
                !(occupied | occupied.south())
            } else {
                !(occupied | occupied.north())
            };

        for to in double_pushes {
            let from = (to as i8 - 2 * up) as u8;
            moves.push(Move::new(from, to, MoveType::DoublePawnPush));
        }

        // Captures (left and right)
        let left_attacks = if us == Color::White {
            pawns.north_west()
        } else {
            pawns.south_west()
        };
        let right_attacks = if us == Color::White {
            pawns.north_east()
        } else {
            pawns.south_east()
        };

        for direction_attacks in [left_attacks, right_attacks] {
            let captures = direction_attacks & them_bb;
            let promo_captures = captures & Bitboard::rank(if us == Color::White { 7 } else { 0 });
            let normal_captures = captures & !promo_captures;

            for to in normal_captures {
                let from = Self::find_pawn_origin(to, pawns, us);
                moves.push(Move::new(from, to, MoveType::Capture));
            }

            for to in promo_captures {
                let from = Self::find_pawn_origin(to, pawns, us);
                moves.push(Move::new(from, to, MoveType::PromotionCaptureQueen));
                moves.push(Move::new(from, to, MoveType::PromotionCaptureRook));
                moves.push(Move::new(from, to, MoveType::PromotionCaptureBishop));
                moves.push(Move::new(from, to, MoveType::PromotionCaptureKnight));
            }
        }

        // En passant
        if let Some(ep_square) = pos.en_passant_square {
            let ep_bb = Bitboard::from_square(ep_square);
            let ep_attackers = if us == Color::White {
                ep_bb.south_west() | ep_bb.south_east()
            } else {
                ep_bb.north_west() | ep_bb.north_east()
            } & pawns;

            for from in ep_attackers {
                moves.push(Move::new(from, ep_square, MoveType::EnPassant));
            }
        }
    }

    fn find_pawn_origin(to: u8, pawns: Bitboard, us: Color) -> u8 {
        let to_bb = Bitboard::from_square(to);
        let possible = if us == Color::White {
            to_bb.south_west() | to_bb.south_east() | to_bb.south()
        } else {
            to_bb.north_west() | to_bb.north_east() | to_bb.north()
        } & pawns;
        possible.lsb()
    }

    fn generate_knight_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let knights = pos.piece_bb(PieceType::Knight, us);
        let us_bb = pos.color_bb(us);
        let them_bb = pos.color_bb(!us);

        for from in knights {
            let attacks = Self::get_knight_attacks(from);
            let captures = attacks & them_bb;
            let quiets = attacks & !pos.occupied;

            for to in captures {
                moves.push(Move::new(from, to, MoveType::Capture));
            }
            for to in quiets {
                moves.push(Move::new(from, to, MoveType::Normal));
            }
        }
    }

    fn generate_bishop_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let bishops = pos.piece_bb(PieceType::Bishop, us);
        let them_bb = pos.color_bb(!us);

        for from in bishops {
            let attacks = Self::get_bishop_attacks(from, pos.occupied);
            let us_pieces = attacks & pos.color_bb(us);
            let valid_attacks = attacks & !us_pieces;
            let captures = valid_attacks & them_bb;
            let quiets = valid_attacks & !pos.occupied;

            for to in captures {
                moves.push(Move::new(from, to, MoveType::Capture));
            }
            for to in quiets {
                moves.push(Move::new(from, to, MoveType::Normal));
            }
        }
    }

    fn generate_rook_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let rooks = pos.piece_bb(PieceType::Rook, us);
        let them_bb = pos.color_bb(!us);

        for from in rooks {
            let attacks = Self::get_rook_attacks(from, pos.occupied);
            let us_pieces = attacks & pos.color_bb(us);
            let valid_attacks = attacks & !us_pieces;
            let captures = valid_attacks & them_bb;
            let quiets = valid_attacks & !pos.occupied;

            for to in captures {
                moves.push(Move::new(from, to, MoveType::Capture));
            }
            for to in quiets {
                moves.push(Move::new(from, to, MoveType::Normal));
            }
        }
    }

    fn generate_queen_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let queens = pos.piece_bb(PieceType::Queen, us);
        let them_bb = pos.color_bb(!us);

        for from in queens {
            let attacks = Self::get_queen_attacks(from, pos.occupied);
            let us_pieces = attacks & pos.color_bb(us);
            let valid_attacks = attacks & !us_pieces;
            let captures = valid_attacks & them_bb;
            let quiets = valid_attacks & !pos.occupied;

            for to in captures {
                moves.push(Move::new(from, to, MoveType::Capture));
            }
            for to in quiets {
                moves.push(Move::new(from, to, MoveType::Normal));
            }
        }
    }

    fn generate_king_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        let king = pos.piece_bb(PieceType::King, us);
        if king.is_empty() {
            return;
        }

        let from = king.lsb();
        let attacks = Self::get_king_attacks(from);
        let us_pieces = attacks & pos.color_bb(us);
        let valid_attacks = attacks & !us_pieces;
        let captures = valid_attacks & pos.color_bb(!us);
        let quiets = valid_attacks & !pos.occupied;

        for to in captures {
            moves.push(Move::new(from, to, MoveType::Capture));
        }
        for to in quiets {
            moves.push(Move::new(from, to, MoveType::Normal));
        }
    }

    fn generate_castling_moves(pos: &Position, moves: &mut MoveList, us: Color) {
        use crate::engine::position::CastlingRights;

        if us == Color::White {
            // White kingside
            if pos.castling_rights.has(CastlingRights::WHITE_KINGSIDE) {
                let king_sq = 4u8;
                let squares_empty = !pos.occupied.test_bit(5) && !pos.occupied.test_bit(6);
                if squares_empty {
                    let squares_safe = !pos.is_square_attacked(4, Color::Black)
                        && !pos.is_square_attacked(5, Color::Black)
                        && !pos.is_square_attacked(6, Color::Black);
                    if squares_safe {
                        moves.push(Move::new(king_sq, 6, MoveType::CastleKingside));
                    }
                }
            }
            // White queenside
            if pos.castling_rights.has(CastlingRights::WHITE_QUEENSIDE) {
                let king_sq = 4u8;
                let squares_empty = !pos.occupied.test_bit(1)
                    && !pos.occupied.test_bit(2)
                    && !pos.occupied.test_bit(3);
                if squares_empty {
                    let squares_safe = !pos.is_square_attacked(4, Color::Black)
                        && !pos.is_square_attacked(3, Color::Black)
                        && !pos.is_square_attacked(2, Color::Black);
                    if squares_safe {
                        moves.push(Move::new(king_sq, 2, MoveType::CastleQueenside));
                    }
                }
            }
        } else {
            // Black kingside
            if pos.castling_rights.has(CastlingRights::BLACK_KINGSIDE) {
                let king_sq = 60u8;
                let squares_empty = !pos.occupied.test_bit(61) && !pos.occupied.test_bit(62);
                if squares_empty {
                    let squares_safe = !pos.is_square_attacked(60, Color::White)
                        && !pos.is_square_attacked(61, Color::White)
                        && !pos.is_square_attacked(62, Color::White);
                    if squares_safe {
                        moves.push(Move::new(king_sq, 62, MoveType::CastleKingside));
                    }
                }
            }
            // Black queenside
            if pos.castling_rights.has(CastlingRights::BLACK_QUEENSIDE) {
                let king_sq = 60u8;
                let squares_empty = !pos.occupied.test_bit(57)
                    && !pos.occupied.test_bit(58)
                    && !pos.occupied.test_bit(59);
                if squares_empty {
                    let squares_safe = !pos.is_square_attacked(60, Color::White)
                        && !pos.is_square_attacked(59, Color::White)
                        && !pos.is_square_attacked(58, Color::White);
                    if squares_safe {
                        moves.push(Move::new(king_sq, 58, MoveType::CastleQueenside));
                    }
                }
            }
        }
    }

    // Attack getters (simplified - full implementation would use magic bitboards)
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

    fn get_king_attacks(square: u8) -> Bitboard {
        let bb = Bitboard::from_square(square);
        let attacks = bb.east() | bb.west();
        let bb_with_attacks = bb | attacks;
        bb_with_attacks.north() | bb_with_attacks.south() | attacks
    }

    fn get_bishop_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        Self::get_diagonal_attacks(square, occupied) | Self::get_anti_diagonal_attacks(square, occupied)
    }

    fn get_rook_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        Self::get_file_attacks(square, occupied) | Self::get_rank_attacks(square, occupied)
    }

    fn get_queen_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        Self::get_bishop_attacks(square, occupied) | Self::get_rook_attacks(square, occupied)
    }

    fn get_file_attacks(square: u8, occupied: Bitboard) -> Bitboard {
        let file = Bitboard::file_of(square);
        let rank = Bitboard::rank_of(square);
        let mut attacks = Bitboard::EMPTY;

        // North
        for r in (rank + 1)..8 {
            let sq = Bitboard::square_from_coords(file, r);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        // South
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

        // East
        for f in (file + 1)..8 {
            let sq = Bitboard::square_from_coords(f, rank);
            attacks.set_bit(sq);
            if occupied.test_bit(sq) {
                break;
            }
        }

        // West
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

        // North-East
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

        // South-West
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

        // North-West
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

        // South-East
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

// Add make_move and is_opponent_in_check to Position
impl Position {
    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        use crate::engine::position::CastlingRights;

        let from = mv.from();
        let to = mv.to();
        let us = self.side_to_move;
        let them = !us;

        let piece = self
            .piece_at(from)
            .ok_or("No piece at from square")?
            .0;

        // Remove piece from origin
        self.remove_piece(from, piece, us);

        // Handle captures
        if mv.is_capture() && mv.move_type() != MoveType::EnPassant {
            if let Some((captured_piece, _)) = self.piece_at(to) {
                self.remove_piece(to, captured_piece, them);
            }
        }

        // Handle special moves
        match mv.move_type() {
            MoveType::EnPassant => {
                let capture_square = if us == Color::White {
                    to - 8
                } else {
                    to + 8
                };
                self.remove_piece(capture_square, PieceType::Pawn, them);
                self.put_piece(to, PieceType::Pawn, us);
            }
            MoveType::CastleKingside => {
                let (rook_from, rook_to) = if us == Color::White {
                    (7u8, 5u8)
                } else {
                    (63u8, 61u8)
                };
                self.remove_piece(rook_from, PieceType::Rook, us);
                self.put_piece(rook_to, PieceType::Rook, us);
                self.put_piece(to, PieceType::King, us);
            }
            MoveType::CastleQueenside => {
                let (rook_from, rook_to) = if us == Color::White {
                    (0u8, 3u8)
                } else {
                    (56u8, 59u8)
                };
                self.remove_piece(rook_from, PieceType::Rook, us);
                self.put_piece(rook_to, PieceType::Rook, us);
                self.put_piece(to, PieceType::King, us);
            }
            _ if mv.is_promotion() => {
                if let Some(promo_piece) = mv.promotion_piece() {
                    self.put_piece(to, promo_piece, us);
                }
            }
            _ => {
                self.put_piece(to, piece, us);
            }
        }

        // Update en passant square
        self.en_passant_square = None;
        if mv.move_type() == MoveType::DoublePawnPush {
            let ep_square = if us == Color::White { to - 8 } else { to + 8 };
            self.en_passant_square = Some(ep_square);
        }

        // Update castling rights
        if piece == PieceType::King {
            if us == Color::White {
                self.castling_rights.remove(CastlingRights::WHITE_BOTH);
            } else {
                self.castling_rights.remove(CastlingRights::BLACK_BOTH);
            }
        }

        if piece == PieceType::Rook {
            match from {
                0 => self.castling_rights.remove(CastlingRights::WHITE_QUEENSIDE),
                7 => self.castling_rights.remove(CastlingRights::WHITE_KINGSIDE),
                56 => self.castling_rights.remove(CastlingRights::BLACK_QUEENSIDE),
                63 => self.castling_rights.remove(CastlingRights::BLACK_KINGSIDE),
                _ => {}
            }
        }

        // Update halfmove clock
        if piece == PieceType::Pawn || mv.is_capture() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Update fullmove number
        if us == Color::Black {
            self.fullmove_number += 1;
        }

        // Switch sides
        self.side_to_move = them;

        // Update hash
        self.update_hash();

        Ok(())
    }

    pub fn is_opponent_in_check(&self) -> bool {
        let opponent = !self.side_to_move;
        let king_square = self.piece_bb(PieceType::King, opponent).lsb();
        self.is_square_attacked(king_square, self.side_to_move)
    }
}
