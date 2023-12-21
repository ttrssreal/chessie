use std::{str::FromStr, fmt::Display};

use crate::error::ChessError;

#[derive(Debug, Clone, Copy)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    pub fn to_u16(&self) -> u16 {
        *self as u16
    }

    pub fn from_u16(n: u16) -> Result<Self, ChessError> {
        use Square::*;

        match n {
            0 => Ok(A1), 1 => Ok(B1), 2 => Ok(C1), 3 => Ok(D1), 4 => Ok(E1), 5 => Ok(F1), 6 => Ok(G1), 7 => Ok(H1),
            8 => Ok(A2), 9 => Ok(B2), 10 => Ok(C2), 11 => Ok(D2), 12 => Ok(E2), 13 => Ok(F2), 14 => Ok(G2), 15 => Ok(H2),
            16 => Ok(A3), 17 => Ok(B3), 18 => Ok(C3), 19 => Ok(D3), 20 => Ok(E3), 21 => Ok(F3), 22 => Ok(G3), 23 => Ok(H3),
            24 => Ok(A4), 25 => Ok(B4), 26 => Ok(C4), 27 => Ok(D4), 28 => Ok(E4), 29 => Ok(F4), 30 => Ok(G4), 31 => Ok(H4),
            32 => Ok(A5), 33 => Ok(B5), 34 => Ok(C5), 35 => Ok(D5), 36 => Ok(E5), 37 => Ok(F5), 38 => Ok(G5), 39 => Ok(H5),
            40 => Ok(A6), 41 => Ok(B6), 42 => Ok(C6), 43 => Ok(D6), 44 => Ok(E6), 45 => Ok(F6), 46 => Ok(G6), 47 => Ok(H6),
            48 => Ok(A7), 49 => Ok(B7), 50 => Ok(C7), 51 => Ok(D7), 52 => Ok(E7), 53 => Ok(F7), 54 => Ok(G7), 55 => Ok(H7),
            56 => Ok(A8), 57 => Ok(B8), 58 => Ok(C8), 59 => Ok(D8), 60 => Ok(E8), 61 => Ok(F8), 62 => Ok(G8), 63 => Ok(H8),
            _ => Err(ChessError::InvalidSquare { square: n }),
        }
    }

    pub fn from_u16_panic(n: u16) -> Self {
        use Square::*;

        match n {
            0 => A1, 1 => B1, 2 => C1, 3 => D1, 4 => E1, 5 => F1, 6 => G1, 7 => H1,
            8 => A2, 9 => B2, 10 => C2, 11 => D2, 12 => E2, 13 => F2, 14 => G2, 15 => H2,
            16 => A3, 17 => B3, 18 => C3, 19 => D3, 20 => E3, 21 => F3, 22 => G3, 23 => H3,
            24 => A4, 25 => B4, 26 => C4, 27 => D4, 28 => E4, 29 => F4, 30 => G4, 31 => H4,
            32 => A5, 33 => B5, 34 => C5, 35 => D5, 36 => E5, 37 => F5, 38 => G5, 39 => H5,
            40 => A6, 41 => B6, 42 => C6, 43 => D6, 44 => E6, 45 => F6, 46 => G6, 47 => H6,
            48 => A7, 49 => B7, 50 => C7, 51 => D7, 52 => E7, 53 => F7, 54 => G7, 55 => H7,
            56 => A8, 57 => B8, 58 => C8, 59 => D8, 60 => E8, 61 => F8, 62 => G8, 63 => H8,
            _ => panic!("Invalid square: {}", n),
        }
    }

    pub fn from_file_rank(file: usize, rank: usize) -> Self {
        use Square::*;

        let file = file as u8;
        let rank = rank as u8;

        match (file, rank) {
            (0, 0) => A1, (1, 0) => B1, (2, 0) => C1, (3, 0) => D1, (4, 0) => E1, (5, 0) => F1, (6, 0) => G1, (7, 0) => H1,
            (0, 1) => A2, (1, 1) => B2, (2, 1) => C2, (3, 1) => D2, (4, 1) => E2, (5, 1) => F2, (6, 1) => G2, (7, 1) => H2,
            (0, 2) => A3, (1, 2) => B3, (2, 2) => C3, (3, 2) => D3, (4, 2) => E3, (5, 2) => F3, (6, 2) => G3, (7, 2) => H3,
            (0, 3) => A4, (1, 3) => B4, (2, 3) => C4, (3, 3) => D4, (4, 3) => E4, (5, 3) => F4, (6, 3) => G4, (7, 3) => H4,
            (0, 4) => A5, (1, 4) => B5, (2, 4) => C5, (3, 4) => D5, (4, 4) => E5, (5, 4) => F5, (6, 4) => G5, (7, 4) => H5,
            (0, 5) => A6, (1, 5) => B6, (2, 5) => C6, (3, 5) => D6, (4, 5) => E6, (5, 5) => F6, (6, 5) => G6, (7, 5) => H6,
            (0, 6) => A7, (1, 6) => B7, (2, 6) => C7, (3, 6) => D7, (4, 6) => E7, (5, 6) => F7, (6, 6) => G7, (7, 6) => H7,
            (0, 7) => A8, (1, 7) => B8, (2, 7) => C8, (3, 7) => D8, (4, 7) => E8, (5, 7) => F8, (6, 7) => G8, (7, 7) => H8,
            _ => panic!("Invalid file({}) or rank({})", file, rank),
        }
    }

    pub fn from_algebraic(algr: &str) -> Result<Self, ChessError> {
        let lc = algr.to_lowercase();
        let algr = lc.as_bytes();

        if algr.len() != 2 {
            return Err(ChessError::InvalidAlgNotation {
                msg: format!("{}", String::from_utf8_lossy(algr)) });
        }

        if !matches!(algr[0], b'a'..=b'h') || !matches!(algr[1], b'1'..=b'8') {
            return Err(ChessError::InvalidAlgNotation {
                msg: format!("Invalid rank/file: {}", String::from_utf8_lossy(algr)) });
        }

        let file = algr[0] as usize - 'a' as usize;
        let rank = algr[1] as usize - '1' as usize;

        Ok(Self::from_file_rank(file, rank))
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.to_u8() % 8;
        let rank = self.to_u8() / 8;

        format!("{}{}", (file + 'a' as u8) as char, (rank + '1' as u8) as char)
    }
}

impl FromStr for Square {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_algebraic(s)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}
