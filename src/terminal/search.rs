//! Terminal buffer search functionality
//!
//! Provides text search capabilities across the terminal grid and scrollback buffer.
//! Supports case-sensitive/insensitive search and regular expressions.

use super::grid::Grid;
use regex::Regex;
use std::fmt;

/// Search options
#[derive(Clone, Debug)]
pub struct SearchOptions {
    /// Case-sensitive search
    pub case_sensitive: bool,
    /// Use regular expressions
    pub use_regex: bool,
    /// Search backwards (from cursor to top)
    pub backwards: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            use_regex: false,
            backwards: false,
        }
    }
}

impl SearchOptions {
    /// Create a new SearchOptions with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set case sensitivity
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    /// Set regex mode
    pub fn use_regex(mut self, value: bool) -> Self {
        self.use_regex = value;
        self
    }

    /// Set backward search direction
    pub fn backwards(mut self, value: bool) -> Self {
        self.backwards = value;
        self
    }
}

/// A match found during search
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchMatch {
    /// Row in grid (or negative for scrollback)
    pub row: isize,
    /// Starting column
    pub start_col: usize,
    /// Ending column (exclusive)
    pub end_col: usize,
    /// Matched text
    pub text: String,
}

impl SearchMatch {
    /// Check if this match is in scrollback
    pub fn is_in_scrollback(&self) -> bool {
        self.row < 0
    }

    /// Get the absolute row in scrollback (0 = most recent scrollback line)
    pub fn scrollback_row(&self) -> Option<usize> {
        if self.row < 0 {
            Some((-self.row - 1) as usize)
        } else {
            None
        }
    }

    /// Get the row in visible grid (0-based)
    pub fn grid_row(&self) -> Option<usize> {
        if self.row >= 0 {
            Some(self.row as usize)
        } else {
            None
        }
    }
}

impl fmt::Display for SearchMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match at row {} (col {}-{}): '{}'",
            self.row, self.start_col, self.end_col, self.text
        )
    }
}

/// Search error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    /// Invalid regular expression
    InvalidRegex(String),
    /// Empty search query
    EmptyQuery,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::InvalidRegex(msg) => write!(f, "Invalid regex: {}", msg),
            SearchError::EmptyQuery => write!(f, "Search query is empty"),
        }
    }
}

impl std::error::Error for SearchError {}

/// Terminal buffer searcher
pub struct Searcher {
    /// Current search matches
    matches: Vec<SearchMatch>,
    /// Current match index (-1 if no current match)
    current_index: isize,
    /// Last search query
    last_query: Option<String>,
    /// Last search options
    last_options: SearchOptions,
}

impl Searcher {
    /// Create a new searcher
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
            current_index: -1,
            last_query: None,
            last_options: SearchOptions::default(),
        }
    }

    /// Search the grid and scrollback for a query
    pub fn search(
        &mut self,
        grid: &Grid,
        query: &str,
        options: SearchOptions,
    ) -> Result<usize, SearchError> {
        if query.is_empty() {
            return Err(SearchError::EmptyQuery);
        }

        // Clear previous matches
        self.matches.clear();
        self.current_index = -1;
        self.last_query = Some(query.to_string());
        self.last_options = options.clone();

        // Search scrollback buffer (from oldest to newest)
        if grid.scrollback_len() > 0 {
            for (scroll_idx, row_cells) in grid.scrollback().iter().enumerate() {
                let row_text: String = row_cells.iter().map(|cell| cell.c).collect();
                let row_num = -(grid.scrollback_len() as isize) + scroll_idx as isize;
                self.search_in_text(&row_text, row_num, query, &options)?;
            }
        }

        // Search visible grid
        for row in 0..grid.rows() {
            let mut row_text = String::new();
            for col in 0..grid.cols() {
                if let Some(cell) = grid.get(col, row) {
                    row_text.push(cell.c);
                }
            }
            self.search_in_text(&row_text, row as isize, query, &options)?;
        }

        // Set current index to first match if any
        if !self.matches.is_empty() {
            self.current_index = 0;
        }

        Ok(self.matches.len())
    }

    /// Search for matches in a single line of text
    fn search_in_text(
        &mut self,
        text: &str,
        row: isize,
        query: &str,
        options: &SearchOptions,
    ) -> Result<(), SearchError> {
        if options.use_regex {
            self.search_regex(text, row, query, options)
        } else {
            self.search_literal(text, row, query, options)
        }
    }

    /// Perform literal string search
    fn search_literal(
        &mut self,
        text: &str,
        row: isize,
        query: &str,
        options: &SearchOptions,
    ) -> Result<(), SearchError> {
        let search_text = if options.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        let search_query = if options.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = search_text[start..].find(&search_query) {
            let abs_pos = start + pos;
            let end_pos = abs_pos + query.len();

            // Extract original text (with original case)
            let matched_text = text[abs_pos..end_pos].to_string();

            self.matches.push(SearchMatch {
                row,
                start_col: abs_pos,
                end_col: end_pos,
                text: matched_text,
            });

            start = end_pos;
        }

        Ok(())
    }

    /// Perform regex search
    fn search_regex(
        &mut self,
        text: &str,
        row: isize,
        pattern: &str,
        options: &SearchOptions,
    ) -> Result<(), SearchError> {
        // Build regex with case sensitivity
        let pattern_with_flags = if options.case_sensitive {
            pattern.to_string()
        } else {
            format!("(?i){}", pattern)
        };

        let re = Regex::new(&pattern_with_flags)
            .map_err(|e| SearchError::InvalidRegex(e.to_string()))?;

        for mat in re.find_iter(text) {
            self.matches.push(SearchMatch {
                row,
                start_col: mat.start(),
                end_col: mat.end(),
                text: mat.as_str().to_string(),
            });
        }

        Ok(())
    }

    /// Get all matches
    pub fn matches(&self) -> &[SearchMatch] {
        &self.matches
    }

    /// Get the number of matches
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get the current match
    pub fn current_match(&self) -> Option<&SearchMatch> {
        if self.current_index >= 0 && (self.current_index as usize) < self.matches.len() {
            Some(&self.matches[self.current_index as usize])
        } else {
            None
        }
    }

    /// Get the current match index (0-based, or None if no match)
    pub fn current_match_index(&self) -> Option<usize> {
        if self.current_index >= 0 && (self.current_index as usize) < self.matches.len() {
            Some(self.current_index as usize)
        } else {
            None
        }
    }

    /// Move to the next match
    pub fn next_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.matches.len() as isize;
        self.current_match()
    }

    /// Move to the previous match
    pub fn previous_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }

        self.current_index = if self.current_index <= 0 {
            self.matches.len() as isize - 1
        } else {
            self.current_index - 1
        };
        self.current_match()
    }

    /// Clear all matches
    pub fn clear(&mut self) {
        self.matches.clear();
        self.current_index = -1;
        self.last_query = None;
    }

    /// Check if there are any matches
    pub fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    /// Get the last search query
    pub fn last_query(&self) -> Option<&str> {
        self.last_query.as_deref()
    }

    /// Get the last search options
    pub fn last_options(&self) -> &SearchOptions {
        &self.last_options
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::cell::Cell;

    fn create_test_grid() -> Grid {
        let mut grid = Grid::new(20, 3, 10);
        
        // Row 0: "Hello World!"
        let text1 = "Hello World!";
        for (col, c) in text1.chars().enumerate() {
            grid.set(col, 0, Cell::new(c));
        }
        
        // Row 1: "Testing search"
        let text2 = "Testing search";
        for (col, c) in text2.chars().enumerate() {
            grid.set(col, 1, Cell::new(c));
        }
        
        // Row 2: "hello again"
        let text3 = "hello again";
        for (col, c) in text3.chars().enumerate() {
            grid.set(col, 2, Cell::new(c));
        }
        
        grid
    }

    #[test]
    fn test_search_case_insensitive() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let options = SearchOptions::new().case_sensitive(false);
        let count = searcher.search(&grid, "hello", options).unwrap();
        
        assert_eq!(count, 2); // "Hello" and "hello"
        assert_eq!(searcher.matches().len(), 2);
        
        // First match
        let m1 = &searcher.matches()[0];
        assert_eq!(m1.row, 0);
        assert_eq!(m1.start_col, 0);
        assert_eq!(m1.end_col, 5);
        assert_eq!(m1.text, "Hello");
        
        // Second match
        let m2 = &searcher.matches()[1];
        assert_eq!(m2.row, 2);
        assert_eq!(m2.start_col, 0);
        assert_eq!(m2.end_col, 5);
        assert_eq!(m2.text, "hello");
    }

    #[test]
    fn test_search_case_sensitive() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let options = SearchOptions::new().case_sensitive(true);
        let count = searcher.search(&grid, "hello", options).unwrap();
        
        assert_eq!(count, 1); // Only lowercase "hello"
        assert_eq!(searcher.matches()[0].text, "hello");
    }

    #[test]
    fn test_search_multiple_matches_same_line() {
        let mut grid = Grid::new(20, 1, 10);
        let text = "test test test";
        for (col, c) in text.chars().enumerate() {
            grid.set(col, 0, Cell::new(c));
        }
        
        let mut searcher = Searcher::new();
        let count = searcher.search(&grid, "test", SearchOptions::new()).unwrap();
        
        assert_eq!(count, 3);
        assert_eq!(searcher.matches()[0].start_col, 0);
        assert_eq!(searcher.matches()[1].start_col, 5);
        assert_eq!(searcher.matches()[2].start_col, 10);
    }

    #[test]
    fn test_search_regex() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let options = SearchOptions::new().use_regex(true);
        let count = searcher.search(&grid, r"[Hh]ello", options).unwrap();
        
        assert_eq!(count, 2);
    }

    #[test]
    fn test_search_regex_word_boundary() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let options = SearchOptions::new().use_regex(true);
        let count = searcher.search(&grid, r"\bWorld\b", options).unwrap();
        
        assert_eq!(count, 1);
        assert_eq!(searcher.matches()[0].text, "World");
    }

    #[test]
    fn test_search_empty_query() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let result = searcher.search(&grid, "", SearchOptions::new());
        assert!(matches!(result, Err(SearchError::EmptyQuery)));
    }

    #[test]
    fn test_search_invalid_regex() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let options = SearchOptions::new().use_regex(true);
        let result = searcher.search(&grid, "[invalid", options);
        assert!(matches!(result, Err(SearchError::InvalidRegex(_))));
    }

    #[test]
    fn test_next_previous_match() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        searcher.search(&grid, "e", SearchOptions::new()).unwrap();
        assert!(searcher.match_count() > 2);
        
        // First match
        let m1 = searcher.current_match().unwrap().clone();
        assert_eq!(m1.row, 0);
        
        // Next match
        let m2 = searcher.next_match().unwrap().clone();
        assert_ne!(m1, m2);
        
        // Previous match (back to first)
        let m3 = searcher.previous_match().unwrap().clone();
        assert_eq!(m1, m3);
    }

    #[test]
    fn test_clear() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        searcher.search(&grid, "hello", SearchOptions::new()).unwrap();
        assert!(searcher.has_matches());
        
        searcher.clear();
        assert!(!searcher.has_matches());
        assert_eq!(searcher.match_count(), 0);
        assert!(searcher.last_query().is_none());
    }

    #[test]
    fn test_no_matches() {
        let grid = create_test_grid();
        let mut searcher = Searcher::new();
        
        let count = searcher.search(&grid, "xyz", SearchOptions::new()).unwrap();
        assert_eq!(count, 0);
        assert!(!searcher.has_matches());
        assert!(searcher.current_match().is_none());
    }
}
