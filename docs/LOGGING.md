# Logging Guide

TermiEmu uses the `tracing` crate for structured logging. This guide explains how to use logging in the project.

## Quick Start

### Running with Logs

Control log output using the `RUST_LOG` environment variable:

```bash
# Show all logs (most verbose)
RUST_LOG=trace cargo run

# Show debug and above
RUST_LOG=debug cargo run

# Show info and above (default)
RUST_LOG=info cargo run

# Show only warnings and errors
RUST_LOG=warn cargo run

# Show only errors
RUST_LOG=error cargo run
```

### Filtering by Module

You can filter logs by module:

```bash
# Only show logs from the termiemu crate
RUST_LOG=termiemu=debug cargo run

# Show debug logs from termiemu, warn from everything else
RUST_LOG=warn,termiemu=debug cargo run

# Multiple modules
RUST_LOG=termiemu::parser=trace,termiemu::pty=debug cargo run
```

## Using Logging in Code

### Basic Logging

```rust
use tracing::{trace, debug, info, warn, error};

// Trace level - Very detailed information, typically only enabled during debugging
trace!("Processing byte: {:#x}", byte);

// Debug level - Detailed information for developers
debug!("Grid dimensions: {}x{}", cols, rows);

// Info level - General informational messages
info!("Terminal initialized successfully");

// Warning level - Something unexpected but not necessarily an error
warn!("Using fallback font, preferred font not found");

// Error level - An error occurred
error!("Failed to spawn PTY: {}", err);
```

### Structured Logging

Add structured fields to log messages:

```rust
use tracing::{info, warn};

info!(
    rows = grid.rows,
    cols = grid.cols,
    "Grid resized"
);

warn!(
    font_family = %config.font.family,
    fallback = %fallback_font,
    "Font not found, using fallback"
);
```

### Instrumenting Functions

Use the `#[instrument]` macro to automatically log function entry and exit:

```rust
use tracing::{instrument, info};

#[instrument]
fn process_command(command: &str) {
    info!("Processing command");
    // Function implementation
}

// This will automatically log:
// TRACE: -> process_command{command="ls"}
// INFO: Processing command
// TRACE: <- process_command{command="ls"}
```

### Instrumenting with Skip

Skip logging certain parameters:

```rust
use tracing::instrument;

#[instrument(skip(large_buffer))]
fn process_data(name: &str, large_buffer: &[u8]) {
    // large_buffer won't be logged
}
```

## Log Levels

Choose the appropriate log level:

| Level | Purpose | Example Use Cases |
|-------|---------|-------------------|
| TRACE | Very detailed, step-by-step execution | Byte-by-byte parsing, every frame render |
| DEBUG | Detailed diagnostic information | Configuration values, state changes |
| INFO  | General informational messages | Application lifecycle, user actions |
| WARN  | Unexpected but handled situations | Missing fonts, deprecated features |
| ERROR | Error conditions | Failed operations, exceptions |

## Performance Considerations

### Conditional Compilation

Use conditional logging for performance-sensitive code:

```rust
use tracing::debug;

// This check happens at runtime
if tracing::enabled!(tracing::Level::DEBUG) {
    let expensive_value = expensive_computation();
    debug!("Value: {}", expensive_value);
}
```

### Avoid Expensive Operations in Log Statements

```rust
// BAD - always computes even if not logged
debug!("Data: {}", expensive_serialization());

// GOOD - only computes if debug logging is enabled
if tracing::enabled!(tracing::Level::DEBUG) {
    debug!("Data: {}", expensive_serialization());
}
```

## Production Logging

In production builds, consider:

1. Set default level to `info` or `warn`
2. Avoid logging sensitive information (passwords, tokens, etc.)
3. Use structured logging for easier parsing and analysis
4. Implement log rotation to prevent disk space issues
5. Consider sending logs to a logging service

## Examples

### Logging in the PTY Module

```rust
use tracing::{debug, error, info, instrument};

#[instrument(skip(data))]
pub fn write_to_pty(&mut self, data: &[u8]) -> Result<()> {
    debug!(len = data.len(), "Writing to PTY");
    
    match self.pty.write_all(data) {
        Ok(_) => {
            info!("Successfully wrote {} bytes to PTY", data.len());
            Ok(())
        }
        Err(e) => {
            error!("Failed to write to PTY: {}", e);
            Err(e.into())
        }
    }
}
```

### Logging in the Parser

```rust
use tracing::{trace, warn};

pub fn parse_escape_sequence(&mut self, seq: &str) {
    trace!(sequence = %seq, "Parsing escape sequence");
    
    match self.parse_csi(seq) {
        Ok(cmd) => self.execute_command(cmd),
        Err(e) => {
            warn!(sequence = %seq, error = %e, "Invalid escape sequence");
        }
    }
}
```

## Best Practices

1. **Use appropriate levels**: Don't log everything at INFO level
2. **Add context**: Include relevant information (IDs, dimensions, states)
3. **Be concise**: Log messages should be clear and brief
4. **Don't log in hot paths**: Avoid logging in performance-critical loops
5. **Use structured fields**: Makes logs easier to parse and analyze
6. **Don't log secrets**: Never log passwords, tokens, or sensitive data
7. **Test with logging enabled**: Ensure logging doesn't break functionality
8. **Document log messages**: Help future developers understand the logs

## Troubleshooting

### Logs not appearing

1. Check if `RUST_LOG` is set correctly
2. Ensure `logging::init()` is called in `main()`
3. Verify the log level is not too restrictive

### Too many logs

1. Increase the log level: `RUST_LOG=warn` or `RUST_LOG=error`
2. Filter by module: `RUST_LOG=termiemu::pty=info`
3. Review and adjust log statements in code

### Performance issues

1. Disable trace and debug logs in production
2. Use conditional logging for expensive operations
3. Profile with and without logging enabled
4. Consider using async logging for high-volume scenarios

## Resources

- [tracing crate documentation](https://docs.rs/tracing/)
- [tracing-subscriber documentation](https://docs.rs/tracing-subscriber/)
- [RUST_LOG syntax](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)
