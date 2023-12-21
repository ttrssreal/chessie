use std::{fmt, error::Error};

#[derive(Debug)]
pub enum ChessError {
    InvalidFen {
        msg: String,
    },
    InvalidAlgNotation {
        msg: String,
    },
    InvalidSquare {
        square: u16,
    }
}

impl Error for ChessError {}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ChessError::InvalidFen { msg } => write!(f, "Invalid FEN: {}", msg),
            ChessError::InvalidAlgNotation { msg } => write!(f, "Invalid algebraic notation: {}", msg),
            ChessError::InvalidSquare { square } => write!(f, "Invalid square: {}", square),
        }
    }
}

impl From<ChessError> for fmt::Error {
    fn from(value: ChessError) -> Self {
        println!("{value}");
        fmt::Error
    }
}

