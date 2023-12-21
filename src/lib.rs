pub mod board;
pub mod error;
pub mod r#move;

mod bitboard;
mod square;

use bitboard::Bitboard;
use square::Square;

pub const VERSION: &str = "0.1";

pub fn version() -> &'static str {
    VERSION
}