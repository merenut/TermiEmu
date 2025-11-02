# Code Quality Standards

This document describes the code quality standards and tools used in TermiEmu.

## Overview

TermiEmu maintains high code quality standards through:
- **Automated Formatting** with rustfmt
- **Linting** with clippy
- **Continuous Integration** checks on all pull requests
- **Optional Pre-commit Hooks** for local development

## Formatting with rustfmt

### Configuration

The project uses rustfmt for consistent code formatting. Configuration is in `rustfmt.toml`.

Key settings:
- Max line width: 100 characters
- Edition: 2021
- Tab spaces: 4
- Auto-reorder imports and modules

### Usage

```bash
# Check if code is formatted correctly
cargo fmt --check

# Format all code
cargo fmt

# Format specific file
rustfmt src/main.rs
```

### Editor Integration

Most editors support automatic formatting on save:

**VS Code:** Install the "rust-analyzer" extension and add to `settings.json`:
```json
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**IntelliJ/CLion:** Enable "Reformat code on save" in Settings → Tools → Actions on Save

**Vim/Neovim:** Use the `rust.vim` plugin or configure with ALE/coc.nvim

## Linting with Clippy

### Configuration

Clippy is configured in `clippy.toml` with project-specific settings.

Key settings:
- Cognitive complexity threshold: 30
- Type complexity threshold: 500
- Maximum arguments: 8 (for terminal state management)

### Usage

```bash
# Run clippy with default settings
cargo clippy

# Run clippy on all targets with all features
cargo clippy --all-targets --all-features

# Treat warnings as errors (what CI does)
cargo clippy --all-targets --all-features -- -D warnings

# Automatically fix issues where possible
cargo clippy --fix
```

### Common Clippy Warnings

**Unnecessary clone:** Use references instead of cloning when possible
```rust
// Bad
let s = string.clone();
process(s);

// Good
process(&string);
```

**Redundant pattern matching:** Use if-let or while-let
```rust
// Bad
match value {
    Some(v) => println!("{}", v),
    None => {}
}

// Good
if let Some(v) = value {
    println!("{}", v);
}
```

**Needless borrowing:** Don't borrow when not needed
```rust
// Bad
foo(&value);

// Good (if foo takes T, not &T)
foo(value);
```

## Pre-commit Hooks (Optional)

Pre-commit hooks automatically run checks before each commit to catch issues early.

### Setup

1. Add cargo-husky to your development dependencies:
```bash
cargo install cargo-edit
cargo add --dev cargo-husky --features user-hooks
```

2. The hooks will be automatically installed on next build.

3. Create `.cargo-husky/hooks/pre-commit` (if not exists):
```bash
#!/bin/sh

# Run cargo fmt check
echo "Running rustfmt..."
cargo fmt --all -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code is not formatted. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy found issues. Fix them or use 'cargo clippy --fix'."
    exit 1
fi

echo "✅ All checks passed!"
```

4. Make it executable:
```bash
chmod +x .cargo-husky/hooks/pre-commit
```

### Bypassing Hooks

If you need to commit without running hooks (not recommended):
```bash
git commit --no-verify -m "Your message"
```

## Continuous Integration

All pull requests must pass CI checks:

1. **Format Check:** Code must be properly formatted
2. **Clippy:** No warnings with `-D warnings` flag
3. **Build:** Code must compile on Linux, macOS, and Windows
4. **Tests:** All tests must pass

### Local CI Simulation

Run the same checks that CI runs:
```bash
# Format check
cargo fmt --all -- --check

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build --verbose

# Test
cargo test --verbose

# Release build
cargo build --release --verbose
```

## Code Review Standards

During code review, we check for:

### Code Organization
- Clear module structure
- Appropriate use of visibility modifiers
- Well-named functions and variables
- Logical separation of concerns

### Documentation
- Public APIs documented with rustdoc comments
- Complex logic explained with inline comments
- Module-level documentation for context

### Error Handling
- Proper use of Result types
- Meaningful error messages
- No unwrap() in production code paths
- Context preserved through error chain

### Performance
- Unnecessary allocations avoided
- Efficient data structures chosen
- Hot paths optimized
- Benchmarks for critical sections

### Safety
- Unsafe code minimized and justified
- FFI boundaries properly handled
- Memory safety ensured
- No data races possible

### Testing
- Unit tests for individual functions
- Integration tests for workflows
- Edge cases covered
- Property-based tests for complex logic

## Style Guidelines

### Naming Conventions

```rust
// Types: PascalCase
struct TerminalGrid { }
enum CursorStyle { }

// Functions and variables: snake_case
fn process_escape_sequence() { }
let cell_width = 10;

// Constants: SCREAMING_SNAKE_CASE
const MAX_SCROLLBACK: usize = 10_000;

// Type parameters: Single uppercase letter or PascalCase
fn generic<T>(value: T) { }
fn generic<Message>(value: Message) { }
```

### Import Organization

```rust
// Standard library
use std::collections::HashMap;
use std::io::{self, Write};

// External crates
use iced::{Application, Element};
use vte::Parser;

// Internal modules
use crate::grid::Grid;
use crate::terminal::Terminal;
```

### Comments

```rust
// Good: Explains why, not what
// Buffer size chosen to match typical terminal output bursts
const BUFFER_SIZE: usize = 8192;

// Bad: Repeats what code does
// Set buffer size to 8192
const BUFFER_SIZE: usize = 8192;
```

### Function Length

- Keep functions focused and short (< 50 lines ideal)
- Extract complex logic into helper functions
- Use clear function names that describe purpose

### Error Handling

```rust
// Good: Proper error propagation
fn load_config() -> Result<Config, ConfigError> {
    let content = fs::read_to_string("config.toml")?;
    toml::from_str(&content).map_err(ConfigError::Parse)
}

// Bad: Unwrapping in library code
fn load_config() -> Config {
    let content = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&content).unwrap()
}
```

## Performance Guidelines

### Profiling

Before optimizing, profile to find actual bottlenecks:
```bash
# CPU profiling
cargo flamegraph --bin termiemu

# Benchmarking
cargo bench

# Memory profiling
cargo instruments -t alloc --bin termiemu
```

### Common Optimizations

**Avoid allocations in hot paths:**
```rust
// Bad: Allocates on every call
fn process_char(c: char) -> String {
    format!("{}", c)
}

// Good: Reuse buffer
fn process_char(c: char, buf: &mut String) {
    buf.clear();
    write!(buf, "{}", c).unwrap();
}
```

**Use appropriate collection types:**
```rust
// For known size: Vec with capacity
let mut cells = Vec::with_capacity(rows * cols);

// For small collections: SmallVec
let mut buffer = SmallVec::<[u8; 16]>::new();

// For frequent lookups: HashMap/BTreeMap
let mut cache = HashMap::new();
```

## Tools

### Useful Cargo Commands

```bash
# Check without building
cargo check

# Build documentation
cargo doc --open

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Watch for changes and rebuild
cargo watch -x check -x test

# Expand macros
cargo expand

# View dependency tree
cargo tree
```

### Additional Tools

```bash
# Install useful development tools
cargo install cargo-watch
cargo install cargo-expand
cargo install cargo-edit
cargo install cargo-outdated
cargo install cargo-audit
cargo install cargo-flamegraph
```

## Troubleshooting

### "cargo fmt failed"
**Solution:** Your code has syntax errors. Fix them first, then run fmt.

### "clippy found warnings"
**Solution:** Read the warnings carefully. Most have suggestions for fixes. Use `cargo clippy --fix` for automatic fixes.

### "hooks are not running"
**Solution:** Ensure hooks are executable: `chmod +x .cargo-husky/hooks/*`

### "CI passed locally but failed on GitHub"
**Solution:** Ensure you're running the exact same commands as CI. Check platform-specific issues.

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [rustfmt Documentation](https://rust-lang.github.io/rustfmt/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)

## Questions?

If you have questions about code quality standards, please:
1. Check this document first
2. Review existing code for examples
3. Ask in pull request discussions
4. Open an issue for clarification
