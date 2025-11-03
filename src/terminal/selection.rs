//! Text selection support for TermiEmu
//!
//! This module implements US-041: Text Selection.
//!
//! Features:
//! - Character selection (click and drag)
//! - Word selection (double-click)
//! - Line selection (triple-click)
//! - Block/rectangular selection (Alt+drag)
//! - Selection works across line wraps

use super::Grid;

/// Selection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// No active selection
    None,
    
    /// Character-based selection (normal selection)
    Character { start: Point, end: Point },
    
    /// Word-based selection (double-click)
    Word { start: Point, end: Point },
    
    /// Line-based selection (triple-click)
    Line { start_row: usize, end_row: usize },
    
    /// Block/rectangular selection (Alt+drag)
    Block { start: Point, end: Point },
}

/// A point in the terminal grid (column, row)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub col: usize,
    pub row: usize,
}

impl Point {
    /// Create a new point
    pub const fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    /// Check if this point comes before another point (reading order)
    pub fn before(&self, other: &Point) -> bool {
        self.row < other.row || (self.row == other.row && self.col < other.col)
    }

    /// Check if this point comes after another point (reading order)
    pub fn after(&self, other: &Point) -> bool {
        self.row > other.row || (self.row == other.row && self.col > other.col)
    }
}

impl Selection {
    /// Create a new character selection
    pub fn character(start: Point, end: Point) -> Self {
        Self::Character { start, end }
    }

    /// Create a new word selection
    pub fn word(start: Point, end: Point) -> Self {
        Self::Word { start, end }
    }

    /// Create a new line selection
    pub fn line(start_row: usize, end_row: usize) -> Self {
        Self::Line { start_row, end_row }
    }

    /// Create a new block selection
    pub fn block(start: Point, end: Point) -> Self {
        Self::Block { start, end }
    }

    /// Check if a point is within the selection
    pub fn contains(&self, point: Point) -> bool {
        match self {
            Self::None => false,
            Self::Character { start, end } | Self::Word { start, end } => {
                let (start, end) = order_points(*start, *end);
                !point.before(&start) && !point.after(&end)
            }
            Self::Line { start_row, end_row } => {
                let (start_row, end_row) = if start_row <= end_row {
                    (*start_row, *end_row)
                } else {
                    (*end_row, *start_row)
                };
                point.row >= start_row && point.row <= end_row
            }
            Self::Block { start, end } => {
                let (min_col, max_col) = if start.col <= end.col {
                    (start.col, end.col)
                } else {
                    (end.col, start.col)
                };
                let (min_row, max_row) = if start.row <= end.row {
                    (start.row, end.row)
                } else {
                    (end.row, start.row)
                };
                point.col >= min_col
                    && point.col <= max_col
                    && point.row >= min_row
                    && point.row <= max_row
            }
        }
    }

    /// Get the selected text from a grid
    pub fn get_text(&self, grid: &Grid) -> String {
        match self {
            Self::None => String::new(),
            Self::Character { start, end } | Self::Word { start, end } => {
                get_text_in_range(grid, *start, *end)
            }
            Self::Line { start_row, end_row } => {
                let (start_row, end_row) = if start_row <= end_row {
                    (*start_row, *end_row)
                } else {
                    (*end_row, *start_row)
                };
                let mut text = String::new();
                for row in start_row..=end_row.min(grid.rows() - 1) {
                    for col in 0..grid.cols() {
                        if let Some(cell) = grid.get(col, row) {
                            text.push(cell.c);
                        }
                    }
                    if row < end_row {
                        text.push('\n');
                    }
                }
                text.trim_end().to_string()
            }
            Self::Block { start, end } => {
                let (min_col, max_col) = if start.col <= end.col {
                    (start.col, end.col)
                } else {
                    (end.col, start.col)
                };
                let (min_row, max_row) = if start.row <= end.row {
                    (start.row, end.row)
                } else {
                    (end.row, start.row)
                };
                let mut text = String::new();
                for row in min_row..=max_row.min(grid.rows() - 1) {
                    for col in min_col..=max_col.min(grid.cols() - 1) {
                        if let Some(cell) = grid.get(col, row) {
                            text.push(cell.c);
                        }
                    }
                    if row < max_row {
                        text.push('\n');
                    }
                }
                text
            }
        }
    }

    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Clear the selection
    pub fn clear(&mut self) {
        *self = Self::None;
    }

    /// Update the end point of the selection (for drag operations)
    pub fn update_end(&mut self, end: Point) {
        match self {
            Self::Character { end: ref mut e, .. }
            | Self::Word { end: ref mut e, .. }
            | Self::Block { end: ref mut e, .. } => {
                *e = end;
            }
            _ => {}
        }
    }
}

/// Order two points so that start comes before end
fn order_points(a: Point, b: Point) -> (Point, Point) {
    if a.before(&b) {
        (a, b)
    } else {
        (b, a)
    }
}

/// Get text in a range from the grid
fn get_text_in_range(grid: &Grid, start: Point, end: Point) -> String {
    let (start, end) = order_points(start, end);
    let mut text = String::new();

    if start.row == end.row {
        // Single line selection
        for col in start.col..=end.col.min(grid.cols() - 1) {
            if let Some(cell) = grid.get(col, start.row) {
                text.push(cell.c);
            }
        }
    } else {
        // Multi-line selection
        // First line (from start.col to end of line)
        for col in start.col..grid.cols() {
            if let Some(cell) = grid.get(col, start.row) {
                text.push(cell.c);
            }
        }
        text.push('\n');

        // Middle lines (full lines)
        for row in (start.row + 1)..end.row {
            for col in 0..grid.cols() {
                if let Some(cell) = grid.get(col, row) {
                    text.push(cell.c);
                }
            }
            text.push('\n');
        }

        // Last line (from start to end.col)
        if end.row < grid.rows() {
            for col in 0..=end.col.min(grid.cols() - 1) {
                if let Some(cell) = grid.get(col, end.row) {
                    text.push(cell.c);
                }
            }
        }
    }

    text.trim_end().to_string()
}

/// Find word boundaries at a given point
pub fn word_boundaries(grid: &Grid, point: Point) -> (Point, Point) {
    let row = point.row;
    if row >= grid.rows() {
        return (point, point);
    }

    // Find start of word (scan backwards)
    let mut start_col = point.col;
    while start_col > 0 {
        if let Some(cell) = grid.get(start_col - 1, row) {
            if is_word_char(cell.c) {
                start_col -= 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Find end of word (scan forwards)
    let mut end_col = point.col;
    while end_col < grid.cols() - 1 {
        if let Some(cell) = grid.get(end_col + 1, row) {
            if is_word_char(cell.c) {
                end_col += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (Point::new(start_col, row), Point::new(end_col, row))
}

/// Check if a character is part of a word
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::Cell;

    #[test]
    fn test_point_ordering() {
        let p1 = Point::new(5, 10);
        let p2 = Point::new(10, 10);
        let p3 = Point::new(5, 11);

        assert!(p1.before(&p2));
        assert!(p2.after(&p1));
        assert!(p1.before(&p3));
        assert!(p3.after(&p1));
    }

    #[test]
    fn test_selection_contains() {
        let sel = Selection::character(Point::new(5, 10), Point::new(15, 12));

        assert!(sel.contains(Point::new(10, 10)));
        assert!(sel.contains(Point::new(5, 10)));
        assert!(sel.contains(Point::new(15, 12)));
        assert!(!sel.contains(Point::new(4, 10)));
        assert!(!sel.contains(Point::new(16, 12)));
    }

    #[test]
    fn test_line_selection_contains() {
        let sel = Selection::line(10, 15);

        assert!(sel.contains(Point::new(0, 10)));
        assert!(sel.contains(Point::new(50, 12)));
        assert!(sel.contains(Point::new(100, 15)));
        assert!(!sel.contains(Point::new(0, 9)));
        assert!(!sel.contains(Point::new(0, 16)));
    }

    #[test]
    fn test_block_selection_contains() {
        let sel = Selection::block(Point::new(5, 10), Point::new(15, 15));

        assert!(sel.contains(Point::new(10, 12)));
        assert!(sel.contains(Point::new(5, 10)));
        assert!(sel.contains(Point::new(15, 15)));
        assert!(!sel.contains(Point::new(4, 12)));
        assert!(!sel.contains(Point::new(16, 12)));
        assert!(!sel.contains(Point::new(10, 9)));
        assert!(!sel.contains(Point::new(10, 16)));
    }

    #[test]
    fn test_selection_get_text_single_line() {
        let mut grid = Grid::new(20, 5, 100);
        
        // Set up some text
        for (i, c) in "Hello World".chars().enumerate() {
            grid.set(i, 0, Cell::new(c));
        }

        let sel = Selection::character(Point::new(0, 0), Point::new(4, 0));
        let text = sel.get_text(&grid);
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_selection_clear() {
        let mut sel = Selection::character(Point::new(0, 0), Point::new(10, 10));
        assert!(!sel.is_empty());
        
        sel.clear();
        assert!(sel.is_empty());
    }

    #[test]
    fn test_is_word_char() {
        assert!(is_word_char('a'));
        assert!(is_word_char('Z'));
        assert!(is_word_char('0'));
        assert!(is_word_char('_'));
        assert!(is_word_char('-'));
        assert!(is_word_char('.'));
        assert!(!is_word_char(' '));
        assert!(!is_word_char('/'));
        assert!(!is_word_char('!'));
    }
}
