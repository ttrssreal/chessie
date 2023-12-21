use std::fmt::Debug;

use crate::square::Square;

pub const EMPTY: Bitboard = Bitboard(0);

/// Bitboard Implementation
/// 
/// Layout/Bit-Ordering
/// 
/// 7 | 63 62 61 60 59 58 57 56
/// 6 | 55 54 53 52 51 50 49 48
/// 5 | 47 46 45 44 43 42 41 40
/// 4 | 39 38 37 36 35 34 33 32
/// 3 | 31 30 29 28 27 26 25 24
/// 2 | 23 22 21 20 19 18 17 16
/// 1 | 15 14 13 12 11 10 09 08
/// 0 | 07 06 05 04 03 02 01 00
///   ------------------------
///     0  1  2  3  4  5  6  7

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn from_u64(value: u64) -> Bitboard {
        Bitboard(value)
    }

    pub fn from_square(square: Square) -> Bitboard {
        Bitboard(1 << square.to_u8())
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    pub fn is_occ(&self, square: Square) -> bool {
        self.0 & (1 << square.to_u8()) != 0
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square.to_u8();
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1 << square.to_u8());
    }

    pub fn flip(&mut self, square: Square) {
        self.0 ^= 1 << square.to_u8();
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl From<Bitboard> for u64 {
    fn from(value: Bitboard) -> Self {
        value.0
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Bitboard(1 << square.to_u8())
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                match self.is_occ(Square::from_file_rank(file, rank)) {
                    true => write!(f, "1")?,
                    false => write!(f, "0")?,
                }

                if file != 7 { write!(f, " ")?; }
            }
            if rank != 0 { write!(f, "\n")?; }
        }
        Ok(())
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}