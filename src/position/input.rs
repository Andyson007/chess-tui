use crossterm::event::MouseEvent;
use ratatui::layout::Rect;

use super::Position;

impl Position {
    /// Handles clicking on the chessboard
    pub fn handle_mouse(&mut self, rect: &Rect, mouse: MouseEvent) {
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
}
