//! This module does everything that has to do with storing chess-positions

mod draw;
mod input;
mod moves;
#[derive(Debug)]
/// Stores a chess position
pub struct Position {
    fen: String,
    highlighted: Option<(u16, u16)>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            highlighted: None,
        }
    }
}

impl Position {
    /// Parses a fen and create a `Position` from it
    pub fn from(fen: impl Into<String>) -> Self {
        Self {
            fen: fen.into(),
            highlighted: None,
        }
    }

    fn at(&self, row: usize, col: usize) -> Option<Piece> {
        let split = self.fen.split('/').nth(row)?;
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

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub const fn to_unicode(self) -> char {
        match self.piece_type {
            PieceType::King => '♚',
            PieceType::Queen => '♛',
            PieceType::Rook => '♜',
            PieceType::Bishop => '♝',
            PieceType::Knight => '♞',
            PieceType::Pawn => '♟',
        }
    }

    pub fn from(a: char) -> Option<Self> {
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
