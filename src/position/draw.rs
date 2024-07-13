//! Handles drawing of the chessposition

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::Position;
impl Position {
    /// Draws a chess-board inside of a rect
    pub fn draw(&self, frame: &mut Frame, chunk: Rect) {
        // The longest possible move I would have to format is 999. Nb8xc6+ Ne5xc6+
        // NOTE: Doesn't allow for more than 1000 moves natively
        let vertical = Layout::vertical([Constraint::Min(8), Constraint::Min(1)]).split(chunk);
        let chunks = Layout::horizontal([
            Constraint::Length(16),
            Constraint::Max(2),
            Constraint::Min(20),
        ])
        .split(vertical[0]);
        self.render_board(frame, chunks[0]);
        self.render_moves(frame, chunks[2]);
        frame.render_widget(Line::from(Span::raw(&self.starting_position)), vertical[1]);
    }

    fn render_board(&self, frame: &mut Frame, chunk: Rect) {
        let mut para = Vec::new();
        for i in 0..8 {
            let mut curr = Vec::new();
            for j in 0..8 {
                let piece = self.at(i, j);
                let mut to_push = piece.map_or(' ', |x| x.piece_type.to_unicode()).to_string();
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
        frame.render_widget(Paragraph::new(para), chunk);
    }

    fn render_moves(&self, frame: &mut Frame, chunk: Rect) {
        let para = Paragraph::new(
            self.moves
                .chunks(2)
                .map(|arr| {
                    if arr.len() == 1 {
                        (arr[0], None)
                    } else {
                        (arr[0], Some(arr[1]))
                    }
                })
                .enumerate()
                .map(|(i, (a, b))| {
                    Line::from(Span::raw(format!(
                        "{:4}: {:>7}  {:<7}",
                        i + 1,
                        a.get_notation(self).unwrap(),
                        b.map_or(String::new(), |x| x.get_notation(self).unwrap())
                    )))
                })
                .collect::<Vec<_>>(),
        );
        frame.render_widget(para, chunk);
    }
}
