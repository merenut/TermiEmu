//! VTE ANSI/ECMA-48 parser integration
//!
//! This module integrates the `vte` crate to parse ANSI escape sequences
//! and dispatch them to the terminal emulator.

use super::{cell::CellFlags, color::Color, cursor::Cursor, grid::Grid};
use tracing::{debug, trace};
use vte::{Params, Perform};

/// Terminal parser state
pub struct Parser {
    /// VTE parser
    parser: vte::Parser,
    /// Terminal state
    terminal: TerminalState,
}

/// Internal terminal state for parser
struct TerminalState {
    grid: Grid,
    cursor: Cursor,
    current_fg: Color,
    current_bg: Color,
    current_flags: CellFlags,
}

impl Parser {
    /// Create a new parser with the given grid dimensions
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            parser: vte::Parser::new(),
            terminal: TerminalState {
                grid: Grid::new(cols, rows, 10_000),
                cursor: Cursor::default(),
                current_fg: Color::default(),
                current_bg: Color::Named(super::color::NamedColor::Background),
                current_flags: CellFlags::empty(),
            },
        }
    }

    /// Advance the parser with a byte of data
    pub fn advance(&mut self, byte: u8) {
        let bytes = [byte];
        self.parser.advance(&mut self.terminal, &bytes);
    }

    /// Advance the parser with a slice of bytes
    pub fn advance_bytes(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.terminal, bytes);
    }

    /// Get a reference to the grid
    pub fn grid(&self) -> &Grid {
        &self.terminal.grid
    }

    /// Get a mutable reference to the grid
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.terminal.grid
    }

    /// Get a reference to the cursor
    pub fn cursor(&self) -> &Cursor {
        &self.terminal.cursor
    }

    /// Get a mutable reference to the cursor
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.terminal.cursor
    }
}

impl Perform for TerminalState {
    fn print(&mut self, c: char) {
        trace!("Print: {:?}", c);

        // Get the current cursor position
        let col = self.cursor.col;

        // Check if we need to wrap to the next line
        if col >= self.grid.cols() {
            self.cursor.col = 0;
            self.cursor.row += 1;

            // Scroll if we're at the bottom
            if self.cursor.row >= self.grid.rows() {
                self.grid.scroll_up(1);
                self.cursor.row = self.grid.rows() - 1;
            }
        }

        // Create a cell with the current attributes
        let mut cell = super::cell::Cell::with_colors(c, self.current_fg, self.current_bg);
        cell.flags = self.current_flags;

        // Set the cell in the grid
        self.grid.set(self.cursor.col, self.cursor.row, cell);

        // Advance cursor
        self.cursor.col += 1;
    }

    fn execute(&mut self, byte: u8) {
        trace!("Execute: 0x{:02X}", byte);

        match byte {
            // Line feed
            0x0A => {
                self.cursor.row += 1;
                if self.cursor.row >= self.grid.rows() {
                    self.grid.scroll_up(1);
                    self.cursor.row = self.grid.rows() - 1;
                }
            }
            // Carriage return
            0x0D => {
                self.cursor.col = 0;
            }
            // Backspace
            0x08 => {
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                }
            }
            // Tab
            0x09 => {
                self.cursor.col = (self.cursor.col + 8) & !7;
                if self.cursor.col >= self.grid.cols() {
                    self.cursor.col = self.grid.cols() - 1;
                }
            }
            _ => {
                debug!("Unhandled execute: 0x{:02X}", byte);
            }
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: char) {
        trace!("Hook");
    }

    fn put(&mut self, _byte: u8) {
        trace!("Put");
    }

    fn unhook(&mut self) {
        trace!("Unhook");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        trace!("OSC dispatch: {:?}", params);
        // OSC sequences (Operating System Commands) will be handled here
        // Examples: window title, color changes, hyperlinks
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, action: char) {
        trace!("CSI dispatch: {:?} {}", params, action);

        match action {
            // Cursor movement
            'A' => {
                // Cursor Up
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                self.cursor.row = self.cursor.row.saturating_sub(n);
            }
            'B' => {
                // Cursor Down
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                self.cursor.row = (self.cursor.row + n).min(self.grid.rows() - 1);
            }
            'C' => {
                // Cursor Forward
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                self.cursor.col = (self.cursor.col + n).min(self.grid.cols() - 1);
            }
            'D' => {
                // Cursor Back
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                self.cursor.col = self.cursor.col.saturating_sub(n);
            }
            'H' | 'f' => {
                // Cursor Position
                let mut iter = params.iter();
                let row = iter
                    .next()
                    .and_then(|p| p.first())
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1) as usize;
                let col = iter
                    .next()
                    .and_then(|p| p.first())
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1) as usize;
                self.cursor.goto(col.min(self.grid.cols() - 1), row.min(self.grid.rows() - 1));
            }
            'J' => {
                // Erase in Display
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                match n {
                    0 => {
                        // Clear from cursor to end of screen
                        self.grid.clear_to_end_of_row(self.cursor.col, self.cursor.row);
                        for row in self.cursor.row + 1..self.grid.rows() {
                            self.grid.clear_row(row);
                        }
                    }
                    1 => {
                        // Clear from cursor to beginning of screen
                        for row in 0..self.cursor.row {
                            self.grid.clear_row(row);
                        }
                    }
                    2 => {
                        // Clear entire screen
                        self.grid.clear();
                    }
                    _ => {}
                }
            }
            'K' => {
                // Erase in Line
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                match n {
                    0 => {
                        // Clear from cursor to end of line
                        self.grid.clear_to_end_of_row(self.cursor.col, self.cursor.row);
                    }
                    1 => {
                        // Clear from cursor to beginning of line
                        for col in 0..=self.cursor.col {
                            if let Some(cell) = self.grid.get_mut(col, self.cursor.row) {
                                cell.reset();
                            }
                        }
                    }
                    2 => {
                        // Clear entire line
                        self.grid.clear_row(self.cursor.row);
                    }
                    _ => {}
                }
            }
            'm' => {
                // SGR - Select Graphic Rendition (colors and attributes)
                self.handle_sgr(params);
            }
            _ => {
                debug!("Unhandled CSI: {:?} {}", params, action);
            }
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        trace!("ESC dispatch");
    }
}

impl TerminalState {
    /// Handle SGR (Select Graphic Rendition) sequences
    fn handle_sgr(&mut self, params: &Params) {
        if params.is_empty() {
            // Reset all attributes
            self.current_fg = Color::default();
            self.current_bg = Color::Named(super::color::NamedColor::Background);
            self.current_flags = CellFlags::empty();
            return;
        }

        let mut iter = params.iter();
        while let Some(param) = iter.next() {
            if let Some(&n) = param.first() {
                match n {
                    0 => {
                        // Reset
                        self.current_fg = Color::default();
                        self.current_bg = Color::Named(super::color::NamedColor::Background);
                        self.current_flags = CellFlags::empty();
                    }
                    1 => self.current_flags.insert(CellFlags::BOLD),
                    2 => self.current_flags.insert(CellFlags::DIM),
                    3 => self.current_flags.insert(CellFlags::ITALIC),
                    4 => self.current_flags.insert(CellFlags::UNDERLINE),
                    7 => self.current_flags.insert(CellFlags::INVERSE),
                    8 => self.current_flags.insert(CellFlags::HIDDEN),
                    9 => self.current_flags.insert(CellFlags::STRIKETHROUGH),
                    22 => self.current_flags.remove(CellFlags::BOLD | CellFlags::DIM),
                    23 => self.current_flags.remove(CellFlags::ITALIC),
                    24 => self.current_flags.remove(CellFlags::UNDERLINE),
                    27 => self.current_flags.remove(CellFlags::INVERSE),
                    28 => self.current_flags.remove(CellFlags::HIDDEN),
                    29 => self.current_flags.remove(CellFlags::STRIKETHROUGH),
                    // Foreground colors (30-37, 90-97)
                    30..=37 => self.current_fg = Color::from_ansi((n - 30) as u8),
                    90..=97 => self.current_fg = Color::from_ansi((n - 90 + 8) as u8),
                    // Background colors (40-47, 100-107)
                    40..=47 => self.current_bg = Color::from_ansi((n - 40) as u8),
                    100..=107 => self.current_bg = Color::from_ansi((n - 100 + 8) as u8),
                    // Extended colors
                    38 | 48 => {
                        let is_fg = n == 38;
                        if let Some(color_type) = iter.next().and_then(|p| p.first()) {
                            match color_type {
                                5 => {
                                    // 256-color
                                    if let Some(&idx) = iter.next().and_then(|p| p.first()) {
                                        let color = Color::from_ansi(idx as u8);
                                        if is_fg {
                                            self.current_fg = color;
                                        } else {
                                            self.current_bg = color;
                                        }
                                    }
                                }
                                2 => {
                                    // RGB color
                                    let r = iter.next().and_then(|p| p.first()).copied().unwrap_or(0) as u8;
                                    let g = iter.next().and_then(|p| p.first()).copied().unwrap_or(0) as u8;
                                    let b = iter.next().and_then(|p| p.first()).copied().unwrap_or(0) as u8;
                                    let color = Color::Rgb(r, g, b);
                                    if is_fg {
                                        self.current_fg = color;
                                    } else {
                                        self.current_bg = color;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    39 => self.current_fg = Color::default(), // Default foreground
                    49 => self.current_bg = Color::Named(super::color::NamedColor::Background), // Default background
                    _ => {
                        debug!("Unhandled SGR parameter: {}", n);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new(80, 24);
        assert_eq!(parser.grid().cols(), 80);
        assert_eq!(parser.grid().rows(), 24);
    }

    #[test]
    fn test_parser_print() {
        let mut parser = Parser::new(80, 24);
        parser.advance(b'H');
        parser.advance(b'e');
        parser.advance(b'l');
        parser.advance(b'l');
        parser.advance(b'o');

        assert_eq!(parser.grid().get(0, 0).unwrap().c, 'H');
        assert_eq!(parser.grid().get(1, 0).unwrap().c, 'e');
        assert_eq!(parser.grid().get(4, 0).unwrap().c, 'o');
    }

    #[test]
    fn test_parser_newline() {
        let mut parser = Parser::new(80, 24);
        parser.advance(b'A');
        parser.advance(b'\n');
        parser.advance(b'\r'); // Carriage return to move to column 0
        parser.advance(b'B');

        assert_eq!(parser.grid().get(0, 0).unwrap().c, 'A');
        assert_eq!(parser.grid().get(0, 1).unwrap().c, 'B');
    }

    #[test]
    fn test_parser_carriage_return() {
        let mut parser = Parser::new(80, 24);
        parser.advance(b'A');
        parser.advance(b'B');
        parser.advance(b'\r');
        parser.advance(b'C');

        assert_eq!(parser.grid().get(0, 0).unwrap().c, 'C');
        assert_eq!(parser.grid().get(1, 0).unwrap().c, 'B');
    }

    #[test]
    fn test_parser_ansi_clear() {
        let mut parser = Parser::new(80, 24);
        // Write some text
        for byte in b"Hello" {
            parser.advance(*byte);
        }
        // Clear screen: ESC[2J
        for byte in b"\x1b[2J" {
            parser.advance(*byte);
        }
        // Grid should be empty
        assert!(parser.grid().get(0, 0).unwrap().is_empty());
    }
}
