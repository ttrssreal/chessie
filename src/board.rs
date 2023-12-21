use std::fmt::Display;

use crate::bitboard;
use crate::error::ChessError;
use crate::Bitboard;
use crate::Square;
use crate::r#move::Move;

pub const FEN_STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_KING: usize = 0;
pub const WHITE_QUEEN: usize = 1;
pub const WHITE_ROOK: usize = 2;
pub const WHITE_BISHOP: usize = 3;
pub const WHITE_KNIGHT: usize = 4;
pub const WHITE_PAWN: usize = 5;
pub const BLACK_KING: usize = 6;
pub const BLACK_QUEEN: usize = 7;
pub const BLACK_ROOK: usize = 8;
pub const BLACK_BISHOP: usize = 9;
pub const BLACK_KNIGHT: usize = 10;
pub const BLACK_PAWN: usize = 11;

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub struct Board {
    /// 0-5: White pieces
    /// 6-11: Black pieces
    pub bitboards: [Bitboard; 12],

    /// true: White
    /// false: Black
    pub side_to_move: usize,

    /// bit 0: White kingside
    /// bit 1: White queenside
    /// bit 2: Black kingside
    /// bit 3: Black queenside
    pub castling_rights: u8,
    
    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_counter: u32,

    pub move_stack: Vec<Move>,
}

impl Board {
    pub fn startpos() -> Self {
        Board::from_fen(FEN_STARTPOS)
            .expect("Internal error: Invalid fen: FEN_STARTPOS")
    }
    
    pub fn from_fen(fen: &str) -> Result<Self, ChessError> {
        let fields: Vec<&str> = fen.split_whitespace().collect();

        if fields.len() != 6 {
            return Err(ChessError::InvalidFen {
                msg: format!("Invalid number of fields ({})", fields.len()) });
        }

        let (
            placement,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_counter
        ) = (
            fields[0],
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
        );

        let mut bitboards = [bitboard::EMPTY; 12];

        let mut file;
        let mut rank = 7;

        for occupancy in placement.split('/') {

            file = 0;
            for c in occupancy.chars() {       
                if file > 7 { break; }

                if let Some(empty) = c.to_digit(10) {

                    let empty = empty as usize;

                    if file + empty > 8 {
                        return Err(ChessError::InvalidFen {
                            msg: format!("Empty squares exceed board: {}", occupancy) });
                    }

                    file += empty;

                    continue;
                }

                let piece = match c {
                    'K' => WHITE_KING,
                    'Q' => WHITE_QUEEN,
                    'R' => WHITE_ROOK,
                    'B' => WHITE_BISHOP,
                    'N' => WHITE_KNIGHT,
                    'P' => WHITE_PAWN,
                    'k' => BLACK_KING,
                    'q' => BLACK_QUEEN,
                    'r' => BLACK_ROOK,
                    'b' => BLACK_BISHOP,
                    'n' => BLACK_KNIGHT,
                    'p' => BLACK_PAWN,
                    _ => return Err(ChessError::InvalidFen {
                        msg: format!("Invalid piece: {}", c) }),
                };

                bitboards[piece].set(Square::from_file_rank(file, rank));

                file += 1;
            }

            if rank == 0 { break; }

            rank -= 1;

        }

        let side_to_move = match side_to_move {
            "w" => WHITE,
            "b" => BLACK,
            _ => return Err(ChessError::InvalidFen {
                msg: format!("Invalid side to move: {}", side_to_move) }),
        };

        let mut cr = 0;
        if castling_rights != "-" {
            for c in castling_rights.chars() {
                cr |= match c {
                    'K' => 1 << 0,
                    'Q' => 1 << 1,
                    'k' => 1 << 2,
                    'q' => 1 << 3,
                    _ => return Err(ChessError::InvalidFen {
                        msg: format!("Invalid castling rights: {}", castling_rights) }),
                }
            };
        }

        let castling_rights = cr;

        let en_passant = match en_passant {
            "-" => None,
            _ => Some(Square::from_algebraic(en_passant)?),
        };

        let halfmove_clock = halfmove_clock.parse::<u32>().map_err(|_| ChessError::InvalidFen {
            msg: format!("Invalid halfmove clock: {}", halfmove_clock) })?;
        
        let fullmove_counter = fullmove_counter.parse::<u32>().map_err(|_| ChessError::InvalidFen {
            msg: format!("Invalid fullmove counter: {}", fullmove_counter) })?;

        Ok(Board {
            bitboards,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_counter,
            move_stack: Vec::new(),
        })
    }

    pub fn make_move(&mut self, mv: Move) {
        let from = mv.from_panic();
        let to = mv.to_panic();

        let toggle_mask = Bitboard::from_square(from) | Bitboard::from_square(to);

        for bb in self.bitboards.iter_mut() {
            if bb.is_occ(from) {
                *bb ^= toggle_mask
            }
        }
        
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sqr = Square::from_file_rank(file, rank);
                let mut found = false;

                for (piece, bb) in self.bitboards.iter().enumerate() {
                    if bb.is_occ(sqr) {
                        found = true;

                        let letter = match piece {
                            WHITE_KING => "K",
                            WHITE_QUEEN => "Q",
                            WHITE_ROOK => "R",
                            WHITE_BISHOP => "B",
                            WHITE_KNIGHT => "N",
                            WHITE_PAWN => "P",
                            BLACK_KING => "k",
                            BLACK_QUEEN => "q",
                            BLACK_ROOK => "r",
                            BLACK_BISHOP => "b",
                            BLACK_KNIGHT => "n",
                            BLACK_PAWN => "p",
                            _ => "?",
                        };
                        write!(f, "{letter}")?;
                        break;
                    }
                }

                if !found {
                    write!(f, "-")?;
                }

                if file != 7 { write!(f, " ")?; }
            }
            
            if rank == 7 {
                write!(f, " | Next move: {}", match self.side_to_move {
                    WHITE => "White",
                    BLACK => "Black",
                    _ => "?",
                })?;
            }

            if rank == 6 {
                write!(f, " | Castling rights: ")?;

                for i in 0..4 {
                    if self.castling_rights & (1 << i) != 0 {
                        write!(f, "{}", match i {
                            0 => "K",
                            1 => "Q",
                            2 => "k",
                            3 => "q",
                            _ => "?",
                        })?;
                    }
                }
            }

            if rank == 5 {
                write!(f, " | En passant: {}", match self.en_passant {
                    Some(sqr) => sqr.to_algebraic(),
                    None => "-".to_string(),
                })?;
            }

            if rank == 4 {
                write!(f, " | Halfmove clock: {}", self.halfmove_clock)?;
            }

            if rank == 3 {
                write!(f, " | Fullmove counter: {}", self.fullmove_counter)?;
            }

            if rank != 0 { write!(f, "\n")?; }
        }
        
        Ok(())
    }
}