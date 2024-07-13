//! This module does everything that has to do with storing chess-positions

use self::moves::Move;

mod draw;
mod input;
mod moves;
#[derive(Debug)]
/// Stores a chess position
pub struct Position {
    /// This is the position the analysis started in
    starting_position: String,
    moves: Vec<Move>,
    highlighted: Option<(u16, u16)>,
}

impl Default for Position {
    fn default() -> Self {
        Self::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Position {
    /// Parses a fen and create a `Position` from it
    pub fn from(fen: impl Into<String>) -> Self {
        Self {
            starting_position: fen.into(),
            moves: Vec::new(),
            highlighted: None,
        }
    }

    fn at(&self, row: usize, col: usize) -> Option<Piece> {
        let split = self.starting_position.split('/').nth(row)?;
        let Ok(mut col): Result<isize, <usize as TryInto<isize>>::Error> = col.try_into() else {
            return None;
        };
        for c in split.chars() {
            match col.cmp(&0) {
                std::cmp::Ordering::Less => return None,
                std::cmp::Ordering::Equal => return Piece::from(c),
                std::cmp::Ordering::Greater => match c {
                    a @ '0'..='9' => col = col + 48 - a as isize,
                    _ => col -= 1,
                },
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black,
}

impl Color {
    pub const fn color(self) -> ratatui::style::Color {
        match self {
            Self::White => ratatui::style::Color::Rgb(255, 255, 255),
            Self::Black => ratatui::style::Color::Rgb(0, 0, 0),
        }
    }
}

/// Defines a chesspiece with its type and color
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    /// Takes a piece from a fen and converts it into a piece
    fn from(a: char) -> Option<Self> {
        if a.is_numeric() {
            None
        } else {
            Some(Self {
                color: if a.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                },
                piece_type: PieceType::from(a)?,
            })
        }
    }
}

impl PieceType {
    pub const fn to_char(self) -> Option<char> {
        Some(match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Bishop => 'B',
            Self::Knight => 'N',
            Self::Pawn => return None,
        })
    }
    /// Generates the unicode character resembeling the current piece
    pub const fn to_unicode(self) -> char {
        match self {
            Self::King => '♚',
            Self::Queen => '♛',
            Self::Rook => '♜',
            Self::Bishop => '♝',
            Self::Knight => '♞',
            Self::Pawn => '♟',
        }
    }

    pub const fn from(c: char) -> Option<Self> {
        Some(match c.to_ascii_uppercase() {
            'K' => Self::King,
            'Q' => Self::Queen,
            'B' => Self::Bishop,
            'N' => Self::Knight,
            'R' => Self::Rook,
            'P' => Self::Pawn,
            _ => return None,
        })
    }
}
