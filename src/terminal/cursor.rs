//! Cursor state and management

/// Cursor style
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CursorStyle {
    /// Block cursor
    #[default]
    Block,
    /// Underline cursor
    Underline,
    /// Bar/beam cursor
    Bar,
}

/// Terminal cursor state
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// Column position (0-indexed)
    pub col: usize,
    /// Row position (0-indexed)
    pub row: usize,
    /// Cursor visibility
    pub visible: bool,
    /// Cursor style
    pub style: CursorStyle,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { col: 0, row: 0, visible: true, style: CursorStyle::default() }
    }
}

impl Cursor {
    /// Create a new cursor at position (0, 0)
    pub fn new() -> Self {
        Self::default()
    }

    /// Move cursor to specified position
    pub fn goto(&mut self, col: usize, row: usize) {
        self.col = col;
        self.row = row;
    }

    /// Move cursor to column 0 of current row
    pub fn goto_col(&mut self, col: usize) {
        self.col = col;
    }

    /// Move cursor to row 0 of current column
    pub fn goto_row(&mut self, row: usize) {
        self.row = row;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_default() {
        let cursor = Cursor::default();
        assert_eq!(cursor.col, 0);
        assert_eq!(cursor.row, 0);
        assert!(cursor.visible);
        assert_eq!(cursor.style, CursorStyle::Block);
    }

    #[test]
    fn test_cursor_goto() {
        let mut cursor = Cursor::new();
        cursor.goto(10, 5);
        assert_eq!(cursor.col, 10);
        assert_eq!(cursor.row, 5);
    }

    #[test]
    fn test_cursor_styles() {
        assert_ne!(CursorStyle::Block, CursorStyle::Bar);
        assert_ne!(CursorStyle::Bar, CursorStyle::Underline);
    }
}
