//! Terminal emulation core
//!
//! This module provides the core terminal emulation functionality including:
//! - Grid buffer for character storage
//! - Cursor management
//! - Color support
//! - Character attributes
//! - Scrolling and scroll regions
//! - VTE parser integration

pub mod cell;
pub mod color;
pub mod cursor;
pub mod grid;
pub mod parser;

pub use cell::{Cell, CellFlags};
pub use color::{Color, NamedColor};
pub use cursor::{Cursor, CursorStyle};
pub use grid::Grid;
pub use parser::Parser;
