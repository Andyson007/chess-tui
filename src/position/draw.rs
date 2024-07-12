//! Handles drawing of the chessposition

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::{Piece, Position};
impl Position {
    /// Draws a chess-board inside of a rect
    pub fn draw(&self, frame: &mut Frame, chunk: Rect) {
        let mut para = Vec::new();
        for i in 0..8 {
            let mut curr = Vec::new();
            for j in 0..8 {
                let piece = self.at(i, j);
                let mut to_push = piece.map_or(' ', Piece::to_unicode).to_string();
                to_push.push(' ');
                curr.push(Span::styled(
                    to_push,
                    Style::new()
                        .bg(
                            #[allow(clippy::collapsible_else_if)]
                            if (i + j) & 1 == 0 {
                                if self.highlighted.is_some_and(|(a, b)| {
                                    usize::from(a) == i && usize::from(b) == j
                                }) {
                                    Color::LightBlue
                                } else {
                                    Color::Blue
                                }
                            } else {
                                if self.highlighted.is_some_and(|(a, b)| {
                                    usize::from(a) == i && usize::from(b) == j
                                }) {
                                    Color::LightGreen
                                } else {
                                    Color::Green
                                }
                            },
                        )
                        .fg(piece.map_or(Color::Green, |x| x.color.color())),
                ));
            }
            para.push(Line::from(curr));
        }
        para.push(Line::from(Span::raw(self.starting_position.clone())));
        frame.render_widget(Paragraph::new(para), chunk);
    }
}
