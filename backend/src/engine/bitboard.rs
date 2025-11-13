use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

/// Bitboard representation of the chess board
/// Each bit represents a square on the board (0-63)
/// Bit 0 = a1, Bit 7 = h1, Bit 56 = a8, Bit 63 = h8
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(0xFFFF_FFFF_FFFF_FFFF);

    // Ranks
    pub const RANK_1: Bitboard = Bitboard(0x0000_0000_0000_00FF);
    pub const RANK_2: Bitboard = Bitboard(0x0000_0000_0000_FF00);
    pub const RANK_3: Bitboard = Bitboard(0x0000_0000_00FF_0000);
    pub const RANK_4: Bitboard = Bitboard(0x0000_0000_FF00_0000);
    pub const RANK_5: Bitboard = Bitboard(0x0000_00FF_0000_0000);
    pub const RANK_6: Bitboard = Bitboard(0x0000_FF00_0000_0000);
    pub const RANK_7: Bitboard = Bitboard(0x00FF_0000_0000_0000);
    pub const RANK_8: Bitboard = Bitboard(0xFF00_0000_0000_0000);

    // Files
    pub const FILE_A: Bitboard = Bitboard(0x0101_0101_0101_0101);
    pub const FILE_B: Bitboard = Bitboard(0x0202_0202_0202_0202);
    pub const FILE_C: Bitboard = Bitboard(0x0404_0404_0404_0404);
    pub const FILE_D: Bitboard = Bitboard(0x0808_0808_0808_0808);
    pub const FILE_E: Bitboard = Bitboard(0x1010_1010_1010_1010);
    pub const FILE_F: Bitboard = Bitboard(0x2020_2020_2020_2020);
    pub const FILE_G: Bitboard = Bitboard(0x4040_4040_4040_4040);
    pub const FILE_H: Bitboard = Bitboard(0x8080_8080_8080_8080);

    // Not on edges (useful for move generation)
    pub const NOT_A_FILE: Bitboard = Bitboard(0xFEFE_FEFE_FEFE_FEFE);
    pub const NOT_H_FILE: Bitboard = Bitboard(0x7F7F_7F7F_7F7F_7F7F);
    pub const NOT_AB_FILE: Bitboard = Bitboard(0xFCFC_FCFC_FCFC_FCFC);
    pub const NOT_GH_FILE: Bitboard = Bitboard(0x3F3F_3F3F_3F3F_3F3F);

    #[inline]
    pub const fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline]
    pub const fn from_square(square: u8) -> Self {
        Bitboard(1u64 << square)
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    #[inline]
    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    #[inline]
    pub fn toggle_bit(&mut self, square: u8) {
        self.0 ^= 1u64 << square;
    }

    #[inline]
    pub const fn test_bit(self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    /// Count the number of set bits (population count)
    #[inline]
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Get the index of the least significant bit
    #[inline]
    pub fn lsb(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    /// Get the index of the most significant bit
    #[inline]
    pub fn msb(self) -> u8 {
        63 - self.0.leading_zeros() as u8
    }

    /// Pop the least significant bit and return its index
    #[inline]
    pub fn pop_lsb(&mut self) -> u8 {
        let lsb = self.lsb();
        self.0 &= self.0 - 1; // Clear the LSB
        lsb
    }

    /// Get rank of a square (0-7)
    #[inline]
    pub const fn rank_of(square: u8) -> u8 {
        square >> 3
    }

    /// Get file of a square (0-7)
    #[inline]
    pub const fn file_of(square: u8) -> u8 {
        square & 7
    }

    /// Convert file and rank to square index
    #[inline]
    pub const fn square_from_coords(file: u8, rank: u8) -> u8 {
        rank * 8 + file
    }

    /// Get rank bitboard
    #[inline]
    pub const fn rank(rank: u8) -> Bitboard {
        Bitboard(0xFF << (rank * 8))
    }

    /// Get file bitboard
    #[inline]
    pub const fn file(file: u8) -> Bitboard {
        Bitboard(0x0101_0101_0101_0101 << file)
    }

    /// Shift north (toward rank 8)
    #[inline]
    pub const fn north(self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    /// Shift south (toward rank 1)
    #[inline]
    pub const fn south(self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    /// Shift east (toward h file)
    #[inline]
    pub const fn east(self) -> Bitboard {
        Bitboard((self.0 << 1) & Self::NOT_A_FILE.0)
    }

    /// Shift west (toward a file)
    #[inline]
    pub const fn west(self) -> Bitboard {
        Bitboard((self.0 >> 1) & Self::NOT_H_FILE.0)
    }

    /// Shift north-east
    #[inline]
    pub const fn north_east(self) -> Bitboard {
        Bitboard((self.0 << 9) & Self::NOT_A_FILE.0)
    }

    /// Shift north-west
    #[inline]
    pub const fn north_west(self) -> Bitboard {
        Bitboard((self.0 << 7) & Self::NOT_H_FILE.0)
    }

    /// Shift south-east
    #[inline]
    pub const fn south_east(self) -> Bitboard {
        Bitboard((self.0 >> 7) & Self::NOT_A_FILE.0)
    }

    /// Shift south-west
    #[inline]
    pub const fn south_west(self) -> Bitboard {
        Bitboard((self.0 >> 9) & Self::NOT_H_FILE.0)
    }

    /// Convert square name (e.g., "e4") to square index
    pub fn square_from_name(name: &str) -> Option<u8> {
        if name.len() != 2 {
            return None;
        }

        let chars: Vec<char> = name.chars().collect();
        let file = (chars[0] as u8).checked_sub(b'a')?;
        let rank = (chars[1] as u8).checked_sub(b'1')?;

        if file < 8 && rank < 8 {
            Some(Self::square_from_coords(file, rank))
        } else {
            None
        }
    }

    /// Convert square index to square name (e.g., "e4")
    pub fn square_to_name(square: u8) -> String {
        let file = Self::file_of(square);
        let rank = Self::rank_of(square);
        format!("{}{}", (b'a' + file) as char, (b'1' + rank) as char)
    }
}

// Iterator over set bits
impl Iterator for Bitboard {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop_lsb())
        }
    }
}

// Bitwise operations
impl BitAnd for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Self::square_from_coords(file, rank);
                if self.test_bit(square) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_basics() {
        let bb = Bitboard::from_square(0);
        assert_eq!(bb.0, 1);
        assert_eq!(bb.count(), 1);
        assert_eq!(bb.lsb(), 0);

        let bb = Bitboard::from_square(63);
        assert_eq!(bb.count(), 1);
        assert_eq!(bb.lsb(), 63);
    }

    #[test]
    fn test_square_conversion() {
        assert_eq!(Bitboard::square_from_name("e4"), Some(28));
        assert_eq!(Bitboard::square_to_name(28), "e4");
        assert_eq!(Bitboard::square_from_name("a1"), Some(0));
        assert_eq!(Bitboard::square_to_name(0), "a1");
        assert_eq!(Bitboard::square_from_name("h8"), Some(63));
        assert_eq!(Bitboard::square_to_name(63), "h8");
    }

    #[test]
    fn test_shifts() {
        let e4 = Bitboard::from_square(28);
        let e5 = e4.north();
        assert_eq!(e5, Bitboard::from_square(36));

        let e3 = e4.south();
        assert_eq!(e3, Bitboard::from_square(20));

        let f4 = e4.east();
        assert_eq!(f4, Bitboard::from_square(29));

        let d4 = e4.west();
        assert_eq!(d4, Bitboard::from_square(27));
    }
}
