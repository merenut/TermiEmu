//! Integration tests for Phase 1 features
//!
//! Tests for:
//! - US-018: Alternate Screen Buffer
//! - US-020: Terminal Mode Management  
//! - US-031: Clipboard Integration
//! - US-036: Configuration System
//! - US-037: Theme System
//! - US-041: Text Selection

use termiemu::{
    clipboard::ClipboardManager,
    config::{Config, Theme},
    terminal::{Parser, Point, Selection, TerminalModes},
};

/// Test alternate screen buffer switching
#[test]
fn test_alternate_screen_buffer() {
    let mut parser = Parser::new(80, 24);

    // Initially on primary screen
    assert!(!parser.is_alt_screen());

    // Write some text to primary screen
    parser.advance_bytes(b"Primary Screen Text");

    // Switch to alternate screen using DECSET 1049
    parser.advance_bytes(b"\x1b[?1049h");
    assert!(parser.is_alt_screen());

    // Alternate screen should be empty
    let grid = parser.grid();
    assert!(grid.get(0, 0).unwrap().is_empty());

    // Write to alternate screen
    parser.advance_bytes(b"Alternate Screen Text");

    // Switch back to primary using DECRST 1049
    parser.advance_bytes(b"\x1b[?1049l");
    assert!(!parser.is_alt_screen());

    // Primary screen text should still be there
    let grid = parser.grid();
    assert_eq!(grid.get(0, 0).unwrap().c, 'P');
}

/// Test terminal modes
#[test]
fn test_terminal_modes() {
    let mut parser = Parser::new(80, 24);
    let modes = parser.modes();

    // Default modes
    assert!(modes.is_auto_wrap());
    assert!(modes.is_cursor_visible());
    assert!(!modes.is_bracketed_paste());
    assert!(!modes.is_cursor_keys_app());

    // Enable bracketed paste mode
    parser.advance_bytes(b"\x1b[?2004h");
    assert!(parser.modes().is_bracketed_paste());

    // Disable bracketed paste mode
    parser.advance_bytes(b"\x1b[?2004l");
    assert!(!parser.modes().is_bracketed_paste());

    // Enable application cursor keys
    parser.advance_bytes(b"\x1b[?1h");
    assert!(parser.modes().is_cursor_keys_app());

    // Disable application cursor keys
    parser.advance_bytes(b"\x1b[?1l");
    assert!(!parser.modes().is_cursor_keys_app());
}

/// Test cursor visibility mode
#[test]
fn test_cursor_visibility() {
    let mut parser = Parser::new(80, 24);

    // Cursor should be visible by default
    assert!(parser.cursor().visible);

    // Hide cursor (DECTCEM reset)
    parser.advance_bytes(b"\x1b[?25l");
    assert!(!parser.cursor().visible);

    // Show cursor (DECTCEM set)
    parser.advance_bytes(b"\x1b[?25h");
    assert!(parser.cursor().visible);
}

/// Test mouse reporting modes
#[test]
fn test_mouse_modes() {
    let mut parser = Parser::new(80, 24);

    // No mouse reporting initially
    assert!(!parser.modes().is_mouse_report());

    // Enable SGR mouse mode
    parser.advance_bytes(b"\x1b[?1006h");
    assert!(parser.modes().is_mouse_report());

    // Disable mouse reporting
    parser.advance_bytes(b"\x1b[?1006l");
    assert!(!parser.modes().is_mouse_report());
}

/// Test text selection - character selection
#[test]
fn test_character_selection() {
    let selection = Selection::character(Point::new(5, 10), Point::new(15, 12));

    // Test containment
    assert!(selection.contains(Point::new(10, 11)));
    assert!(selection.contains(Point::new(5, 10)));
    assert!(selection.contains(Point::new(15, 12)));
    assert!(!selection.contains(Point::new(4, 10)));
    assert!(!selection.contains(Point::new(16, 12)));
    assert!(!selection.contains(Point::new(10, 9)));
    assert!(!selection.contains(Point::new(10, 13)));
}

/// Test text selection - line selection
#[test]
fn test_line_selection() {
    let selection = Selection::line(5, 10);

    // Any column in the selected rows should be contained
    assert!(selection.contains(Point::new(0, 5)));
    assert!(selection.contains(Point::new(50, 7)));
    assert!(selection.contains(Point::new(100, 10)));
    assert!(!selection.contains(Point::new(0, 4)));
    assert!(!selection.contains(Point::new(0, 11)));
}

/// Test text selection - block selection
#[test]
fn test_block_selection() {
    let selection = Selection::block(Point::new(10, 5), Point::new(20, 10));

    // Within the block
    assert!(selection.contains(Point::new(15, 7)));
    assert!(selection.contains(Point::new(10, 5)));
    assert!(selection.contains(Point::new(20, 10)));

    // Outside the block
    assert!(!selection.contains(Point::new(9, 7)));
    assert!(!selection.contains(Point::new(21, 7)));
    assert!(!selection.contains(Point::new(15, 4)));
    assert!(!selection.contains(Point::new(15, 11)));
}

/// Test text selection - clearing
#[test]
fn test_selection_clear() {
    let mut selection = Selection::character(Point::new(0, 0), Point::new(10, 10));
    assert!(!selection.is_empty());

    selection.clear();
    assert!(selection.is_empty());
    assert!(!selection.contains(Point::new(5, 5)));
}

/// Test text selection - update end point
#[test]
fn test_selection_update_end() {
    let mut selection = Selection::character(Point::new(0, 0), Point::new(10, 10));

    // Update the end point (simulating drag)
    selection.update_end(Point::new(20, 15));

    assert!(selection.contains(Point::new(15, 12)));
    assert!(selection.contains(Point::new(20, 15)));
}

/// Test clipboard manager creation
#[test]
fn test_clipboard_manager_creation() {
    let manager = ClipboardManager::new();
    // Just ensure it doesn't panic
    // Clipboard may not be available in CI
    let _ = manager.is_available();
}

/// Test configuration system - default config
#[test]
fn test_config_default() {
    let config = Config::default();

    assert_eq!(config.font.family, "JetBrains Mono");
    assert_eq!(config.font.size, 12.0);
    assert!(config.font.ligatures);
    assert_eq!(config.theme, "catppuccin-mocha");
    assert_eq!(config.scrollback.max_lines, 10_000);
    assert_eq!(config.window.opacity, 1.0);
}

/// Test configuration validation
#[test]
fn test_config_validation() {
    let mut config = Config::default();

    // Valid config
    assert!(config.validate().is_ok());

    // Invalid font size
    config.font.size = -1.0;
    assert!(config.validate().is_err());

    config.font.size = 100.0;
    assert!(config.validate().is_err());

    config.font.size = 12.0;
    assert!(config.validate().is_ok());

    // Invalid opacity
    config.window.opacity = -0.1;
    assert!(config.validate().is_err());

    config.window.opacity = 1.5;
    assert!(config.validate().is_err());

    config.window.opacity = 0.9;
    assert!(config.validate().is_ok());
}

/// Test configuration serialization
#[test]
fn test_config_serialization() {
    let config = Config::default();
    let toml_str = toml::to_string(&config).expect("Failed to serialize config");
    let deserialized: Config =
        toml::from_str(&toml_str).expect("Failed to deserialize config");

    assert_eq!(config.font.family, deserialized.font.family);
    assert_eq!(config.theme, deserialized.theme);
    assert_eq!(config.window.opacity, deserialized.window.opacity);
}

/// Test theme system - built-in themes
#[test]
fn test_builtin_themes() {
    assert!(Theme::builtin("catppuccin-mocha").is_some());
    assert!(Theme::builtin("tokyo-night").is_some());
    assert!(Theme::builtin("dracula").is_some());
    assert!(Theme::builtin("nord").is_some());
    assert!(Theme::builtin("nonexistent").is_none());
}

/// Test theme system - Catppuccin Mocha
#[test]
fn test_catppuccin_mocha_theme() {
    let theme = Theme::catppuccin_mocha();

    assert_eq!(theme.name, "Catppuccin Mocha");
    assert_eq!(theme.ansi.len(), 16);
    assert!(theme.description.contains("pastel"));
    assert_eq!(theme.background.r, 30);
    assert_eq!(theme.background.g, 30);
    assert_eq!(theme.background.b, 46);
}

/// Test theme system - all builtin themes are valid
#[test]
fn test_all_builtin_themes_valid() {
    for name in Theme::builtin_names() {
        let theme = Theme::builtin(name).expect("Theme should exist");
        assert_eq!(theme.ansi.len(), 16, "Theme {} has wrong number of colors", name);
        assert!(!theme.name.is_empty(), "Theme {} has empty name", name);
    }
}

/// Test theme serialization
#[test]
fn test_theme_serialization() {
    let theme = Theme::catppuccin_mocha();
    let toml_str = toml::to_string(&theme).expect("Failed to serialize theme");
    let deserialized: Theme =
        toml::from_str(&toml_str).expect("Failed to deserialize theme");

    assert_eq!(theme.name, deserialized.name);
    assert_eq!(theme.background, deserialized.background);
    assert_eq!(theme.ansi[0], deserialized.ansi[0]);
}

/// Integration test: Full terminal session with modes
#[test]
fn test_full_terminal_session() {
    let mut parser = Parser::new(80, 24);

    // Enable some modes
    parser.advance_bytes(b"\x1b[?2004h"); // Bracketed paste
    parser.advance_bytes(b"\x1b[?1006h"); // SGR mouse

    // Write some colored text
    parser.advance_bytes(b"\x1b[31mRed Text\x1b[0m");
    parser.advance_bytes(b" ");
    parser.advance_bytes(b"\x1b[32mGreen Text\x1b[0m");

    // Check grid content
    let grid = parser.grid();
    assert_eq!(grid.get(0, 0).unwrap().c, 'R');
    assert_eq!(grid.get(9, 0).unwrap().c, 'G');

    // Verify modes are still active
    assert!(parser.modes().is_bracketed_paste());
    assert!(parser.modes().is_mouse_report());

    // Switch to alternate screen
    parser.advance_bytes(b"\x1b[?1049h");
    assert!(parser.is_alt_screen());

    // Clear screen in alternate buffer
    parser.advance_bytes(b"\x1b[2J");

    // Write new content
    parser.advance_bytes(b"Alternate buffer content");

    // Switch back
    parser.advance_bytes(b"\x1b[?1049l");
    assert!(!parser.is_alt_screen());

    // Original content should still be there
    let grid = parser.grid();
    assert_eq!(grid.get(0, 0).unwrap().c, 'R');
}

/// Integration test: Configuration with theme
#[test]
fn test_config_with_theme() {
    let mut config = Config::default();
    config.theme = "tokyo-night".to_string();

    // Validate config
    assert!(config.validate().is_ok());

    // Load the theme
    let theme = Theme::builtin(&config.theme).expect("Theme should exist");
    assert_eq!(theme.name, "Tokyo Night");
    assert_eq!(theme.ansi.len(), 16);
}

/// Integration test: Selection with clipboard
#[test]
fn test_selection_with_clipboard() {
    use termiemu::terminal::{Cell, Grid};

    let mut grid = Grid::new(80, 24, 10_000);

    // Write some text to grid
    let text = "Hello, TermiEmu!";
    for (i, c) in text.chars().enumerate() {
        grid.set(i, 0, Cell::new(c));
    }

    // Create a selection
    let selection = Selection::character(Point::new(0, 0), Point::new(15, 0));

    // Extract selected text
    let selected_text = selection.get_text(&grid);
    assert_eq!(selected_text, text);

    // Try to copy to clipboard (may fail in CI)
    let mut clipboard = ClipboardManager::new();
    if clipboard.is_available() {
        let _ = clipboard.copy(&selected_text);
    }
}
