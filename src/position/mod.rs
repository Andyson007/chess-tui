//! This module does everything that has to do with storing chess-positions

use self::moves::{Move, Square};

mod draw;
mod input;
mod moves;
#[derive(Debug)]
/// Stores a chess position
pub struct Position {
    /// A fen
    starting_position: String,
    turn: Color,
    castling_rights: CastingRights,
    halfmove_count: u8,
    movecount: u64,
    moves: Vec<Move>,
    highlighted: Option<Square>,
    en_passant: Option<Square>,
}

impl Default for Position {
    fn default() -> Self {
        Self::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl Position {
    /// Parses a fen and create a `Position` from it
    pub fn try_from<T>(fen: T) -> Option<Self>
    where
        T: Clone + Into<String>,
    {
        let split = fen
            .clone()
            .into()
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        Some(Self {
            starting_position: fen.into(),
            turn: if split[1] == "w" {
                Color::White
            } else {
                Color::Black
            },
            castling_rights: CastingRights::from(&split[2]),
            en_passant: None,
            halfmove_count: split[4].parse::<u8>().ok()?,
            movecount: split[5].parse::<u64>().ok()?,
            moves: Vec::new(),
            highlighted: None,
        })
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

#[derive(Debug)]
struct CastingRights {
    kingside_white: bool,
    queenside_white: bool,
    kingside_black: bool,
    queenside_black: bool,
}

impl CastingRights {
    pub fn to_string(&self) -> String {
        [
            if self.kingside_white { 'K' } else { '-' },
            if self.queenside_white { 'Q' } else { '-' },
            if self.kingside_black { 'k' } else { '-' },
            if self.queenside_black { 'q' } else { '-' },
        ]
        .into_iter()
        .collect()
    }

    pub fn from(raw: impl Into<String>) -> Self {
        let raw = raw.into().chars().collect::<Vec<char>>();
        assert_eq!(raw.len(), 4);
        Self {
            kingside_white: raw[0] != '-',
            kingside_black: raw[1] != '-',
            queenside_white: raw[2] != '-',
            queenside_black: raw[3] != '-',
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Describes the size of the chess board when it gets rendered
pub enum ScreenLayout {
    /// Unicode chess pieces
    Small,
    /// Ascii Art chess pieces
    Large,
}
