//! Phase 2 (Beta - Feature Completion) Integration Tests
//!
//! Tests for advanced features including:
//! - Hyperlink support (OSC 8)
//! - Search functionality
//! - Debug mode
//! - Performance benchmarking infrastructure

use termiemu::terminal::{
    parser::Parser,
    search::{SearchOptions, Searcher},
    cell::Cell,
    grid::Grid,
};

#[test]
fn test_hyperlink_osc8_basic() {
    let mut parser = Parser::new(80, 24);
    
    // Start a hyperlink: ESC ] 8 ; ; http://example.com ESC \
    let hyperlink_start = b"\x1b]8;;http://example.com\x1b\\";
    parser.advance_bytes(hyperlink_start);
    
    // Write some text
    parser.advance_bytes(b"click here");
    
    // End the hyperlink: ESC ] 8 ; ; ESC \
    let hyperlink_end = b"\x1b]8;;\x1b\\";
    parser.advance_bytes(hyperlink_end);
    
    // Write more text
    parser.advance_bytes(b" for more info");
    
    // Check the grid
    let grid = parser.grid();
    
    // First cell should have "c" with hyperlink
    let cell_0 = grid.get(0, 0).unwrap();
    assert_eq!(cell_0.c, 'c');
    assert!(cell_0.has_hyperlink());
    assert_eq!(cell_0.get_hyperlink(), Some("http://example.com"));
    
    // Cell at position 9 should have "e" with hyperlink
    let cell_9 = grid.get(9, 0).unwrap();
    assert_eq!(cell_9.c, 'e');
    assert!(cell_9.has_hyperlink());
    assert_eq!(cell_9.get_hyperlink(), Some("http://example.com"));
    
    // Cell at position 10 should have space without hyperlink
    let cell_10 = grid.get(10, 0).unwrap();
    assert_eq!(cell_10.c, ' ');
    assert!(!cell_10.has_hyperlink());
    
    // Cell at position 11 should have "f" without hyperlink
    let cell_11 = grid.get(11, 0).unwrap();
    assert_eq!(cell_11.c, 'f');
    assert!(!cell_11.has_hyperlink());
}

#[test]
fn test_hyperlink_osc8_multiple() {
    let mut parser = Parser::new(80, 24);
    
    // First hyperlink
    parser.advance_bytes(b"\x1b]8;;http://first.com\x1b\\");
    parser.advance_bytes(b"link1");
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    
    // Space
    parser.advance_bytes(b" ");
    
    // Second hyperlink
    parser.advance_bytes(b"\x1b]8;;http://second.com\x1b\\");
    parser.advance_bytes(b"link2");
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    
    let grid = parser.grid();
    
    // Check first link
    let cell_0 = grid.get(0, 0).unwrap();
    assert!(cell_0.has_hyperlink());
    assert_eq!(cell_0.get_hyperlink(), Some("http://first.com"));
    
    // Check space between
    let cell_5 = grid.get(5, 0).unwrap();
    assert!(!cell_5.has_hyperlink());
    
    // Check second link
    let cell_6 = grid.get(6, 0).unwrap();
    assert!(cell_6.has_hyperlink());
    assert_eq!(cell_6.get_hyperlink(), Some("http://second.com"));
}

#[test]
fn test_hyperlink_osc8_with_params() {
    let mut parser = Parser::new(80, 24);
    
    // Hyperlink with ID parameter: ESC ] 8 ; id=123 ; http://example.com ESC \
    parser.advance_bytes(b"\x1b]8;id=123;http://example.com\x1b\\");
    parser.advance_bytes(b"text");
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    
    let grid = parser.grid();
    let cell = grid.get(0, 0).unwrap();
    
    // Should still have the URL (params are currently ignored but URL is preserved)
    assert!(cell.has_hyperlink());
    assert_eq!(cell.get_hyperlink(), Some("http://example.com"));
}

#[test]
fn test_hyperlink_osc8_empty_closes() {
    let mut parser = Parser::new(80, 24);
    
    // Start hyperlink
    parser.advance_bytes(b"\x1b]8;;http://example.com\x1b\\");
    parser.advance_bytes(b"A");
    
    // Verify hyperlink is set
    assert!(parser.grid().get(0, 0).unwrap().has_hyperlink());
    
    // Close hyperlink (empty URI)
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    parser.advance_bytes(b"B");
    
    // Second character should not have hyperlink
    assert!(!parser.grid().get(1, 0).unwrap().has_hyperlink());
}

#[test]
fn test_hyperlink_with_special_chars() {
    let mut parser = Parser::new(80, 24);
    
    // URL with query string and fragment
    let url = "https://example.com/path?query=value&foo=bar#section";
    let sequence = format!("\x1b]8;;{}\x1b\\", url);
    parser.advance_bytes(sequence.as_bytes());
    parser.advance_bytes(b"link");
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    
    let grid = parser.grid();
    let cell = grid.get(0, 0).unwrap();
    assert!(cell.has_hyperlink());
    assert_eq!(cell.get_hyperlink(), Some(url));
}

#[test]
fn test_hyperlink_across_scrolling() {
    let mut parser = Parser::new(10, 3);
    
    // Fill first row with hyperlink
    parser.advance_bytes(b"\x1b]8;;http://test.com\x1b\\");
    parser.advance_bytes(b"1234567890");
    parser.advance_bytes(b"\x1b]8;;\x1b\\");
    
    // Scroll by adding more lines
    parser.advance_bytes(b"\r\n");
    parser.advance_bytes(b"line 2");
    parser.advance_bytes(b"\r\n");
    parser.advance_bytes(b"line 3");
    parser.advance_bytes(b"\r\n");
    parser.advance_bytes(b"line 4");
    
    // First row should be in scrollback now
    let grid = parser.grid();
    assert!(grid.scrollback_len() > 0);
}

#[test]
fn test_no_hyperlink_by_default() {
    let mut parser = Parser::new(80, 24);
    
    // Write text without hyperlink
    parser.advance_bytes(b"normal text");
    
    let grid = parser.grid();
    for col in 0..11 {
        let cell = grid.get(col, 0).unwrap();
        assert!(!cell.has_hyperlink());
    }
}

// ============================================================================
// Search Functionality Tests (US-043)
// ============================================================================

#[test]
fn test_search_basic() {
    let mut grid = Grid::new(20, 3, 10);
    
    // Add some text
    let text = "Hello World";
    for (col, c) in text.chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    let count = searcher.search(&grid, "World", SearchOptions::new()).unwrap();
    
    assert_eq!(count, 1);
    assert!(searcher.has_matches());
    
    let m = searcher.current_match().unwrap();
    assert_eq!(m.start_col, 6);
    assert_eq!(m.end_col, 11);
    assert_eq!(m.text, "World");
}

#[test]
fn test_search_case_insensitive() {
    let mut grid = Grid::new(20, 3, 10);
    
    for (col, c) in "Hello HELLO hello".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    let options = SearchOptions::new().case_sensitive(false);
    let count = searcher.search(&grid, "hello", options).unwrap();
    
    assert_eq!(count, 3);
}

#[test]
fn test_search_case_sensitive() {
    let mut grid = Grid::new(20, 3, 10);
    
    for (col, c) in "Hello HELLO hello".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    let options = SearchOptions::new().case_sensitive(true);
    let count = searcher.search(&grid, "hello", options).unwrap();
    
    assert_eq!(count, 1);
    
    let m = searcher.current_match().unwrap();
    assert_eq!(m.text, "hello");
}

#[test]
fn test_search_regex() {
    let mut grid = Grid::new(30, 1, 10);
    
    for (col, c) in "abc123 def456 ghi789".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    let options = SearchOptions::new().use_regex(true);
    let count = searcher.search(&grid, r"\d+", options).unwrap();
    
    assert_eq!(count, 3);
    assert_eq!(searcher.matches()[0].text, "123");
    assert_eq!(searcher.matches()[1].text, "456");
    assert_eq!(searcher.matches()[2].text, "789");
}

#[test]
fn test_search_navigate_matches() {
    let mut grid = Grid::new(20, 1, 10);
    
    for (col, c) in "test test test".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    searcher.search(&grid, "test", SearchOptions::new()).unwrap();
    
    assert_eq!(searcher.match_count(), 3);
    assert_eq!(searcher.current_match_index(), Some(0));
    
    // Navigate forward
    searcher.next_match();
    assert_eq!(searcher.current_match_index(), Some(1));
    
    searcher.next_match();
    assert_eq!(searcher.current_match_index(), Some(2));
    
    // Wrap around
    searcher.next_match();
    assert_eq!(searcher.current_match_index(), Some(0));
    
    // Navigate backward
    searcher.previous_match();
    assert_eq!(searcher.current_match_index(), Some(2));
}

#[test]
fn test_search_with_scrollback() {
    let mut parser = Parser::new(10, 2);
    
    // Fill grid and create scrollback
    parser.advance_bytes(b"line1\r\n");
    parser.advance_bytes(b"line2\r\n");
    parser.advance_bytes(b"line3\r\n");
    
    let grid = parser.grid();
    assert!(grid.scrollback_len() > 0);
    
    let mut searcher = Searcher::new();
    let count = searcher.search(grid, "line", SearchOptions::new()).unwrap();
    
    // Should find "line" in both scrollback and visible grid
    assert!(count >= 2);
}

#[test]
fn test_search_empty_query() {
    let grid = Grid::new(10, 10, 10);
    let mut searcher = Searcher::new();
    
    let result = searcher.search(&grid, "", SearchOptions::new());
    assert!(result.is_err());
}

#[test]
fn test_search_clear() {
    let mut grid = Grid::new(20, 1, 10);
    
    for (col, c) in "test".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    searcher.search(&grid, "test", SearchOptions::new()).unwrap();
    
    assert!(searcher.has_matches());
    
    searcher.clear();
    
    assert!(!searcher.has_matches());
    assert_eq!(searcher.match_count(), 0);
    assert!(searcher.last_query().is_none());
}

#[test]
fn test_search_no_matches() {
    let mut grid = Grid::new(20, 1, 10);
    
    for (col, c) in "hello".chars().enumerate() {
        grid.set(col, 0, Cell::new(c));
    }
    
    let mut searcher = Searcher::new();
    let count = searcher.search(&grid, "xyz", SearchOptions::new()).unwrap();
    
    assert_eq!(count, 0);
    assert!(!searcher.has_matches());
    assert!(searcher.current_match().is_none());
}
