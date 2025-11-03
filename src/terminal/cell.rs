//! Terminal grid cell representation

use super::color::Color;
use bitflags::bitflags;

bitflags! {
    /// Cell attribute flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct CellFlags: u8 {
        /// Bold text
        const BOLD = 0b0000_0001;
        /// Dim/faint text
        const DIM = 0b0000_0010;
        /// Italic text
        const ITALIC = 0b0000_0100;
        /// Underline text
        const UNDERLINE = 0b0000_1000;
        /// Inverse/reverse video
        const INVERSE = 0b0001_0000;
        /// Hidden/invisible text
        const HIDDEN = 0b0010_0000;
        /// Strikethrough text
        const STRIKETHROUGH = 0b0100_0000;
        /// Wide character (CJK, emoji)
        const WIDE_CHAR = 0b1000_0000;
    }
}

/// A single cell in the terminal grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The character in this cell
    pub c: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text attributes (bold, italic, etc.)
    pub flags: CellFlags,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color::default(),
            bg: Color::Named(super::color::NamedColor::Background),
            flags: CellFlags::empty(),
        }
    }
}

impl Cell {
    /// Create a new cell with the given character
    pub fn new(c: char) -> Self {
        Self { c, ..Default::default() }
    }

    /// Create a new cell with character and colors
    pub fn with_colors(c: char, fg: Color, bg: Color) -> Self {
        Self { c, fg, bg, flags: CellFlags::empty() }
    }

    /// Check if this cell is empty (contains only whitespace)
    pub fn is_empty(&self) -> bool {
        self.c == ' ' && self.flags.is_empty()
    }

    /// Reset the cell to default state
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.c, ' ');
        assert!(cell.flags.is_empty());
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new('A');
        assert_eq!(cell.c, 'A');
    }

    #[test]
    fn test_cell_flags() {
        let mut cell = Cell::default();
        cell.flags = CellFlags::BOLD | CellFlags::ITALIC;
        assert!(cell.flags.contains(CellFlags::BOLD));
        assert!(cell.flags.contains(CellFlags::ITALIC));
        assert!(!cell.flags.contains(CellFlags::UNDERLINE));
    }

    #[test]
    fn test_cell_is_empty() {
        let cell = Cell::default();
        assert!(cell.is_empty());

        let cell = Cell::new('A');
        assert!(!cell.is_empty());
    }
}
