//! Clipboard integration for TermiEmu
//!
//! This module implements US-031: Clipboard Integration.
//!
//! Features:
//! - Copy text to system clipboard
//! - Paste text from system clipboard
//! - Bracketed paste mode support (protects against malicious pastes)
//! - Cross-platform clipboard support (Linux, macOS, Windows)

use anyhow::{Context, Result};
use arboard::Clipboard;
use tracing::{debug, warn};

/// Clipboard manager
///
/// Provides a simple interface for clipboard operations with automatic
/// initialization and error handling.
pub struct ClipboardManager {
    clipboard: Option<Clipboard>,
}

impl ClipboardManager {
    /// Create a new clipboard manager
    ///
    /// Note: Clipboard initialization may fail on some platforms (e.g., headless systems).
    /// In such cases, clipboard operations will fail gracefully.
    pub fn new() -> Self {
        match Clipboard::new() {
            Ok(clipboard) => {
                debug!("Clipboard initialized successfully");
                Self { clipboard: Some(clipboard) }
            }
            Err(e) => {
                warn!("Failed to initialize clipboard: {}. Clipboard operations will be disabled.", e);
                Self { clipboard: None }
            }
        }
    }

    /// Copy text to the clipboard
    ///
    /// # Arguments
    ///
    /// * `text` - The text to copy to the clipboard
    ///
    /// # Errors
    ///
    /// Returns an error if the clipboard is not available or the operation fails.
    pub fn copy(&mut self, text: &str) -> Result<()> {
        if let Some(ref mut clipboard) = self.clipboard {
            clipboard
                .set_text(text)
                .context("Failed to copy text to clipboard")?;
            debug!("Copied {} bytes to clipboard", text.len());
            Ok(())
        } else {
            anyhow::bail!("Clipboard not available");
        }
    }

    /// Paste text from the clipboard
    ///
    /// # Errors
    ///
    /// Returns an error if the clipboard is not available or the operation fails.
    pub fn paste(&mut self) -> Result<String> {
        if let Some(ref mut clipboard) = self.clipboard {
            let text = clipboard
                .get_text()
                .context("Failed to paste text from clipboard")?;
            debug!("Pasted {} bytes from clipboard", text.len());
            Ok(text)
        } else {
            anyhow::bail!("Clipboard not available");
        }
    }

    /// Check if the clipboard is available
    pub fn is_available(&self) -> bool {
        self.clipboard.is_some()
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Format text for bracketed paste mode
///
/// Wraps the text with escape sequences that indicate the start and end
/// of a paste operation. This protects against malicious pastes that could
/// execute commands.
///
/// # Arguments
///
/// * `text` - The text to wrap
///
/// # Returns
///
/// A string with bracketed paste escape sequences: `\x1b[200~{text}\x1b[201~`
pub fn format_bracketed_paste(text: &str) -> String {
    format!("\x1b[200~{}\x1b[201~", text)
}

/// Check if text contains potentially dangerous characters
///
/// Returns true if the text contains newlines or other characters that
/// could cause unintended command execution.
pub fn is_multiline_or_dangerous(text: &str) -> bool {
    text.contains('\n') || text.contains('\r')
}

/// Strip trailing whitespace from text
///
/// Useful for cleaning up copied text before pasting.
pub fn strip_trailing_whitespace(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_creation() {
        let manager = ClipboardManager::new();
        // Clipboard availability depends on the environment
        // Just ensure it doesn't panic
        let _ = manager.is_available();
    }

    #[test]
    fn test_format_bracketed_paste() {
        let text = "hello world";
        let result = format_bracketed_paste(text);
        assert_eq!(result, "\x1b[200~hello world\x1b[201~");
    }

    #[test]
    fn test_format_bracketed_paste_multiline() {
        let text = "line1\nline2\nline3";
        let result = format_bracketed_paste(text);
        assert_eq!(result, "\x1b[200~line1\nline2\nline3\x1b[201~");
    }

    #[test]
    fn test_is_multiline_or_dangerous() {
        assert!(!is_multiline_or_dangerous("single line"));
        assert!(is_multiline_or_dangerous("multi\nline"));
        assert!(is_multiline_or_dangerous("with\rcarriage return"));
        assert!(!is_multiline_or_dangerous(""));
    }

    #[test]
    fn test_strip_trailing_whitespace() {
        let text = "hello world  \nfoo bar   \nbaz   ";
        let result = strip_trailing_whitespace(text);
        assert_eq!(result, "hello world\nfoo bar\nbaz");
    }

    #[test]
    fn test_strip_trailing_whitespace_single_line() {
        let text = "hello world   ";
        let result = strip_trailing_whitespace(text);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_strip_trailing_whitespace_empty() {
        let text = "";
        let result = strip_trailing_whitespace(text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_clipboard_copy_paste_roundtrip() {
        let mut manager = ClipboardManager::new();
        
        // Skip test if clipboard is not available (e.g., in CI)
        if !manager.is_available() {
            println!("Clipboard not available, skipping test");
            return;
        }

        let test_text = "Hello, TermiEmu!";
        
        // Copy text
        match manager.copy(test_text) {
            Ok(()) => {
                // Try to paste it back
                match manager.paste() {
                    Ok(pasted) => {
                        assert_eq!(pasted, test_text);
                    }
                    Err(e) => {
                        println!("Paste failed (may be expected in some environments): {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Copy failed (may be expected in some environments): {}", e);
            }
        }
    }
}
