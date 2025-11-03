//! TermiEmu - A modern, high-performance terminal emulator library
//!
//! This library provides the core functionality for the TermiEmu terminal emulator.
//!
//! # Design Philosophy
//!
//! "Fluid Minimalism with Ruthless Performance"
//!
//! # Modules
//!
//! - `clipboard`: Clipboard integration for copy/paste operations
//! - `config`: Configuration and theme system
//! - `debug`: Debug mode, diagnostics, and performance metrics
//! - `error`: Error types and error handling utilities
//! - `logging`: Logging infrastructure using `tracing`
//! - `pty`: PTY (Pseudo-Terminal) integration for process management
//! - `terminal`: Terminal emulation core (grid, parser, cursor, colors, modes, search, hyperlinks)
//! - `ui`: User interface (Iced GUI application)

pub mod clipboard;
pub mod config;
pub mod debug;
pub mod error;
pub mod logging;
pub mod pty;
pub mod terminal;
pub mod ui;
