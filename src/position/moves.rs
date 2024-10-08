use std::collections::HashSet;

use super::{Piece, Position};

impl Piece {
    /// Gets all available moves for the current piece
    pub fn get_moves(&self, position: &Position) -> HashSet<Move> {
        todo!()
    }
}

impl Position {
    /// Gets all available moves for a given chessboard
    #[must_use]
    pub fn get_moves(&self) -> HashSet<Move> {
        // self.pos
        todo!()
    }

    /// Makes a move in place
    pub fn make_move(&mut self, r#move: Move) {
        // self.
        todo!()
    }
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct Move {
    start: Square,
    end: Square,
}

impl Move {
    // FIX: This doesn't account for disambiguation yet
    pub fn get_notation(self, pos: &Position) -> Option<String> {
        let piece = pos.at(self.start.row as usize, self.start.col as usize)?;
        Some(
            piece
                .piece_type
                .to_char()
                .into_iter()
                .chain(self.end.at(pos).map(|_| 'x'))
                .chain(self.end.to_chess_square())
                .collect::<String>(),
        )
    }

    pub const fn new(start: Square, end: Square) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Hash, Clone, Copy)]
pub struct Square {
    /// Zero indexed
    pub row: u8,
    /// Zero indexed
    pub col: u8,
}

impl Square {
    pub fn at(self, pos: &Position) -> Option<Piece> {
        pos.at(7 - self.row as usize, self.col as usize)
    }

    /// Converts itself to coordinates e.g.
    /// (0, 0) => a1
    /// (5, 7) => f8
    pub const fn to_chess_square(self) -> [char; 2] {
        [(self.row + b'a') as char, ((self.col + 1) ^ 0x30) as char]
    }

    pub fn from_chess_square(data: &[char; 2]) -> Self {
        Self::new(data[0] as u8 - b'a', (data[1] as u8 ^ 0x30) - 1)
    }

    pub fn new(row: u8, col: u8) -> Self {
        assert!(row < 8);
        assert!(col < 8);
        Self { row, col }
    }
}

#[cfg(test)]
mod test {
    use crate::position::Position;

    use super::Move;

    #[test]
    fn notation() {
        let pos = Position::default();
        let r#move = Move {
            start: super::Square { row: 0, col: 1 },
            end: super::Square { row: 2, col: 2 },
        };
        assert_eq!(r#move.get_notation(&pos), Some("Nc3".to_string()));
    }
}
