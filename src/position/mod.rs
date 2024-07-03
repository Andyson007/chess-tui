//! This module does everything that has to do with storing chess-positions

mod draw;
#[derive(Debug)]
/// Stores a chess position
pub struct Position {
    fen: String,
}

impl Position {
    /// Parses a fen and create a `Position` from it
    pub fn from(fen: impl Into<String>) -> Self {
        Self { fen: fen.into() }
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

#[derive(Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct Piece {
    pub piece: PieceType,
    pub color: Color,
}

impl Piece {
    pub const fn to_char(self) -> char {
        match self.piece {
            PieceType::King => '♚',
            PieceType::Queen => '♛',
            PieceType::Rook => '♜',
            PieceType::Bishop => '♝',
            PieceType::Knight => '♞',
            PieceType::Pawn => '♟',
        }
        // match (self.piece, self.color) {
        //     (PieceType::King, Color::White) => '♔',
        //     (PieceType::King, Color::Black) => '♚',
        //     (PieceType::Queen, Color::White) => '♕',
        //     (PieceType::Queen, Color::Black) => '♛',
        //     (PieceType::Rook, Color::White) => '♖',
        //     (PieceType::Rook, Color::Black) => '♜',
        //     (PieceType::Bishop, Color::White) => '♗',
        //     (PieceType::Bishop, Color::Black) => '♝',
        //     (PieceType::Knight, Color::White) => '♘',
        //     (PieceType::Knight, Color::Black) => '♞',
        //     (PieceType::Pawn, Color::White) => '♙',
        //     (PieceType::Pawn, Color::Black) => '♟',
        // }
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
                piece: PieceType::from(a)?,
            })
        }
    }
}

impl PieceType {
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
