# TermiEmu: GUI/UX Design - Executive Summary

This document provides a high-level overview of the comprehensive GUI/UX design for TermiEmu. For full details, see [GUI_UX_DESIGN.md](./GUI_UX_DESIGN.md).

## Core Design Philosophy

**"Fluid Minimalism with Ruthless Performance"**

TermiEmu is designed to be:
- **Sleek & Beautiful:** Modern, flat design with subtle depth
- **Exceptionally Fast:** Sub-10ms input latency, 60fps+ rendering
- **Minimally Intrusive:** Maximum terminal space, minimal chrome
- **Highly Responsive:** Immediate visual feedback to all actions

## Framework Choice: Iced

After analyzing Iced, Dioxus, Slint, and egui, **Iced** was selected as the optimal framework because:

✅ **Elm-inspired retained mode** - Perfect for terminal's reactive nature  
✅ **GPU-accelerated rendering** - Built on wgpu for cross-platform performance  
✅ **Custom widget support** - Full control over text grid rendering  
✅ **Async-native** - First-class async/await for PTY I/O  
✅ **Type-safe** - Compile-time UI correctness guarantees  
✅ **MIT License** - Permissive for commercial use  

### Framework Comparison

| Feature | Iced ⭐ | egui | Slint | Dioxus |
|---------|--------|------|-------|---------|
| Grid Performance | ★★★★☆ | ★★★☆☆ | ★★★★★ | ★★☆☆☆ |
| Async Support | ★★★★★ | ★★★☆☆ | ★★★★☆ | ★★★★☆ |
| Custom Widgets | ★★★★☆ | ★★★★★ | ★★★★★ | ★★☆☆☆ |
| Learning Curve | Medium | Low | Medium-High | Medium |

## Key Design Decisions

### 1. Typography: cosmic-text
- **Ligature support** for modern programming fonts (Fira Code, JetBrains Mono)
- **Color emoji rendering** via swash
- **Robust font fallback** (browser-grade, multi-language support)
- **Complex scripts** (Arabic, Indic, CJK) with proper shaping

### 2. Theming System
- **TOML-based** configuration (human-readable, Rust-native)
- **Hot-reload** without restart
- **8 default themes** (Catppuccin, Tokyo Night, Dracula, Nord, etc.)
- **Platform-specific blur** (macOS vibrancy, Windows 11 Mica/Acrylic)

### 3. UI Chrome (Application Shell)

**Near-Headerless Design:**
```
┌───────────────────────────────────┐
│ [+] Tab 1  Tab 2  Tab 3    [⚙]   │ ← 28px tab bar
├───────────────────────────────────┤
│                                   │
│   Terminal Grid (full content)   │
│                                   │
│                                ║  │ ← Auto-hide scrollbar
└───────────────────────────────────┘
```

**Features:**
- **Tab bar** (28px, minimal, drag-to-reorder)
- **Split panes** (vim-style navigation, visual focus indicator)
- **Custom scrollbar** (8px, auto-hide, proportional thumb)
- **Command palette** (`Ctrl+Shift+P`, fuzzy search, 600px wide)

### 4. Grid Interaction (UX)

**Text Selection:**
- **Character mode** (default): Click and drag
- **Word mode**: Double-click (smart boundaries for URLs, paths, hashes)
- **Line mode**: Triple-click
- **Block mode**: Alt+drag (rectangular/column selection)

**Copy/Paste:**
- `Ctrl+Shift+C` / `Ctrl+Shift+V` (always)
- `Ctrl+C` (copy if text selected, else SIGINT)
- **Bracketed paste** for safety
- **Multi-line paste warning** (configurable)

**Mouse Features:**
- **Clickable hyperlinks** (underline on hover, Ctrl+Click to open)
- **OSC 8 support** (explicit hyperlinks via escape sequences)
- **Mouse reporting** (X10, VT200, SGR protocols for vim/tmux)
- **Smart scrolling** (normal vs. alternate screen detection)

## Visual Design Highlights

### Modern Aesthetics
- **Dark-first** with excellent light mode
- **Subtle transparency** (0.90-0.95 opacity)
- **Platform blur** (native blur/acrylic on macOS/Windows)
- **Micro-animations** (100-200ms, 60fps+, respect "reduce motion")
- **High contrast** typography (WCAG AA compliant)

### Animation Philosophy
- **Instant feedback** - Text rendering, selection, cursor (0ms delay)
- **Micro-animations** - Tab switch (100ms), theme fade (300ms), palette slide (200ms)
- **Smooth scrolling** - Inertial on trackpad, crisp on mouse wheel
- **NO decorative delays** - Every animation serves functional feedback

## Performance Targets

| Metric | Target | Strategy |
|--------|--------|----------|
| Input Latency | < 10ms | Direct PTY write, GPU acceleration |
| Frame Rate | 60fps (120fps capable) | Damage tracking, efficient redraws |
| Startup Time | < 200ms | Lazy loading, optimized binary |
| Memory (1 session) | < 100MB | Glyph caching, efficient buffers |
| Grid Render | < 2ms (full 1920x1080) | wgpu + damage tracking |

## Implementation Roadmap

### Phase 1: Core Rendering (Weeks 1-3)
- Iced application shell
- Custom grid widget (wgpu backend)
- cosmic-text integration
- Basic text rendering + colors

### Phase 2: Visual Polish (Weeks 4-6)
- Ligatures & emoji
- Theme engine + hot-reload
- Transparency/blur effects
- Tab bar + scrollbar

### Phase 3: Interaction (Weeks 7-9)
- Text selection (all modes)
- Copy/paste + bracketed paste
- Hyperlinks + OSC 8
- Mouse reporting

### Phase 4: Advanced UI (Weeks 10-12)
- Split panes
- Command palette
- Search in buffer
- Settings UI

### Phase 5: Optimization (Weeks 13-14)
- Damage tracking
- Glyph caching
- Benchmarking
- Accessibility

### Phase 6: Launch (Weeks 15-16)
- Testing + documentation
- Performance benchmarks
- Theme gallery

## Technical Stack

```toml
[dependencies]
iced = { version = "0.12", features = ["canvas", "wgpu"] }
cosmic-text = "0.10"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
tokio = { version = "1.0", features = ["full"] }
```

## Competitive Differentiation

### vs. Alacritty
✅ Rich UI (tabs, splits, command palette)  
✅ Extensible theming (hot-reload)  
✅ Modern interactions (clickable links, smart selection)  

### vs. WezTerm
✅ Simpler, more focused feature set  
✅ Native Rust GUI (vs. Lua scripting)  
✅ Faster startup (< 200ms vs. ~500ms)  

### vs. Warp
✅ Open source, free (vs. commercial)  
✅ Privacy-focused (no cloud/AI)  
✅ Lightweight (< 100MB vs. ~300MB)  

### vs. Ghostty
✅ Cross-platform (Linux/macOS/Windows)  
✅ Rich theming + customization  
✅ Modern UI chrome (tabs, command palette)  

## Accessibility

- **Screen reader support** via platform accessibility APIs
- **High contrast mode** for visual impairments
- **Color blindness themes** (protanopia, deuteranopia, tritanopia)
- **Keyboard-only navigation** for all features
- **Respect "reduce motion"** system setting

## Next Steps

1. **Prototype Phase 1** - Validate Iced + cosmic-text architecture
2. **User Testing** - Early feedback on selection, scrollbar, tab UX
3. **Performance Benchmarking** - Compare vs. Alacritty
4. **Theme Curation** - Gather community favorite themes

---

**Full Documentation:** [GUI_UX_DESIGN.md](./GUI_UX_DESIGN.md)  
**Project Repository:** [github.com/merenut/TermiEmu](https://github.com/merenut/TermiEmu)

*Last Updated: November 2025*
