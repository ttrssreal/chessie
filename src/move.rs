use std::fmt::Display;

use crate::{square::Square, error::ChessError};


/// Move
/// 
/// Encoding:
/// 
/// 8 bits | 0-5:   from
/// 
/// 8 bits | 6-11:  to
/// 
/// 4 bits | 12-15: promotion
///
/// https://www.chessprogramming.org/Encoding_Moves#From-To_Based
///
/// code | promotion | capture | special 1 | special 0 | kind of move
///  0      0           0         0           0           quiet moves
///  1      0           0         0           1           double pawn push
///  2      0           0         1           0           king castle
///  3      0           0         1           1           queen castle
///  4      0           1         0           0           captures
///  5      0           1         0           1           ep-capture
///  8      1           0         0           0           knight-promotion
///  9      1           0         0           1           bishop-promotion
///  10     1           0         1           0           rook-promotion
///  11     1           0         1           1           queen-promotion
///  12     1           1         0           0           knight-promo capture
///  13     1           1         0           1           bishop-promo capture
///  14     1           1         1           0           rook-promo capture
///  15     1           1         1           1           queen-promo capture 
pub struct Move(u16);

impl Move {
    pub fn from_raw(raw: u16) -> Move {
        Move(raw)
    }

    pub fn new(from: Square, to: Square, promotion: u16) -> Move {
        let mut mv: u16 = from.to_u16() << 6| to.to_u16() & 0b0000_111111_111111;
        mv |= promotion << 12;
        Move(mv)
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }

    pub fn from(&self) -> Result<Square, ChessError> {
        Square::from_u16(self.0 & 0b0000_000000_111111)
    }

    pub fn to(&self) -> Result<Square, ChessError> {
        Square::from_u16((self.0 & 0b0000_111111_000000) >> 6)
    }

    pub fn from_panic(&self) -> Square {
        Square::from_u16_panic(self.0 & 0b0000_000000_111111)
    }

    pub fn to_panic(&self) -> Square {
        Square::from_u16_panic((self.0 & 0b0000_111111_000000) >> 6)
    }

    pub fn promotion(&self) -> u16 {
        (self.0 & 0b1111_000000_000000) >> 12
    }

    /// Parses out a move from algebraic notation, eg. e2e4, e7e8q, e1g1, e7e5
    /// 
    /// Without context from a game, whilst we know about any promoted pieces, we
    /// dont know some metadata about the move such as if it was a capture or not,
    /// so we just set promotion bits.
    pub fn from_algebraic_simple(algr: &str) -> Result<Move, ChessError> {
        if algr.len() < 4 {
            return Err(ChessError::InvalidAlgNotation {
                msg: format!("Invalid algebraic move notation: {}", algr) });
        }

        let from = algr[0..2].parse::<Square>()?;
        let to = algr[2..4].parse::<Square>()?;

        let mut mv: u16 = to.to_u16() << 6| from.to_u16() & 0b0000_111111_111111;

        let promotion = match algr.get(4..5) {
            None => 0,
            Some("n") => 1 << 3 | 0,
            Some("b") => 1 << 3 | 1 << 0,
            Some("r") => 1 << 3 | 1 << 1,
            Some("q") => 1 << 3 | 1 << 0 | 1 << 1,
            Some(p) => return Err(ChessError::InvalidAlgNotation {
                msg: format!("Invalid promo piece ({}): {}", algr, p) })
        };

        mv |= promotion << 12;

        Ok(Move(mv))
    }

    pub fn to_algebraic(&self) -> Result<String, ChessError> {
        let from = self.from()?;
        let to = self.to()?;

        let mut algr = format!("{}{}", from, to);

        let promotion = self.promotion();

        if promotion & 1 << 3 != 0 {
            algr.push_str(match promotion & 0b11 {
                0 => "n",
                1 => "b",
                2 => "r",
                3 => "q",
                _ => return Err(ChessError::InvalidAlgNotation {
                    msg: format!("Invalid promotion piece: {}", promotion) })
            });
        }

        Ok(algr)
    }

}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from()?, self.to()?)
    }
}
