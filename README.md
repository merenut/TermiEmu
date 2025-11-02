# TermiEmu

A modern, high-performance terminal emulator built in Rust with a focus on speed, beauty, and exceptional user experience.

## Design Philosophy

**"Fluid Minimalism with Ruthless Performance"**

TermiEmu combines cutting-edge Rust GUI technology with thoughtful UX design to deliver a terminal that's both powerful and pleasant to use.

## Documentation

- **[Design Summary](./DESIGN_SUMMARY.md)** - Quick overview of design decisions and architecture
- **[Full GUI/UX Design Document](./GUI_UX_DESIGN.md)** - Comprehensive design specification covering:
  - Modern terminal UX research (Warp, WezTerm, Ghostty, Alacritty)
  - Framework selection rationale (Iced vs. egui vs. Slint vs. Dioxus)
  - Visual design & theming system
  - UI chrome (tabs, splits, scrollbar, command palette)
  - Grid interaction & user experience
  - Implementation roadmap

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

🚧 **In Design Phase** - Currently developing comprehensive GUI/UX specifications. Implementation will begin once design is validated.

## Contributing

Design feedback and contributions are welcome! Please review the [Design Documents](./GUI_UX_DESIGN.md) before proposing changes.

## License

TBD