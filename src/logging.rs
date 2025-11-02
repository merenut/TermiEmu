//! Logging infrastructure for TermiEmu
//!
//! This module configures structured logging using the `tracing` crate.
//! Log levels can be controlled via the RUST_LOG environment variable.
//!
//! # Examples
//!
//! ```bash
//! # Enable all logs
//! RUST_LOG=trace cargo run
//!
//! # Enable debug logs for TermiEmu only
//! RUST_LOG=termiemu=debug cargo run
//!
//! # Enable info logs by default
//! RUST_LOG=info cargo run
//! ```

use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the logging system
///
/// This sets up tracing with:
/// - Environment-based log level filtering (via RUST_LOG)
/// - Colored output when writing to a terminal
/// - Compact format for better readability
///
/// # Panics
///
/// Panics if the logging subscriber cannot be initialized (this should only
/// happen if called more than once).
pub fn init() {
    // Set up the environment filter
    // Default to "info" level if RUST_LOG is not set
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Configure the subscriber
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .compact()
        .init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_logging_init() {
        // Note: We can't actually test init() because it can only be called once
        // This test just ensures the module compiles
        assert!(true);
    }
}
