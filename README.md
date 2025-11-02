# TermiEmu

[![CI](https://github.com/merenut/TermiEmu/actions/workflows/ci.yml/badge.svg)](https://github.com/merenut/TermiEmu/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A modern, high-performance terminal emulator built in Rust with a focus on speed, beauty, and exceptional user experience.

## Design Philosophy

**"Fluid Minimalism with Ruthless Performance"**

TermiEmu combines cutting-edge Rust GUI technology with thoughtful UX design to deliver a terminal that's both powerful and pleasant to use.

## Documentation

📚 **[Documentation Index](./DOCUMENTATION_INDEX.md)** - Complete guide to navigating all documentation

### Quick Links

- **[Production Roadmap](./PRODUCTION_ROADMAP.md)** - Complete roadmap to v1.0 (55+ user stories) 🗺️
- **[Quick Reference](./ROADMAP_QUICK_REFERENCE.md)** - TL;DR roadmap summary ⚡
- **[Design Summary](./DESIGN_SUMMARY.md)** - 10-minute overview of key decisions
- **[Full GUI/UX Design](./GUI_UX_DESIGN.md)** - Complete design specification (2,900+ lines)
- **[Architecture](./ARCHITECTURE.md)** - Technical architecture with visual diagrams
- **[Getting Started](./GETTING_STARTED.md)** - Developer guide with code examples

### What's Covered

The documentation provides a complete blueprint covering:
- Modern terminal UX research (Warp, WezTerm, Ghostty, Alacritty)
- Framework selection rationale (Iced vs. egui vs. Slint vs. Dioxus)
- Visual design & theming system (cosmic-text, TOML themes, hot-reload)
- UI chrome (tabs, splits, scrollbar, command palette)
- Grid interaction & user experience (selection, copy/paste, mouse, hyperlinks)
- System architecture and rendering pipeline
- 16-week implementation roadmap (6 phases)
- Complete code examples for getting started

## Key Features (Planned)

- ⚡ **Exceptional Performance:** Sub-10ms input latency, GPU-accelerated rendering
- 🎨 **Beautiful Theming:** Hot-reloadable themes with platform-specific blur effects
- 🔤 **Rich Typography:** Ligatures, emoji, and complex scripts via cosmic-text
- 🖱️ **Modern Interactions:** Smart text selection, clickable links, intuitive mouse support
- 📐 **Flexible Layout:** Tabs and split panes with vim-style navigation
- ⌨️ **Command Palette:** Fuzzy-search command launcher (`Ctrl+Shift+P`)
- 🌐 **Cross-Platform:** Linux, macOS, and Windows support

## Technology Stack

- **GUI Framework:** [Iced](https://iced.rs/) (Elm-inspired, async-native, GPU-accelerated)
- **Font Rendering:** [cosmic-text](https://github.com/pop-os/cosmic-text) (ligatures, emoji, fallback)
- **Language:** Rust 2021 (performance, safety, modern ergonomics)

## Status

🚧 **Phase 0 (Pre-Alpha) - Foundation** - Project structure initialized. Beginning implementation of core terminal emulator features.

## Building from Source

### Prerequisites

- Rust 1.70.0 or later (Install from [rustup.rs](https://rustup.rs/))

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/merenut/TermiEmu.git
cd TermiEmu

# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Build optimized release version
cargo build --release
```

### Development Commands

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Build documentation
cargo doc --open
```

## Contributing

Contributions are welcome! We value contributions from developers of all experience levels.

- **[Contributing Guide](./CONTRIBUTING.md)** - Development setup, workflow, and coding standards
- **[Code of Conduct](./CODE_OF_CONDUCT.md)** - Community guidelines and expectations

For design context, see:
- [Design Documents](./GUI_UX_DESIGN.md) and [Production Roadmap](./PRODUCTION_ROADMAP.md)
- [Getting Started](./GETTING_STARTED.md) and [Code Quality Standards](./CODE_QUALITY.md)

## License

MIT OR Apache-2.0
