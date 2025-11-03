//! Iced application shell for TermiEmu

use crate::terminal::Parser;
use iced::{
    widget::{column, container, text},
    Element, Length, Task,
};

/// Main application state
pub struct TermiEmuApp {
    /// Terminal parser
    parser: Parser,
}

/// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Terminal output received
    TerminalOutput(Vec<u8>),
}

impl Default for TermiEmuApp {
    fn default() -> Self {
        Self { parser: Parser::new(80, 24) }
    }
}

impl TermiEmuApp {
    /// Create a new TermiEmu application
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    /// Get the window title
    pub fn title(&self) -> String {
        String::from("TermiEmu - Modern Terminal Emulator")
    }

    /// Update the application state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TerminalOutput(data) => {
                // Process terminal output through parser
                self.parser.advance_bytes(&data);
                Task::none()
            }
        }
    }

    /// Create the view
    pub fn view(&self) -> Element<'_, Message> {
        let grid = &self.parser.grid();
        let cursor = self.parser.cursor();

        // Create a simple text-based view (for now)
        let mut content = String::new();

        // Render grid as text
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if let Some(cell) = grid.get(col, row) {
                    if col == cursor.col && row == cursor.row {
                        content.push('█'); // Show cursor as block
                    } else {
                        content.push(cell.c);
                    }
                } else {
                    content.push(' ');
                }
            }
            content.push('\n');
        }

        let terminal_view = text(content).font(iced::Font::MONOSPACE).size(14);

        let content_column = column![
            text(format!("TermiEmu v{} - Phase 0 (Pre-Alpha)", env!("CARGO_PKG_VERSION"))).size(16),
            text(format!(
                "Grid: {}x{} | Cursor: ({}, {})",
                grid.cols(),
                grid.rows(),
                cursor.col,
                cursor.row
            ))
            .size(12),
            terminal_view,
        ]
        .spacing(10)
        .padding(20);

        container(content_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }
}
