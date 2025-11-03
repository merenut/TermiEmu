//! VTE ANSI/ECMA-48 parser integration
//!
//! This module integrates the `vte` crate to parse ANSI escape sequences
//! and dispatch them to the terminal emulator.

use super::{cell::CellFlags, color::Color, cursor::Cursor, grid::Grid, modes::TerminalModes};
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
    /// Primary (normal) grid
    primary_grid: Grid,
    /// Alternate screen grid (for full-screen apps like vim)
    alternate_grid: Grid,
    /// Currently active grid
    active_grid: GridType,
    /// Cursor position
    cursor: Cursor,
    /// Saved cursor position for primary screen
    saved_cursor_primary: Cursor,
    /// Saved cursor position for alternate screen
    saved_cursor_alternate: Cursor,
    /// Current foreground color
    current_fg: Color,
    /// Current background color
    current_bg: Color,
    /// Current cell flags (bold, italic, etc.)
    current_flags: CellFlags,
    /// Terminal modes
    modes: TerminalModes,
    /// Current hyperlink URL (set by OSC 8)
    current_hyperlink: Option<String>,
}

/// Enum to track which grid is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridType {
    Primary,
    Alternate,
}

impl Parser {
    /// Create a new parser with the given grid dimensions
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            parser: vte::Parser::new(),
            terminal: TerminalState {
                primary_grid: Grid::new(cols, rows, 10_000),
                alternate_grid: Grid::new(cols, rows, 0), // No scrollback for alt screen
                active_grid: GridType::Primary,
                cursor: Cursor::default(),
                saved_cursor_primary: Cursor::default(),
                saved_cursor_alternate: Cursor::default(),
                current_fg: Color::default(),
                current_bg: Color::Named(super::color::NamedColor::Background),
                current_flags: CellFlags::empty(),
                modes: TerminalModes::new(),
                current_hyperlink: None,
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

    /// Get a reference to the active grid
    pub fn grid(&self) -> &Grid {
        match self.terminal.active_grid {
            GridType::Primary => &self.terminal.primary_grid,
            GridType::Alternate => &self.terminal.alternate_grid,
        }
    }

    /// Get a mutable reference to the active grid
    pub fn grid_mut(&mut self) -> &mut Grid {
        match self.terminal.active_grid {
            GridType::Primary => &mut self.terminal.primary_grid,
            GridType::Alternate => &mut self.terminal.alternate_grid,
        }
    }

    /// Get a reference to the cursor
    pub fn cursor(&self) -> &Cursor {
        &self.terminal.cursor
    }

    /// Get a mutable reference to the cursor
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.terminal.cursor
    }

    /// Get terminal modes
    pub fn modes(&self) -> &TerminalModes {
        &self.terminal.modes
    }

    /// Check if alternate screen is active
    pub fn is_alt_screen(&self) -> bool {
        self.terminal.active_grid == GridType::Alternate
    }
}

impl TerminalState {
    /// Get the active grid
    fn grid(&mut self) -> &mut Grid {
        match self.active_grid {
            GridType::Primary => &mut self.primary_grid,
            GridType::Alternate => &mut self.alternate_grid,
        }
    }

    /// Switch to alternate screen
    fn use_alternate_screen(&mut self) {
        if self.active_grid == GridType::Primary {
            // Save current cursor position
            self.saved_cursor_primary = self.cursor.clone();
            // Switch to alternate screen
            self.active_grid = GridType::Alternate;
            // Restore alternate screen cursor
            self.cursor = self.saved_cursor_alternate.clone();
            // Set mode flag
            self.modes.insert(TerminalModes::ALT_SCREEN);
            debug!("Switched to alternate screen");
        }
    }

    /// Switch to primary screen
    fn use_primary_screen(&mut self) {
        if self.active_grid == GridType::Alternate {
            // Save alternate cursor position
            self.saved_cursor_alternate = self.cursor.clone();
            // Switch to primary screen
            self.active_grid = GridType::Primary;
            // Restore primary screen cursor
            self.cursor = self.saved_cursor_primary.clone();
            // Clear mode flag
            self.modes.remove(TerminalModes::ALT_SCREEN);
            debug!("Switched to primary screen");
        }
    }
}

impl Perform for TerminalState {
    fn print(&mut self, c: char) {
        trace!("Print: {:?}", c);

        // Get the current cursor position
        let col = self.cursor.col;
        let cols = self.grid().cols();
        let rows = self.grid().rows();

        // Check if we need to wrap to the next line
        if col >= cols {
            if self.modes.is_auto_wrap() {
                self.cursor.col = 0;
                self.cursor.row += 1;

                // Scroll if we're at the bottom
                if self.cursor.row >= rows {
                    self.grid().scroll_up(1);
                    self.cursor.row = rows - 1;
                }
            } else {
                // No wrap - just stay at the end
                self.cursor.col = cols - 1;
            }
        }

        // Create a cell with the current attributes
        let mut cell = super::cell::Cell::with_colors(c, self.current_fg, self.current_bg);
        cell.flags = self.current_flags;
        cell.hyperlink = self.current_hyperlink.clone();

        // Store cursor position before borrowing grid
        let cursor_col = self.cursor.col;
        let cursor_row = self.cursor.row;
        
        // Set the cell in the grid
        self.grid().set(cursor_col, cursor_row, cell);

        // Advance cursor
        self.cursor.col += 1;
    }

    fn execute(&mut self, byte: u8) {
        trace!("Execute: 0x{:02X}", byte);

        match byte {
            // Line feed
            0x0A => {
                let rows = self.grid().rows();
                self.cursor.row += 1;
                if self.cursor.row >= rows {
                    self.grid().scroll_up(1);
                    self.cursor.row = rows - 1;
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
                let cols = self.grid().cols();
                self.cursor.col = (self.cursor.col + 8) & !7;
                if self.cursor.col >= cols {
                    self.cursor.col = cols - 1;
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
        
        // OSC sequences format: OSC Ps ; Pt ST
        // Where Ps is a numeric parameter and Pt is text parameter
        
        if params.is_empty() {
            return;
        }
        
        // Parse the command number (Ps)
        let cmd = String::from_utf8_lossy(params[0]);
        
        match cmd.as_ref() {
            "8" => {
                // OSC 8 ; params ; URI ST
                // Hyperlink support: OSC 8 ; params ; URI ST
                // To start a hyperlink: OSC 8 ; ; https://example.com ST
                // To end a hyperlink: OSC 8 ; ; ST
                
                if params.len() >= 3 {
                    // params[1] contains optional parameters (id=..., etc.)
                    // params[2] contains the URI
                    let uri = String::from_utf8_lossy(params[2]);
                    
                    if uri.is_empty() {
                        // Empty URI means end the hyperlink
                        self.current_hyperlink = None;
                        debug!("Hyperlink ended");
                    } else {
                        // Set the current hyperlink
                        self.current_hyperlink = Some(uri.to_string());
                        debug!("Hyperlink started: {}", uri);
                    }
                } else if params.len() == 2 {
                    // Only separator, no URI - end hyperlink
                    self.current_hyperlink = None;
                    debug!("Hyperlink ended (short form)");
                }
            }
            "0" | "2" => {
                // Window title
                if params.len() >= 2 {
                    let title = String::from_utf8_lossy(params[1]);
                    debug!("Window title set: {}", title);
                    // Title handling would go here (set window title)
                }
            }
            _ => {
                debug!("Unhandled OSC command: {}", cmd);
            }
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &Params,
        intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        trace!("CSI dispatch: {:?} {}", params, action);

        // Handle private mode sequences (DEC codes)
        if !intermediates.is_empty() && intermediates[0] == b'?' {
            self.handle_decset(params, action);
            return;
        }

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
                let max_row = self.grid().rows() - 1;
                self.cursor.row = (self.cursor.row + n).min(max_row);
            }
            'C' => {
                // Cursor Forward
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                let max_col = self.grid().cols() - 1;
                self.cursor.col = (self.cursor.col + n).min(max_col);
            }
            'D' => {
                // Cursor Back
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) as usize;
                self.cursor.col = self.cursor.col.saturating_sub(n);
            }
            'H' | 'f' => {
                // Cursor Position
                let mut iter = params.iter();
                let row =
                    iter.next().and_then(|p| p.first()).copied().unwrap_or(1).saturating_sub(1)
                        as usize;
                let col =
                    iter.next().and_then(|p| p.first()).copied().unwrap_or(1).saturating_sub(1)
                        as usize;
                let max_col = self.grid().cols() - 1;
                let max_row = self.grid().rows() - 1;
                self.cursor.goto(col.min(max_col), row.min(max_row));
            }
            'J' => {
                // Erase in Display
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                let cursor_col = self.cursor.col;
                let cursor_row = self.cursor.row;
                let rows = self.grid().rows();
                match n {
                    0 => {
                        // Clear from cursor to end of screen
                        self.grid().clear_to_end_of_row(cursor_col, cursor_row);
                        for row in cursor_row + 1..rows {
                            self.grid().clear_row(row);
                        }
                    }
                    1 => {
                        // Clear from cursor to beginning of screen
                        for row in 0..cursor_row {
                            self.grid().clear_row(row);
                        }
                    }
                    2 => {
                        // Clear entire screen
                        self.grid().clear();
                    }
                    _ => {}
                }
            }
            'K' => {
                // Erase in Line
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                let cursor_col = self.cursor.col;
                let cursor_row = self.cursor.row;
                match n {
                    0 => {
                        // Clear from cursor to end of line
                        self.grid().clear_to_end_of_row(cursor_col, cursor_row);
                    }
                    1 => {
                        // Clear from cursor to beginning of line
                        for col in 0..=cursor_col {
                            if let Some(cell) = self.grid().get_mut(col, cursor_row) {
                                cell.reset();
                            }
                        }
                    }
                    2 => {
                        // Clear entire line
                        self.grid().clear_row(cursor_row);
                    }
                    _ => {}
                }
            }
            'm' => {
                // SGR - Select Graphic Rendition (colors and attributes)
                self.handle_sgr(params);
            }
            'h' | 'l' => {
                // Set/Reset mode (non-private)
                self.handle_mode(params, action == 'h');
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
                                    let r =
                                        iter.next().and_then(|p| p.first()).copied().unwrap_or(0)
                                            as u8;
                                    let g =
                                        iter.next().and_then(|p| p.first()).copied().unwrap_or(0)
                                            as u8;
                                    let b =
                                        iter.next().and_then(|p| p.first()).copied().unwrap_or(0)
                                            as u8;
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

    /// Handle DECSET/DECRST (DEC private mode set/reset)
    fn handle_decset(&mut self, params: &Params, action: char) {
        let is_set = action == 'h';
        
        for param in params.iter() {
            if let Some(&n) = param.first() {
                match n {
                    1 => {
                        // DECCKM - Application Cursor Keys
                        if is_set {
                            self.modes.insert(TerminalModes::CURSOR_KEYS_APP);
                            debug!("Application cursor keys enabled");
                        } else {
                            self.modes.remove(TerminalModes::CURSOR_KEYS_APP);
                            debug!("Application cursor keys disabled");
                        }
                    }
                    6 => {
                        // DECOM - Origin Mode
                        if is_set {
                            self.modes.insert(TerminalModes::ORIGIN_MODE);
                            debug!("Origin mode enabled");
                        } else {
                            self.modes.remove(TerminalModes::ORIGIN_MODE);
                            debug!("Origin mode disabled");
                        }
                    }
                    7 => {
                        // DECAWM - Auto-wrap Mode
                        if is_set {
                            self.modes.insert(TerminalModes::AUTO_WRAP);
                            debug!("Auto-wrap enabled");
                        } else {
                            self.modes.remove(TerminalModes::AUTO_WRAP);
                            debug!("Auto-wrap disabled");
                        }
                    }
                    25 => {
                        // DECTCEM - Cursor Visibility
                        if is_set {
                            self.modes.insert(TerminalModes::SHOW_CURSOR);
                            self.cursor.visible = true;
                            debug!("Cursor visible");
                        } else {
                            self.modes.remove(TerminalModes::SHOW_CURSOR);
                            self.cursor.visible = false;
                            debug!("Cursor hidden");
                        }
                    }
                    47 | 1047 => {
                        // Alternate screen buffer (without clearing)
                        if is_set {
                            self.use_alternate_screen();
                        } else {
                            self.use_primary_screen();
                        }
                    }
                    1048 => {
                        // Save/Restore cursor
                        if is_set {
                            // Save cursor
                            match self.active_grid {
                                GridType::Primary => {
                                    self.saved_cursor_primary = self.cursor.clone();
                                }
                                GridType::Alternate => {
                                    self.saved_cursor_alternate = self.cursor.clone();
                                }
                            }
                            debug!("Cursor saved");
                        } else {
                            // Restore cursor
                            self.cursor = match self.active_grid {
                                GridType::Primary => self.saved_cursor_primary.clone(),
                                GridType::Alternate => self.saved_cursor_alternate.clone(),
                            };
                            debug!("Cursor restored");
                        }
                    }
                    1049 => {
                        // Alternate screen buffer with cursor save/restore
                        if is_set {
                            // Save cursor and switch to alternate screen
                            self.saved_cursor_primary = self.cursor.clone();
                            self.use_alternate_screen();
                            self.grid().clear();
                        } else {
                            // Switch to primary screen and restore cursor
                            self.use_primary_screen();
                            self.cursor = self.saved_cursor_primary.clone();
                        }
                    }
                    1004 => {
                        // Focus reporting
                        if is_set {
                            self.modes.insert(TerminalModes::FOCUS_REPORT);
                            debug!("Focus reporting enabled");
                        } else {
                            self.modes.remove(TerminalModes::FOCUS_REPORT);
                            debug!("Focus reporting disabled");
                        }
                    }
                    1000 => {
                        // X10 mouse mode
                        if is_set {
                            self.modes.set_mouse_mode(super::modes::MouseMode::X10);
                            debug!("Mouse X10 mode enabled");
                        } else {
                            self.modes.set_mouse_mode(super::modes::MouseMode::None);
                            debug!("Mouse mode disabled");
                        }
                    }
                    1002 => {
                        // Button event tracking
                        if is_set {
                            self.modes.set_mouse_mode(super::modes::MouseMode::ButtonEvent);
                            debug!("Mouse button event tracking enabled");
                        } else {
                            self.modes.set_mouse_mode(super::modes::MouseMode::None);
                        }
                    }
                    1003 => {
                        // Any event tracking
                        if is_set {
                            self.modes.set_mouse_mode(super::modes::MouseMode::AnyEvent);
                            debug!("Mouse any event tracking enabled");
                        } else {
                            self.modes.set_mouse_mode(super::modes::MouseMode::None);
                        }
                    }
                    1006 => {
                        // SGR mouse mode
                        if is_set {
                            self.modes.set_mouse_mode(super::modes::MouseMode::Sgr);
                            debug!("Mouse SGR mode enabled");
                        } else {
                            self.modes.set_mouse_mode(super::modes::MouseMode::None);
                        }
                    }
                    2004 => {
                        // Bracketed paste mode
                        if is_set {
                            self.modes.insert(TerminalModes::BRACKETED_PASTE);
                            debug!("Bracketed paste enabled");
                        } else {
                            self.modes.remove(TerminalModes::BRACKETED_PASTE);
                            debug!("Bracketed paste disabled");
                        }
                    }
                    _ => {
                        debug!("Unhandled DECSET/DECRST: {} ({})", n, if is_set { "set" } else { "reset" });
                    }
                }
            }
        }
    }

    /// Handle standard mode set/reset (SM/RM)
    fn handle_mode(&mut self, params: &Params, is_set: bool) {
        for param in params.iter() {
            if let Some(&n) = param.first() {
                match n {
                    4 => {
                        // IRM - Insert/Replace Mode
                        if is_set {
                            self.modes.insert(TerminalModes::INSERT_MODE);
                            debug!("Insert mode enabled");
                        } else {
                            self.modes.remove(TerminalModes::INSERT_MODE);
                            debug!("Insert mode disabled");
                        }
                    }
                    _ => {
                        debug!("Unhandled mode: {} ({})", n, if is_set { "set" } else { "reset" });
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
