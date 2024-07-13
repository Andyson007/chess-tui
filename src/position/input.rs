use crossterm::event::{KeyCode, MouseEvent};
use ratatui::layout::Rect;

use super::{
    moves::{Move, Square},
    Position,
};

impl Position {
    /// Handles clicking on the chessboard
    pub fn handle_mouse(&mut self, rect: &Rect, mouse: MouseEvent) {
        self.highlighted = None;
        let row = i32::from(mouse.row) - i32::from(rect.x);
        let column = i32::from(mouse.column) - i32::from(rect.y);
        if row < 0 || column < 0 {
            return;
        }
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let row = row as u16;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let column = column as u16;

        if row >= rect.width || column >= rect.height {
            return;
        }
        // NOTE: This assumes that the chessboard is located top left
        let x = row;
        let y = column / 2;
        self.highlighted = Some((x, y));
    }

    /// Handles keyboard events
    pub fn handle_keyboard(&mut self, code: KeyCode) {
        if !matches!(code, KeyCode::Char(' ')) {
            return;
        }
        self.moves
            .push(Move::new(Square::new(0, 1), Square::new(2, 2)));
    }
}
