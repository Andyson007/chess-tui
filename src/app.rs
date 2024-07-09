//! The main module.
//! implements App and all of its features

use crossterm::event::KeyCode;

use crate::{position::Position, engine::Engine};

#[derive(Debug)]
/// Contains all state information of the app
pub struct App {
    /// The chess position currently displayed
    pub position: Position,
    /// The stockfish instance
    pub stockfish: Engine,
}

impl Default for App {
    fn default() -> Self {
        Self {
            position: Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            stockfish: Engine::new(),
        }
    }
}

impl App {
    /// Handles input
    /// # Return values
    /// returns true if the app should exit
    pub fn handle_input(&mut self, code: KeyCode) -> bool {
        matches!(code, KeyCode::Esc | KeyCode::Char('q'))
    }
}
