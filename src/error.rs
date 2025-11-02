//! Error types for TermiEmu
//!
//! This module defines custom error types using `thiserror` for library-level
//! errors. Application-level code should use `anyhow` for error handling with
//! context.
//!
//! # Error Handling Strategy
//!
//! - Use `Result<T, TermError>` for recoverable errors in library code
//! - Use `anyhow::Result<T>` for application-level error handling
//! - Use `anyhow::Context` to add context to errors
//! - Avoid `unwrap()` and `expect()` in production code paths
//! - Log errors at appropriate levels before propagating

// Allow dead code since this is infrastructure for future use
#![allow(dead_code)]

use std::io;
use thiserror::Error;

/// Result type alias for TermiEmu operations
pub type Result<T> = std::result::Result<T, TermError>;

/// Custom error types for TermiEmu
#[derive(Error, Debug)]
pub enum TermError {
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// PTY (Pseudo-Terminal) error
    #[error("PTY error: {0}")]
    Pty(String),

    /// Parser error for ANSI escape sequences
    #[error("Parser error: {0}")]
    Parse(String),

    /// Rendering error
    #[error("Rendering error: {0}")]
    Render(String),

    /// Font error
    #[error("Font error: {0}")]
    Font(String),

    /// Invalid state error
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Not implemented error (for features in development)
    #[error("Not yet implemented: {0}")]
    NotImplemented(String),

    /// Generic error with custom message
    #[error("{0}")]
    Other(String),
}

impl TermError {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        TermError::Config(msg.into())
    }

    /// Create a new PTY error
    pub fn pty<S: Into<String>>(msg: S) -> Self {
        TermError::Pty(msg.into())
    }

    /// Create a new parser error
    pub fn parse<S: Into<String>>(msg: S) -> Self {
        TermError::Parse(msg.into())
    }

    /// Create a new rendering error
    pub fn render<S: Into<String>>(msg: S) -> Self {
        TermError::Render(msg.into())
    }

    /// Create a new font error
    pub fn font<S: Into<String>>(msg: S) -> Self {
        TermError::Font(msg.into())
    }

    /// Create a new invalid state error
    pub fn invalid_state<S: Into<String>>(msg: S) -> Self {
        TermError::InvalidState(msg.into())
    }

    /// Create a new not implemented error
    pub fn not_implemented<S: Into<String>>(msg: S) -> Self {
        TermError::NotImplemented(msg.into())
    }

    /// Create a new other error
    pub fn other<S: Into<String>>(msg: S) -> Self {
        TermError::Other(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = TermError::config("test config error");
        assert!(err.to_string().contains("Configuration error"));

        let err = TermError::pty("test pty error");
        assert!(err.to_string().contains("PTY error"));

        let err = TermError::parse("test parse error");
        assert!(err.to_string().contains("Parser error"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let term_err: TermError = io_err.into();
        assert!(term_err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_not_implemented() {
        let err = TermError::not_implemented("feature X");
        assert!(err.to_string().contains("Not yet implemented"));
        assert!(err.to_string().contains("feature X"));
    }
}
