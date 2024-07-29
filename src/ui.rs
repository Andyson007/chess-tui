//! This module is responsible for handling all ui operations
//! It uses an [`App`] instance for this

use ratatui::Frame;

use crate::{app::App, position::ScreenLayout};

/// Draws the ui.
/// It probably assumes a lot about the
/// terminal being in raw mode etc.
pub fn ui(frame: &mut Frame, app: &App) {
    app.position.draw(frame, frame.size(), ScreenLayout::Small);
}
