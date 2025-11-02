# Contributing to TermiEmu

Thank you for your interest in contributing to TermiEmu! We're building a modern, high-performance terminal emulator in Rust, and we welcome contributions from developers of all experience levels.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment Setup](#development-environment-setup)
- [Building and Testing](#building-and-testing)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Submitting Changes](#submitting-changes)
- [Issue Guidelines](#issue-guidelines)
- [Getting Help](#getting-help)

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

Before contributing, please:

1. **Review the Documentation:**
   - [Production Roadmap](./PRODUCTION_ROADMAP.md) - Overview of planned features and user stories
   - [Architecture](./ARCHITECTURE.md) - System design and technical architecture
   - [Design Summary](./DESIGN_SUMMARY.md) - Key design decisions and rationale
   - [Getting Started](./GETTING_STARTED.md) - Developer guide with code examples

2. **Check Existing Issues:**
   - Look for issues labeled `good first issue` or `help wanted`
   - Comment on an issue to express interest before starting work
   - Ask questions if anything is unclear

3. **Understand the Project Status:**
   - We're currently in Phase 0 (Pre-Alpha) - Foundation stage
   - Focus is on establishing core terminal emulation infrastructure
   - See the [Production Roadmap](./PRODUCTION_ROADMAP.md) for current priorities

## Development Environment Setup

### Prerequisites

**Required:**
- Rust 1.70.0 or later ([install from rustup.rs](https://rustup.rs/))
- Git

**Optional but Recommended:**
- A code editor with Rust support (VS Code with rust-analyzer, IntelliJ IDEA, etc.)
- Platform-specific development tools (see below)

### Platform-Specific Setup

#### Linux

**Ubuntu/Debian:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional dependencies (for future GUI dependencies)
sudo apt-get update
sudo apt-get install build-essential pkg-config libfontconfig1-dev
```

**Fedora/RHEL:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional dependencies
sudo dnf install gcc pkg-config fontconfig-devel
```

**Arch Linux:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional dependencies
sudo pacman -S base-devel fontconfig
```

#### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Xcode Command Line Tools (if not already installed)
xcode-select --install
```

#### Windows

1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with C++ development tools
3. Restart your terminal after installation

### Clone and Build

```bash
# Fork the repository on GitHub first, then clone your fork
git clone https://github.com/YOUR_USERNAME/TermiEmu.git
cd TermiEmu

# Add the upstream repository
git remote add upstream https://github.com/merenut/TermiEmu.git

# Build the project
cargo build

# Run the project (currently minimal implementation)
cargo run

# Verify tests pass
cargo test
```

## Building and Testing

### Build Commands

```bash
# Development build (faster, with debug symbols)
cargo build

# Release build (optimized, takes longer)
cargo build --release

# Check code without building (fast)
cargo check

# Check with all features
cargo check --all-features

# Build documentation
cargo doc --no-deps --open
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name::

# Run benchmarks (when implemented)
cargo bench
```

### Code Quality

```bash
# Format code (run before committing)
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run linter (must pass with no warnings)
cargo clippy

# Run linter with all features
cargo clippy --all-targets --all-features

# More strict clippy (recommended during development)
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit
```

### Continuous Integration

Our CI pipeline runs on every push and pull request:
- **Formatting:** `cargo fmt --check`
- **Linting:** `cargo clippy -- -D warnings`
- **Building:** Debug and release builds
- **Testing:** All tests on Linux, macOS, and Windows
- **Rust Versions:** Stable and nightly
- **Security:** Dependency vulnerability scanning
- **Documentation:** Ensures `cargo doc` builds without warnings

All CI checks must pass before a PR can be merged.

## Development Workflow

### Branch Strategy

1. **Main Branch (`main`):** Stable code, protected
2. **Development Branch (`develop`):** Integration branch for features
3. **Feature Branches:** Your work goes here

### Creating a Feature Branch

```bash
# Update your fork
git fetch upstream
git checkout develop
git merge upstream/develop

# Create a feature branch
git checkout -b feature/my-feature-name

# Or for a bug fix
git checkout -b fix/issue-number-description
```

### Making Changes

1. **Make your changes** in small, logical commits
2. **Write clear commit messages:**
   ```
   Short summary (50 chars or less)

   More detailed explanation if needed. Wrap at 72 characters.
   Explain what and why, not how.

   - Bullet points are fine
   - Use present tense ("Add feature" not "Added feature")
   - Reference issues with #issue-number
   ```

3. **Test your changes thoroughly:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Add clear, descriptive commit message"
   ```

### Keeping Your Branch Updated

```bash
# Fetch latest changes from upstream
git fetch upstream

# Rebase your branch on latest develop
git rebase upstream/develop

# Force push to your fork (only do this on your own branches!)
git push --force-with-lease origin feature/my-feature-name
```

## Coding Standards

### Rust Style

We follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and use automated tools to enforce style:

- **Formatting:** Automated with `rustfmt` (see `rustfmt.toml`)
- **Linting:** Enforced with `clippy` (see `clippy.toml`)
- **Line Length:** 100 characters (configured in `rustfmt.toml`)
- **Edition:** Rust 2021

### Code Organization

```
src/
├── main.rs              # Application entry point
├── app.rs               # Application state and event loop
├── terminal/            # Terminal state and logic
│   ├── mod.rs
│   ├── grid.rs          # Grid data structure
│   └── cursor.rs        # Cursor management
├── parser/              # ANSI escape sequence parsing
│   └── mod.rs
├── pty/                 # PTY abstraction layer
│   ├── mod.rs
│   ├── unix.rs          # Unix implementation
│   └── windows.rs       # Windows implementation
└── renderer/            # Rendering system
    └── mod.rs
```

### Documentation

- **Public APIs:** Must have doc comments with `///`
- **Modules:** Should have module-level documentation
- **Examples:** Include usage examples in doc comments when helpful
- **Safety:** Document any `unsafe` code with safety invariants

Example:
```rust
/// Represents a cell in the terminal grid.
///
/// Each cell contains a character, foreground/background colors,
/// and text attributes (bold, italic, etc.).
///
/// # Examples
///
/// ```
/// use termiemu::terminal::Cell;
///
/// let cell = Cell::new('A');
/// assert_eq!(cell.c, 'A');
/// ```
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
    pub flags: CellFlags,
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `thiserror` or `anyhow` for error types
- Avoid `unwrap()` and `expect()` in production code
- Provide context when propagating errors

Example:
```rust
use anyhow::{Context, Result};

fn read_config(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .context(format!("Failed to read config file: {}", path.display()))?;
    
    toml::from_str(&contents)
        .context("Failed to parse config file")
}
```

### Testing

- Write unit tests for individual functions and modules
- Write integration tests for end-to-end scenarios
- Use property-based testing (`proptest`) for complex logic
- Aim for >80% code coverage

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new('A');
        assert_eq!(cell.c, 'A');
        assert_eq!(cell.fg, Color::default_foreground());
        assert_eq!(cell.bg, Color::default_background());
    }

    #[test]
    fn test_grid_resize() {
        let mut grid = Grid::new(80, 24);
        grid.resize(120, 30);
        assert_eq!(grid.cols(), 120);
        assert_eq!(grid.rows(), 30);
    }
}
```

### Performance Considerations

- Profile before optimizing
- Avoid allocations in hot paths
- Use `&str` instead of `String` when possible
- Consider using `smallvec` for small, stack-allocated collections
- Document performance-critical code sections

## Submitting Changes

### Before Submitting a Pull Request

1. **Ensure all tests pass:**
   ```bash
   cargo test
   ```

2. **Ensure code is formatted:**
   ```bash
   cargo fmt
   ```

3. **Ensure clippy is happy:**
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Update documentation if needed:**
   - Update relevant `.md` files
   - Add/update doc comments
   - Run `cargo doc` to verify

5. **Add tests for new functionality:**
   - Unit tests for new functions
   - Integration tests for new features
   - Update existing tests if behavior changes

### Creating a Pull Request

1. **Push your branch to your fork:**
   ```bash
   git push origin feature/my-feature-name
   ```

2. **Open a Pull Request on GitHub:**
   - Go to the [TermiEmu repository](https://github.com/merenut/TermiEmu)
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template (created automatically)

3. **PR Title:** Use a clear, descriptive title
   - Good: "Implement basic PTY integration for Linux"
   - Bad: "Fix stuff"

4. **PR Description:** Include:
   - What changes were made
   - Why the changes were made
   - How to test the changes
   - Links to related issues (`Closes #123`, `Relates to #456`)
   - Screenshots/GIFs for UI changes

### Example PR Description

```markdown
## Summary
Implements the basic PTY (pseudo-terminal) integration for Unix-like systems, enabling the terminal to spawn and communicate with shell processes.

## Changes
- Added `pty` module with Unix implementation using `portable-pty` crate
- Implemented process spawning with environment variable passing
- Added PTY resizing support (TIOCSWINSZ)
- Added signal handling for child process termination

## Testing
- Unit tests for PTY creation and basic I/O
- Integration test spawning bash and capturing output
- Tested on Linux (Ubuntu 22.04) and macOS (Ventura)

## Related Issues
Closes #11 (US-011: Implement PTY Integration)

## Checklist
- [x] Tests pass (`cargo test`)
- [x] Code is formatted (`cargo fmt`)
- [x] Clippy is happy (`cargo clippy`)
- [x] Documentation updated
- [x] Tested on Linux
- [x] Tested on macOS
```

### Code Review Process

1. **Automated Checks:** CI must pass (builds, tests, linting)
2. **Maintainer Review:** A maintainer will review your code
3. **Feedback:** Address any requested changes
4. **Approval:** Once approved, your PR will be merged
5. **Recognition:** You'll be added to CONTRIBUTORS.md (if not already there)

**Review Timeline:**
- Initial response: Within 2-3 days
- Full review: Within 1 week
- Complex PRs may take longer

**What Reviewers Look For:**
- Code correctness and safety
- Test coverage
- Documentation quality
- Code style and consistency
- Performance implications
- Security considerations

## Issue Guidelines

### Creating Issues

**Bug Reports:**
Use the bug report template and include:
- Clear description of the problem
- Steps to reproduce
- Expected vs. actual behavior
- System information (OS, Rust version, etc.)
- Relevant logs or error messages

**Feature Requests:**
Use the feature request template and include:
- Clear description of the feature
- Use case and motivation
- Proposed implementation approach (if any)
- References to similar features in other terminals

**Questions:**
For questions about usage or development:
- Check existing documentation first
- Search closed issues
- Use GitHub Discussions if available
- Be specific and provide context

### Claiming Issues

1. Comment on the issue: "I'd like to work on this"
2. Wait for maintainer confirmation (to avoid duplicate work)
3. Ask questions if anything is unclear
4. Provide regular updates (weekly is good)

### Good First Issues

Look for issues labeled:
- `good first issue` - Great for first-time contributors
- `help wanted` - Maintainers are specifically looking for help
- `documentation` - Documentation improvements
- `testing` - Adding or improving tests

## Getting Help

### Resources

- **Documentation:** [./docs](./docs) and root-level `.md` files
- **Code Examples:** [./examples](./examples)
- **Architecture:** [ARCHITECTURE.md](./ARCHITECTURE.md)
- **Getting Started:** [GETTING_STARTED.md](./GETTING_STARTED.md)

### Communication

- **GitHub Issues:** For bugs and feature requests
- **GitHub Discussions:** For questions and general discussion (if enabled)
- **Pull Requests:** For code review and technical discussions

### Questions?

If you're stuck or have questions:
1. Check the documentation
2. Search existing issues
3. Open a new issue with the `question` label
4. Be patient and respectful - maintainers are volunteers

## Recognition

Contributors are recognized in:
- Git commit history
- CONTRIBUTORS.md file (TBD)
- GitHub's Contributors page
- Release notes for significant contributions

## License

By contributing to TermiEmu, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to TermiEmu! Together, we're building something great. 🚀
