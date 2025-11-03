//! Terminal grid buffer implementation
//!
//! Provides a 2D grid for storing terminal cells with scrollback support.

use super::cell::Cell;
use std::collections::VecDeque;

/// Terminal grid with scrollback buffer
#[derive(Clone, Debug)]
pub struct Grid {
    /// Number of columns
    cols: usize,
    /// Number of rows (visible area)
    rows: usize,
    /// Current grid cells (visible area)
    cells: Vec<Cell>,
    /// Scrollback buffer (older lines)
    scrollback: VecDeque<Vec<Cell>>,
    /// Maximum scrollback lines
    max_scrollback: usize,
}

impl Grid {
    /// Create a new grid with the given dimensions
    ///
    /// # Arguments
    ///
    /// * `cols` - Number of columns
    /// * `rows` - Number of rows
    /// * `max_scrollback` - Maximum number of scrollback lines (default 10,000)
    pub fn new(cols: usize, rows: usize, max_scrollback: usize) -> Self {
        let cells = vec![Cell::default(); cols * rows];
        Self { cols, rows, cells, scrollback: VecDeque::new(), max_scrollback }
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of scrollback lines
    pub fn scrollback_len(&self) -> usize {
        self.scrollback.len()
    }

    /// Get a reference to the scrollback buffer
    pub fn scrollback(&self) -> &VecDeque<Vec<Cell>> {
        &self.scrollback
    }

    /// Get a cell at the given position
    ///
    /// Returns None if the position is out of bounds
    pub fn get(&self, col: usize, row: usize) -> Option<&Cell> {
        if col >= self.cols || row >= self.rows {
            return None;
        }
        let index = row * self.cols + col;
        self.cells.get(index)
    }

    /// Get a mutable cell at the given position
    ///
    /// Returns None if the position is out of bounds
    pub fn get_mut(&mut self, col: usize, row: usize) -> Option<&mut Cell> {
        if col >= self.cols || row >= self.rows {
            return None;
        }
        let index = row * self.cols + col;
        self.cells.get_mut(index)
    }

    /// Set a cell at the given position
    pub fn set(&mut self, col: usize, row: usize, cell: Cell) {
        if let Some(target) = self.get_mut(col, row) {
            *target = cell;
        }
    }

    /// Clear the entire grid
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.reset();
        }
    }

    /// Clear a specific row
    pub fn clear_row(&mut self, row: usize) {
        if row >= self.rows {
            return;
        }
        let start = row * self.cols;
        let end = start + self.cols;
        for cell in &mut self.cells[start..end] {
            cell.reset();
        }
    }

    /// Clear from cursor to end of row
    pub fn clear_to_end_of_row(&mut self, col: usize, row: usize) {
        if row >= self.rows {
            return;
        }
        let start = row * self.cols + col;
        let end = (row + 1) * self.cols;
        for cell in &mut self.cells[start..end] {
            cell.reset();
        }
    }

    /// Scroll up by n lines (lines move to scrollback)
    pub fn scroll_up(&mut self, n: usize) {
        for _ in 0..n {
            // Take the top row and add it to scrollback
            let mut row = Vec::with_capacity(self.cols);
            for col in 0..self.cols {
                let index = col;
                row.push(self.cells[index].clone());
            }

            // Add to scrollback (limit size)
            self.scrollback.push_back(row);
            if self.scrollback.len() > self.max_scrollback {
                self.scrollback.pop_front();
            }

            // Shift all rows up
            let cells_len = self.cells.len();
            for i in 0..(cells_len - self.cols) {
                self.cells[i] = self.cells[i + self.cols].clone();
            }

            // Clear the last row
            let last_row_start = (self.rows - 1) * self.cols;
            for i in last_row_start..cells_len {
                self.cells[i].reset();
            }
        }
    }

    /// Scroll down by n lines (insert blank lines at top)
    pub fn scroll_down(&mut self, n: usize) {
        for _ in 0..n {
            // Shift all rows down
            let last_row_start = (self.rows - 1) * self.cols;
            for i in (self.cols..=last_row_start).rev() {
                self.cells[i] = self.cells[i - self.cols].clone();
            }

            // Clear the first row
            for i in 0..self.cols {
                self.cells[i].reset();
            }
        }
    }

    /// Resize the grid
    pub fn resize(&mut self, cols: usize, rows: usize) {
        if cols == self.cols && rows == self.rows {
            return;
        }

        let mut new_cells = vec![Cell::default(); cols * rows];

        // Copy existing content (truncate or pad as needed)
        let min_rows = self.rows.min(rows);
        let min_cols = self.cols.min(cols);

        for row in 0..min_rows {
            for col in 0..min_cols {
                let old_index = row * self.cols + col;
                let new_index = row * cols + col;
                new_cells[new_index] = self.cells[old_index].clone();
            }
        }

        self.cols = cols;
        self.rows = rows;
        self.cells = new_cells;
    }

    /// Get a line from scrollback (0 = oldest)
    pub fn scrollback_line(&self, index: usize) -> Option<&Vec<Cell>> {
        self.scrollback.get(index)
    }

    /// Clear scrollback buffer
    pub fn clear_scrollback(&mut self) {
        self.scrollback.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(80, 24, 10000);
        assert_eq!(grid.cols(), 80);
        assert_eq!(grid.rows(), 24);
        assert_eq!(grid.scrollback_len(), 0);
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(80, 24, 10000);
        let cell = Cell::new('A');
        grid.set(5, 10, cell.clone());
        assert_eq!(grid.get(5, 10), Some(&cell));
    }

    #[test]
    fn test_grid_clear() {
        let mut grid = Grid::new(80, 24, 10000);
        grid.set(5, 10, Cell::new('A'));
        grid.clear();
        assert!(grid.get(5, 10).unwrap().is_empty());
    }

    #[test]
    fn test_grid_clear_row() {
        let mut grid = Grid::new(80, 24, 10000);
        grid.set(5, 10, Cell::new('A'));
        grid.clear_row(10);
        assert!(grid.get(5, 10).unwrap().is_empty());
    }

    #[test]
    fn test_grid_scroll_up() {
        let mut grid = Grid::new(80, 24, 10000);
        grid.set(0, 0, Cell::new('A'));
        grid.scroll_up(1);
        assert_eq!(grid.scrollback_len(), 1);
        // First row should be empty after scroll
        assert!(grid.get(0, 0).unwrap().is_empty());
    }

    #[test]
    fn test_grid_resize() {
        let mut grid = Grid::new(80, 24, 10000);
        grid.set(5, 5, Cell::new('A'));
        grid.resize(100, 30);
        assert_eq!(grid.cols(), 100);
        assert_eq!(grid.rows(), 30);
        // Content should be preserved
        assert_eq!(grid.get(5, 5).unwrap().c, 'A');
    }

    #[test]
    fn test_scrollback_limit() {
        let mut grid = Grid::new(80, 24, 5); // Small scrollback
        for _ in 0..10 {
            grid.scroll_up(1);
        }
        assert_eq!(grid.scrollback_len(), 5); // Should be limited to 5
    }
}
