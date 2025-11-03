//! Iced application shell for TermiEmu

use crate::{
    pty::{Pty, PtyConfig},
    terminal::Parser,
};
use iced::{
    keyboard,
    widget::{column, container, text},
    Element, Event, Length, Subscription, Task,
};
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

/// Main application state
pub struct TermiEmuApp {
    /// Terminal parser
    parser: Parser,
    /// PTY instance (optional, may not be spawned yet)
    #[allow(clippy::arc_with_non_send_sync)]
    pty: Option<Arc<Mutex<Pty>>>,
    /// Error message if PTY fails
    error: Option<String>,
}

/// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Terminal output received
    TerminalOutput(Vec<u8>),
    /// Keyboard event
    KeyPressed(keyboard::Event),
}

impl Default for TermiEmuApp {
    fn default() -> Self {
        Self { parser: Parser::new(80, 24), pty: None, error: None }
    }
}

impl TermiEmuApp {
    /// Create a new TermiEmu application
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> (Self, Task<Message>) {
        info!("Initializing TermiEmu application");

        // Try to spawn PTY immediately
        let (pty, error) = match spawn_pty() {
            Ok(p) => (Some(Arc::new(Mutex::new(p))), None),
            Err(e) => (None, Some(format!("Failed to spawn PTY: {}", e))),
        };

        let app = Self { parser: Parser::new(80, 24), pty, error };

        (app, Task::none())
    }

    /// Handle keyboard input
    fn handle_keyboard(&mut self, event: keyboard::Event) {
        if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
            if let Some(pty_ref) = &self.pty {
                if let Ok(mut pty) = pty_ref.lock() {
                    if let Some(bytes) = key_to_bytes(key, modifiers) {
                        debug!("Sending to PTY: {:?}", bytes);
                        if let Err(e) = pty.write(&bytes) {
                            self.error = Some(format!("PTY write error: {}", e));
                        }
                    }
                }
            }
        }
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
            Message::KeyPressed(event) => {
                self.handle_keyboard(event);
                Task::none()
            }
        }
    }

    /// Subscribe to events
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            if let Event::Keyboard(kbd_event) = event {
                Some(Message::KeyPressed(kbd_event))
            } else {
                // Ignore other events for now
                None
            }
        })
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
                    if col == cursor.col && row == cursor.row && cursor.visible {
                        // Show cursor: use block for empty space, preserve char for content
                        // Note: proper cursor styling will be done with custom widget in Phase 1
                        if cell.c == ' ' {
                            content.push('█');
                        } else {
                            // For non-empty cells, show the character
                            // (inverse video would be applied in a proper renderer)
                            content.push(cell.c);
                        }
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

        let status_text = if let Some(ref err) = self.error {
            format!("Error: {}", err)
        } else if self.pty.is_some() {
            format!(
                "Grid: {}x{} | Cursor: ({}, {}) | PTY: Active",
                grid.cols(),
                grid.rows(),
                cursor.col,
                cursor.row
            )
        } else {
            format!(
                "Grid: {}x{} | Cursor: ({}, {}) | PTY: Initializing...",
                grid.cols(),
                grid.rows(),
                cursor.col,
                cursor.row
            )
        };

        let content_column = column![
            text(format!("TermiEmu v{} - Phase 0 (Pre-Alpha)", env!("CARGO_PKG_VERSION"))).size(16),
            text(status_text).size(12),
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

/// Spawn PTY synchronously
fn spawn_pty() -> anyhow::Result<Pty> {
    let config = PtyConfig {
        rows: 24,
        cols: 80,
        shell: None,
        working_directory: None,
        env: vec![("TERM".to_string(), "xterm-256color".to_string())],
    };

    let mut pty = Pty::new(config)?;
    pty.spawn()?;
    info!("PTY spawned successfully");
    Ok(pty)
}

/// Convert keyboard key and modifiers to bytes for PTY
fn key_to_bytes(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Vec<u8>> {
    use keyboard::Key;

    match key {
        // Printable characters
        Key::Character(ref s) => {
            let bytes: Vec<u8> = if modifiers.control() {
                // Ctrl key combinations
                if s.len() == 1 {
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_alphabetic() {
                        // Ctrl+A = 0x01, Ctrl+Z = 0x1A
                        vec![((c.to_ascii_lowercase() as u8) - b'a' + 1)]
                    } else {
                        // Other Ctrl combinations
                        match c {
                            '2' | '@' => vec![0x00],
                            '3' | '[' => vec![0x1b],
                            '4' | '\\' => vec![0x1c],
                            '5' | ']' => vec![0x1d],
                            '6' | '^' => vec![0x1e],
                            '7' | '_' => vec![0x1f],
                            '8' => vec![0x7f],
                            _ => s.as_bytes().to_vec(),
                        }
                    }
                } else {
                    s.as_bytes().to_vec()
                }
            } else if modifiers.alt() {
                // Alt key: prefix with ESC
                let mut bytes = vec![0x1b];
                bytes.extend_from_slice(s.as_bytes());
                bytes
            } else {
                s.as_bytes().to_vec()
            };
            Some(bytes)
        }
        // Special keys
        Key::Named(named) => {
            use keyboard::key::Named;
            let bytes = match named {
                Named::Enter => vec![b'\r'],
                Named::Tab => vec![b'\t'],
                Named::Backspace => vec![0x7f],
                Named::Escape => vec![0x1b],
                Named::ArrowUp => b"\x1b[A".to_vec(),
                Named::ArrowDown => b"\x1b[B".to_vec(),
                Named::ArrowRight => b"\x1b[C".to_vec(),
                Named::ArrowLeft => b"\x1b[D".to_vec(),
                Named::Home => b"\x1b[H".to_vec(),
                Named::End => b"\x1b[F".to_vec(),
                Named::PageUp => b"\x1b[5~".to_vec(),
                Named::PageDown => b"\x1b[6~".to_vec(),
                Named::Insert => b"\x1b[2~".to_vec(),
                Named::Delete => b"\x1b[3~".to_vec(),
                _ => return None,
            };
            Some(bytes)
        }
        _ => None,
    }
}
