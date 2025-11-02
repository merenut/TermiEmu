# Modern Terminal Emulator: GUI/UX Design Document

**Project:** TermiEmu  
**Design Philosophy:** Sleek, beautiful, modern, and exceptionally fast  
**Date:** November 2025  
**Version:** 1.0

---

## Table of Contents

1. [Research: Modern Terminal Emulator UX](#1-research-modern-terminal-emulator-ux)
2. [Core Philosophy & Framework Choice](#2-core-philosophy--framework-choice)
3. [Visual Design & Theming](#3-visual-design--theming)
4. [UI "Chrome" (Application Shell)](#4-ui-chrome-application-shell)
5. [Core Grid Interaction (User Experience)](#5-core-grid-interaction-user-experience)
6. [Implementation Roadmap](#6-implementation-roadmap)

---

## 1. Research: Modern Terminal Emulator UX

### 1.1 Key Innovators Analysis

Modern terminal emulators have redefined what users expect from their command-line interface. Here's what makes the current generation exceptional:

#### **Warp** - The IDE-Like Terminal
- **Block-Based UI:** Groups commands and outputs into distinct, navigable blocks—making terminal history scannable and organized
- **AI-Powered Features:** Smart completions for 400+ CLI tools, inline error highlighting, and workflow suggestions
- **Advanced Collaboration:** "Warp Drive" notebooks for sharing terminal sessions and workflows
- **Modern Editing:** Full mouse and cursor support with IDE-like text editing capabilities
- **Visual Polish:** Clean interfaces with customizable panels, command palettes, and split panes with synced inputs
- **Design Focus:** Minimal visual clutter with context-aware grouping and rapid feedback loops

#### **WezTerm** - The Power User's Choice
- **Extreme Customization:** Extensive configuration via Lua, programmable keybindings, and session management
- **GPU-Accelerated Rendering:** True color support with ligatures and inline images
- **Productivity Features:** Split panes, tabbed interfaces, clickable links, and searchable scrollback
- **Cross-Platform Consistency:** Unified experience across all platforms with flexible themes
- **Design Focus:** Clean, uncluttered interface that favors power-user features through configuration

#### **Ghostty** - The Minimalist Speedster
- **Extreme Performance:** Optimized for minimal startup time and maximum responsiveness
- **Simplicity First:** "Just works" out-of-the-box with minimal configuration
- **Lightweight:** Focuses on essential features without unnecessary integrations
- **Design Focus:** Clean lines, understated colors, and maximum focus on content

#### **Alacritty** - The Performance Benchmark
- **GPU Acceleration:** Exceptionally fast rendering using OpenGL
- **Minimal Core:** YAML configuration with zero GUI overlays
- **High Fidelity:** True color, ligature support, and high-DPI compatibility
- **Design Focus:** Stark, modern aesthetic with bold colored text and minimal distractions

### 1.2 Common Design Principles

Across these successful terminals, several patterns emerge:

1. **Performance-Driven Architecture**
   - GPU acceleration is non-negotiable for smooth rendering
   - Damage tracking to redraw only changed cells
   - Low-latency input handling with immediate visual feedback

2. **Modern Interaction Paradigms**
   - Full mouse support (clicking, selecting, scrolling)
   - Clickable hyperlinks with visual hover states
   - Intuitive text selection (word, line, and block-based)
   - Keyboard-first navigation with discoverable shortcuts

3. **Visual Minimalism**
   - Clean, distraction-free interfaces
   - Optional window chrome (headerless/borderless modes)
   - Subtle transparency and blur effects
   - High-contrast, legible typography

4. **Deep Customization**
   - Hot-reloadable themes and color schemes
   - Flexible font configuration with ligature support
   - Per-profile settings (different configs for different workflows)
   - Command palettes for quick access to features

5. **Organization & Navigation**
   - Intelligent tab management
   - Split pane support (horizontal and vertical)
   - Visual distinction between input and output (Warp's "blocks")
   - Searchable command history

### 1.3 Must-Have UX Features

Based on modern terminal analysis, these features are essential:

- ✅ **Pixel-perfect text selection** with smart word/line boundaries
- ✅ **Clickable hyperlinks** with underline-on-hover
- ✅ **True color support** with 24-bit RGB
- ✅ **Ligature rendering** for modern programming fonts
- ✅ **Copy/paste integration** with system clipboard
- ✅ **Mouse reporting** for applications like vim/tmux
- ✅ **Scrollback buffer** with fast searching
- ✅ **Split panes** with keyboard and mouse navigation
- ✅ **Tab management** with visual indicators
- ✅ **Command palette** for discoverability
- ✅ **Theme switching** without restart
- ✅ **Emoji and Unicode support** with fallback fonts

---

## 2. Core Philosophy & Framework Choice

### 2.1 Design Philosophy

**TermiEmu embodies: "Fluid Minimalism with Ruthless Performance"**

**Core Values:**
1. **Speed Above All:** Sub-millisecond input-to-pixel latency
2. **Visual Clarity:** Every element serves a purpose; nothing is decorative
3. **Spatial Economy:** Maximize terminal real estate; minimize chrome
4. **Tactile Responsiveness:** Immediate visual feedback to every user action
5. **Adaptive Beauty:** Interface adapts to user workflow without configuration burden

**Visual Identity:**
- **Aesthetic:** Modern, flat design with subtle depth cues (shadows, not gradients)
- **Color Philosophy:** Dark-first with excellent light mode support
- **Typography:** Programming-optimized fonts as first-class citizens
- **Animation:** Micro-animations for state changes (60fps+), never decorative delays
- **Transparency:** Subtle, functional opacity (never gimmicky)

### 2.2 Rust GUI Framework Recommendation: **Iced**

After careful analysis, **Iced** is the recommended framework for TermiEmu.

#### Why Iced?

**1. Architecture Match**
- **Elm-inspired Retained Mode:** Perfect for terminal's reactive nature
  - Terminal state changes trigger minimal, efficient redraws
  - Built-in async/message passing aligns with PTY event handling
  - Predictable update cycle: `Model → Message → Update → View`
- **Custom Widget Support:** Essential for the text grid
  - `Canvas` widget provides low-level drawing control
  - Custom widget trait allows complete control over grid rendering
  - Native integration with `wgpu` for GPU-accelerated rendering

**2. Performance Profile**
- **Selective Redraws:** Only changed UI regions are rerendered
- **GPU Acceleration:** Built on `wgpu` for modern, cross-platform graphics
- **Efficient State Management:** Functional update model prevents unnecessary work
- **Async-Native:** First-class `async`/`await` support for PTY I/O

**3. Developer Experience**
- **Type Safety:** Compile-time guarantees for UI correctness
- **Clear Patterns:** Message-driven architecture reduces complexity
- **Active Community:** Well-maintained with responsive development
- **Documentation:** Excellent examples and guides

### 2.3 Framework Trade-Off Analysis

#### Iced vs. Alternatives

| Aspect | **Iced (Recommended)** | egui | Slint | Dioxus |
|--------|------------------------|------|-------|---------|
| **Rendering Model** | Retained (Elm) | Immediate | Declarative (QML-like) | Virtual DOM (React-like) |
| **Custom Grid Performance** | ★★★★☆ | ★★★☆☆ | ★★★★★ | ★★☆☆☆ |
| **Async/Multithreading** | ★★★★★ | ★★★☆☆ | ★★★★☆ | ★★★★☆ |
| **State Management** | ★★★★★ | ★★☆☆☆ | ★★★★☆ | ★★★★☆ |
| **Learning Curve** | Medium | Low | Medium-High | Medium |
| **Custom Widget Control** | ★★★★☆ | ★★★★★ | ★★★★★ | ★★☆☆☆ |
| **Desktop Native Feel** | ★★★★☆ | ★★★☆☆ | ★★★★★ | ★★★☆☆ |
| **Cross-Platform** | ★★★★★ | ★★★★★ | ★★★★☆ | ★★★★★ |
| **License** | MIT | MIT/Apache-2.0 | GPL/Commercial | MIT/Apache-2.0 |

#### Detailed Comparison

**egui (Immediate Mode):**
- ✅ **Pros:** Extremely fast prototyping, tight rendering loop control, simple mental model
- ❌ **Cons:** CPU overhead on large UIs, manual optimization needed for grid, no declarative layout
- **Verdict:** Excellent for overlays/debug tools, but retained mode is better for terminal's reactive nature

**Slint (Declarative QML-Style):**
- ✅ **Pros:** Best embedded performance, pixel-perfect control, excellent tooling (LSP, live preview)
- ❌ **Cons:** Dual license (GPL + commercial), markup language learning curve, smaller ecosystem
- **Verdict:** Top choice if targeting embedded or needing maximum grid optimization, but licensing may restrict usage

**Dioxus (React-Style Virtual DOM):**
- ✅ **Pros:** Familiar for web developers, full-stack capabilities, good for standard UIs
- ❌ **Cons:** Virtual DOM overhead, less control over low-level rendering, not optimized for grid performance
- **Verdict:** Better for web/mobile hybrid apps; overkill for terminal with less performance control

#### Why Iced Wins for TermiEmu

1. **Sweet Spot Performance:** Retained mode offers excellent grid performance without egui's manual optimization burden
2. **Async-First:** Terminal emulators are inherently async (PTY I/O, rendering); Iced's architecture matches perfectly
3. **Custom Widget Freedom:** `Canvas` + custom widgets give full control over grid rendering while leveraging framework benefits
4. **Permissive License:** MIT license allows commercial use without restrictions
5. **Growing Ecosystem:** Active development, excellent documentation, responsive community

### 2.4 Architectural Integration

**TermiEmu + Iced Architecture:**

```
┌─────────────────────────────────────────────┐
│           Iced Application                  │
│  ┌─────────────────────────────────────┐   │
│  │  UI Layer (Iced Widgets)            │   │
│  │  • Tabs, Command Palette, Settings  │   │
│  │  • Scrollbar, Search Bar            │   │
│  └──────────────┬──────────────────────┘   │
│                 │ Messages                  │
│  ┌──────────────▼──────────────────────┐   │
│  │  Application State (Model)          │   │
│  │  • Terminal sessions                │   │
│  │  • Theme configuration              │   │
│  │  • User preferences                 │   │
│  └──────────────┬──────────────────────┘   │
│                 │ Commands                  │
│  ┌──────────────▼──────────────────────┐   │
│  │  Custom Grid Widget (Canvas)        │   │
│  │  • wgpu-accelerated rendering       │   │
│  │  • Damage tracking                  │   │
│  │  • Text shaping (cosmic-text)       │   │
│  └──────────────┬──────────────────────┘   │
└─────────────────┼───────────────────────────┘
                  │
   ┌──────────────▼──────────────────────┐
   │  Terminal Core (Async)              │
   │  • PTY management                   │
   │  • VTE/ANSI parser                  │
   │  • Grid buffer (cells, attributes)  │
   │  • Selection state                  │
   └─────────────────────────────────────┘
```

**Message Flow Example:**
```rust
// User types 'a'
KeyPressed('a') → Terminal::input('a') → PTY::write('a')
                                       ↓
                        Echo from PTY → Parser::parse()
                                       ↓
                        Grid updated → Iced::request_redraw()
                                       ↓
                        Canvas::draw() → GPU renders changed cells
```

---

## 3. Visual Design & Theming

### 3.1 Typography & Font Rendering

#### Font Stack Recommendation: **cosmic-text**

**Why cosmic-text:**
- Pure Rust, well-maintained (by System76/Pop!_OS team)
- Excellent OpenType feature support (ligatures, contextual alternates)
- Color emoji rendering via `swash`
- Robust font fallback (browser-grade, inspired by Chromium/Firefox)
- Bidirectional text support (RTL languages)
- Multi-line text and advanced shaping built-in

#### Font Configuration Design

```rust
pub struct FontConfig {
    // Primary monospace font for terminal
    primary: FontFamily,           // e.g., "JetBrains Mono", "Fira Code"
    
    // Fallback chain
    fallbacks: Vec<FontFamily>,    // e.g., ["Noto Sans Mono", "DejaVu Sans Mono"]
    
    // Emoji/symbol fonts
    emoji: FontFamily,             // e.g., "Noto Color Emoji"
    symbols: FontFamily,           // e.g., "Symbols Nerd Font"
    
    // Rendering options
    size: f32,                     // Base font size (e.g., 14.0)
    line_height: f32,              // Line height multiplier (e.g., 1.2)
    enable_ligatures: bool,        // Programming ligatures (->>=, ===)
    hinting: HintingMode,          // None, Slight, Medium, Full
    antialiasing: AAMode,          // None, Grayscale, Subpixel
}
```

#### Ligature Support

**Implementation Strategy:**
- cosmic-text handles ligature shaping automatically via OpenType features
- Expose user toggle: `enable_ligatures: bool` in config
- Popular ligature fonts to test:
  - Fira Code
  - JetBrains Mono
  - Cascadia Code
  - Iosevka

**Rendering Pipeline:**
```
Text Input → cosmic-text (shaping) → Glyph positions → Cache glyphs
                                                      ↓
                                          GPU texture atlas → Render
```

#### Emoji & Complex Script Handling

**Emoji Strategy:**
1. **Color Emoji:** cosmic-text + swash render color emoji natively
2. **Fallback Chain:** 
   ```
   Primary Font → Emoji Font (Noto Color Emoji) → Symbol Font → System Fallback
   ```
3. **Width Handling:** Track double-width characters (CJK, emoji) for proper cursor positioning

**Complex Scripts:**
- cosmic-text handles Indic, Arabic, Thai scripts with proper shaping
- Bidirectional text support for RTL languages
- Combining characters (diacritics) handled automatically

#### Font Rendering Quality

**Performance Optimizations:**
- **Glyph Caching:** Pre-render common glyphs to GPU texture atlas
- **Damage Tracking:** Only re-shape/render changed text regions
- **Subpixel Positioning:** For crisp rendering at any zoom level

**Quality Settings:**
```rust
pub enum RenderQuality {
    Performance,  // Fast, grayscale AA, minimal hinting
    Balanced,     // Subpixel AA, slight hinting (default)
    Quality,      // Full hinting, subpixel AA, extra ligature features
}
```

### 3.2 Color & Theming Engine

#### Theme Architecture

**Theme Structure:**
```rust
pub struct Theme {
    metadata: ThemeMetadata,
    colors: ColorScheme,
    ui: UiColors,
    effects: VisualEffects,
}

pub struct ThemeMetadata {
    name: String,
    author: String,
    variant: ThemeVariant,  // Dark, Light, Auto
}

pub struct ColorScheme {
    // Standard ANSI colors (16 colors)
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
    // Bright variants
    bright_black: Color,    // (gray)
    bright_red: Color,
    // ... (8 more)
    
    // Extended 256-color palette (optional override)
    palette_256: Option<[Color; 256]>,
    
    // True color support (24-bit RGB)
    supports_true_color: bool,
    
    // Terminal background/foreground
    background: Color,
    foreground: Color,
    cursor: CursorColors,
    selection: Color,       // Selection highlight
}

pub struct UiColors {
    // Window chrome
    tab_bar_bg: Color,
    tab_active: Color,
    tab_inactive: Color,
    scrollbar: Color,
    scrollbar_thumb: Color,
    
    // Command palette
    palette_bg: Color,
    palette_border: Color,
    palette_selection: Color,
    
    // Borders/separators
    split_divider: Color,
}

pub struct VisualEffects {
    window_opacity: f32,          // 0.0 (transparent) to 1.0 (opaque)
    blur_enabled: bool,           // Platform-specific blur
    blur_radius: f32,             // Blur amount (if supported)
}
```

#### Theme Loading & Hot-Reload

**Format Support:**
- **Primary:** TOML (Rust-native, human-readable)
- **Import:** Support popular formats (iTerm2, Alacritty YAML, VSCode JSON)

**TOML Theme Example:**
```toml
[metadata]
name = "Catppuccin Mocha"
author = "Catppuccin"
variant = "dark"

[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"
cursor_fg = "#1e1e2e"
cursor_bg = "#f5e0dc"
selection = "#585b7088"  # with alpha

# ANSI colors
black = "#45475a"
red = "#f38ba8"
green = "#a6e3a1"
yellow = "#f9e2af"
blue = "#89b4fa"
magenta = "#f5c2e7"
cyan = "#94e2d5"
white = "#bac2de"

[ui]
tab_bar_bg = "#181825"
tab_active = "#89b4fa"
tab_inactive = "#6c7086"

[effects]
window_opacity = 0.95
blur_enabled = true
blur_radius = 20.0
```

**Hot-Reload Implementation:**
```rust
// Watch theme directory for changes
pub struct ThemeManager {
    current: Theme,
    available: HashMap<String, PathBuf>,
    watcher: FileWatcher,
}

impl ThemeManager {
    pub fn load_theme(&mut self, name: &str) -> Result<Theme> {
        let path = self.available.get(name)?;
        let theme = Theme::from_toml(path)?;
        self.validate_theme(&theme)?;
        Ok(theme)
    }
    
    pub fn reload_current(&mut self) -> Result<()> {
        // Hot-reload without restart
        let theme = self.load_theme(&self.current.metadata.name)?;
        self.current = theme;
        Ok(())
    }
}
```

#### Popular Theme Pre-sets

**Ship with high-quality defaults:**
1. **Catppuccin** (Mocha, Latte, Frappé, Macchiato)
2. **Tokyo Night** (Night, Storm, Day)
3. **Dracula**
4. **Nord**
5. **Solarized** (Dark, Light)
6. **Gruvbox** (Dark, Light)
7. **One Dark** (Atom-inspired)
8. **GitHub** (Dark, Light)

**Theme Discovery:**
- Default themes bundled in binary
- User themes in `~/.config/termiemu/themes/`
- Command palette: `Ctrl+P` → "Change Theme" → fuzzy search

### 3.3 Visual Effects

#### Window Transparency

**Implementation Strategy:**
```rust
pub struct WindowConfig {
    opacity: f32,              // 0.0 - 1.0
    transparency_mode: TransparencyMode,
}

pub enum TransparencyMode {
    None,                      // Fully opaque
    Simple,                    // Simple alpha blending
    Blur,                      // Platform blur (macOS, Windows 11)
    Acrylic,                   // Windows 10+ acrylic material
}
```

**Platform-Specific Blur:**

**macOS (NSVisualEffectView):**
```rust
#[cfg(target_os = "macos")]
fn enable_vibrancy(window: &Window, material: NSVisualEffectMaterial) {
    // Use cocoa bindings to enable native blur
    unsafe {
        let ns_view = window.ns_view();
        let effect_view = NSVisualEffectView::new();
        effect_view.setMaterial(material);
        effect_view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
    }
}
```

**Windows 11 (Mica/Acrylic):**
```rust
#[cfg(target_os = "windows")]
fn enable_backdrop(window: &Window, backdrop: DWM_SYSTEMBACKDROP_TYPE) {
    use windows::Win32::Graphics::Dwm::*;
    unsafe {
        DwmSetWindowAttribute(
            window.hwnd(),
            DWMWA_SYSTEMBACKDROP_TYPE,
            &backdrop as *const _ as *const _,
            std::mem::size_of::<DWM_SYSTEMBACKDROP_TYPE>(),
        );
    }
}
```

**Linux (Compositor-Dependent):**
- X11: Set `_NET_WM_WINDOW_OPACITY` property
- Wayland: Compositor-specific (limited support)

**Performance Considerations:**
- Blur is GPU-intensive; offer quality settings
- Disable blur on low-end hardware automatically
- Cache blurred background when window is static

#### Animation & Motion

**Guiding Principles:**
- Animations should feel instant, not delay interaction
- 60fps minimum (prefer 120fps on high-refresh displays)
- Respect system "reduce motion" accessibility settings

**Animated Elements:**
```rust
pub struct AnimationConfig {
    duration_ms: u32,          // Base duration (default: 150ms)
    easing: EasingFunction,    // Ease-out-cubic (default)
    
    // Per-element overrides
    tab_switch: Duration,      // Tab switching (100ms)
    theme_transition: Duration,// Theme fade (300ms)
    command_palette: Duration, // Palette slide-in (200ms)
}
```

**Micro-Animations:**
- **Tab Switch:** Smooth slide + fade (100ms)
- **Theme Change:** Cross-fade colors (300ms, imperceptible)
- **Command Palette:** Slide from top with ease-out (200ms)
- **Scrollbar:** Fade in on mouse hover (150ms)
- **Link Hover:** Underline draw-in (80ms)

**NO Animations:**
- Text rendering (instant)
- Cursor movement (instant)
- Scrolling (instant, but smooth with inertia on trackpad)
- Text selection (instant visual feedback)

---

## 4. UI "Chrome" (Application Shell)

### 4.1 Window Layout Design

#### Minimal Chrome Philosophy

**Default Mode: "Near-Headerless"**
```
┌───────────────────────────────────────┐
│ [+] Tab 1  Tab 2  Tab 3       [⚙]    │ ← Minimal tab bar (28px)
├───────────────────────────────────────┤
│                                       │
│      Terminal Grid (main content)    │
│                                       │
│                                    ║  │ ← Scrollbar (auto-hide)
└───────────────────────────────────────┘
  ↑ No bottom status bar
```

**Truly Headerless Mode:**
```
┌───────────────────────────────────────┐
│                                       │
│      Terminal Grid (full bleed)      │
│                                       │
│                                       │
└───────────────────────────────────────┘
```
- Activate: `Ctrl+Shift+F` or auto-enter on fullscreen
- Access tabs: `Cmd/Ctrl+T` command palette overlay
- Exit: Hover top edge to reveal tab bar

#### Window Decorations

**Platform-Specific Approach:**

**macOS:**
- Use native window controls (traffic lights)
- Position traffic lights over tab bar (translucent region)
- Drag-to-move: entire tab bar is drag region

**Windows:**
- Custom title bar with native snap/minimize/maximize/close
- Drag region: left 60% of tab bar
- System menu: right-click tab bar

**Linux:**
- Respect window manager decorations (X11)
- Client-side decorations (Wayland) similar to Windows

#### Split Panes

**Layout System:**
```rust
pub enum PaneLayout {
    Single(TerminalId),
    Horizontal {
        top: Box<PaneLayout>,
        bottom: Box<PaneLayout>,
        split_ratio: f32,  // 0.0 - 1.0
    },
    Vertical {
        left: Box<PaneLayout>,
        right: Box<PaneLayout>,
        split_ratio: f32,
    },
}
```

**Visual Design:**
- **Divider:** 1px line (theme.ui.split_divider)
- **Resize Handle:** 5px hot area (cursor changes to resize icon)
- **Focus Indicator:** Subtle border (2px, accent color) around active pane
- **Keyboard Nav:** `Ctrl+W, h/j/k/l` (vim-style)

**Example Split Layout:**
```
┌─────────────────────────────────────┐
│ Tab 1                               │
├──────────────────┬──────────────────┤
│                  │                  │
│   Pane 1         │   Pane 2         │
│   (active)       │                  │
│                  │                  │
├──────────────────┴──────────────────┤
│            Pane 3                   │
│                                     │
└─────────────────────────────────────┘
```

### 4.2 Tab Management

#### Tab Bar Design

**Layout:**
```
[+] [Tab Name 1 ●] [Tab Name 2] [Tab Name 3] ... [⚙] [🔍]
 ↑        ↑            ↑                         ↑    ↑
New    Active      Inactive                  Settings Search
      (has changes)
```

**Tab Appearance:**
- **Height:** 28px (compact)
- **Width:** Dynamic (min 120px, max 200px)
- **Active Tab:** Full opacity, accent color underline (2px)
- **Inactive Tab:** 70% opacity, hover → 85% opacity
- **Close Button:** Appears on hover (× icon, right side)
- **Modified Indicator:** White dot (●) before title if unsaved work

**Tab Interactions:**
- **New Tab:** Click `[+]` or `Ctrl+T`
- **Close Tab:** Middle-click or click close button or `Ctrl+W`
- **Reorder:** Drag and drop tabs
- **Switch:** `Ctrl+Tab` (next), `Ctrl+Shift+Tab` (previous)
- **Jump:** `Ctrl+[1-9]` for tabs 1-9

**Tab Context Menu (right-click):**
```
Rename Tab
Duplicate Tab
Close Tab
Close Other Tabs
Close Tabs to the Right
─────────────
New Tab
New Window
```

#### Tab Overflow Handling

When tabs exceed window width:
```
[+] [Tab 1] [Tab 2] [Tab 3] [...] [⚙]
                               ↑
                          Overflow menu
```
- Overflow menu shows remaining tabs
- Active tab always visible
- Scroll horizontally with mouse wheel over tab bar

### 4.3 Scrollbar Design

#### Custom Scrollbar

**Philosophy:** Invisible until needed, minimal when visible

**Visual Design:**
```
Standard State (hidden):     Hover State:         Dragging:
                            ┃                     ┃
Terminal Grid               ┃ ░                   ┃ ▓
                            ┃ ░                   ┃ ▓
                            ┃ ▓                   ┃ ▓
                            ┃ ░                   ┃ ▓
                            ┃                     ┃
    (no scrollbar)          ↑ 8px wide           ↑ 10px wide
                           Track + Thumb          Thumb solid
```

**Behavior:**
- **Auto-Hide:** Fade out after 1s of no scroll activity
- **Width:** 8px (hover), 10px (dragging)
- **Position:** Right edge, 2px from edge
- **Thumb:** Proportional to visible content (min 20px height)
- **Track:** Subtle (barely visible)
- **Thumb Color:** 40% opacity when hovering, 60% when dragging

**Hide Conditions:**
1. No scrollback buffer (at prompt)
2. Alternate screen mode (vim, less, etc.)
3. Content fits in viewport

**Implementation:**
```rust
pub struct Scrollbar {
    visible: bool,
    thumb_position: f32,    // 0.0 - 1.0
    thumb_size: f32,        // 0.0 - 1.0 (proportion of track)
    dragging: bool,
    hover: bool,
    fade_timer: Timer,
}

impl Scrollbar {
    pub fn should_show(&self, grid: &Grid) -> bool {
        grid.scrollback_len() > 0 && !grid.in_alt_screen()
    }
    
    pub fn update_from_grid(&mut self, grid: &Grid, viewport_height: usize) {
        let total = grid.total_lines();
        self.thumb_size = (viewport_height as f32) / (total as f32);
        self.thumb_position = grid.scroll_offset() as f32 / total as f32;
    }
}
```

**Scrolling Methods:**
- **Mouse Wheel:** Smooth scroll (3 lines per notch)
- **Trackpad:** Inertial scrolling (native feel)
- **Drag Thumb:** Direct manipulation
- **Click Track:** Jump to position
- **Keyboard:** `PgUp`/`PgDn`, `Shift+Arrow`, `Ctrl+Home`/`End`

### 4.4 Command Palette

#### Design

**Activation:** `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)

**Visual Layout:**
```
┌─────────────────────────────────────┐
│ ┌─────────────────────────────────┐ │
│ │ >_ Type command...              │ │ ← Search input
│ └─────────────────────────────────┘ │
│ ┌─────────────────────────────────┐ │
│ │ ⚙  Change Theme                 │ │ ← Selected
│ │ 📋 Copy Mode                    │ │
│ │ 🔍 Search in Terminal           │ │
│ │ 🎨 Open Settings                │ │
│ │ 📂 Change Directory             │ │
│ │ ⌨  Show Keyboard Shortcuts      │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Behavior:**
- **Position:** Centered, 600px wide, max 400px tall
- **Background:** Blur/translucent (theme.ui.palette_bg)
- **Border:** Subtle (1px, theme.ui.palette_border)
- **Shadow:** Soft drop shadow for depth
- **Animation:** Slide down from top (200ms ease-out)
- **Dismiss:** `Esc`, click outside, or execute command

**Fuzzy Search:**
```rust
pub struct CommandPalette {
    query: String,
    commands: Vec<Command>,
    filtered: Vec<(Command, Score)>,  // Fuzzy-matched results
    selected: usize,
}

pub struct Command {
    name: String,
    description: String,
    icon: &'static str,      // Emoji or icon
    action: CommandAction,
    keywords: Vec<String>,   // Additional search terms
}
```

**Command Categories:**
1. **Theme:** Change theme, edit current theme
2. **Settings:** Open settings, reload config
3. **Navigation:** Switch tab, new tab, close tab, split pane
4. **Edit:** Copy mode, paste, select all
5. **Search:** Search in buffer, search history
6. **View:** Toggle tab bar, toggle scrollbar, zoom in/out
7. **Help:** Show shortcuts, open documentation

**Smart Ordering:**
- Frequently used commands ranked higher
- Recently used commands appear first
- Context-aware (e.g., "Close Pane" only if panes exist)

**Keyboard Navigation:**
- `↑`/`↓` or `Ctrl+P`/`Ctrl+N`: Navigate results
- `Enter`: Execute selected command
- `Esc`: Close palette
- Type to filter in real-time

---

## 5. Core Grid Interaction (User Experience)

### 5.1 Text Selection

#### Selection Modes

**Three Selection Types:**

1. **Character Selection** (default)
   - Click and drag to select individual characters
   - Smart word boundaries (double-click extends to word)

2. **Line Selection**
   - Triple-click: Select entire line
   - `Shift+Click`: Extend selection to line

3. **Block Selection**
   - `Alt+Drag`: Rectangular selection (column mode)
   - Useful for selecting columns in formatted output

#### Selection Behavior

**Visual Design:**
```rust
pub struct SelectionStyle {
    background: Color,       // theme.colors.selection
    foreground: Option<Color>, // Invert colors or None (keep text colors)
    opacity: f32,            // 0.3 - 0.5 (semi-transparent)
}
```

**Smart Selection Rules:**
- **Word Boundaries:** Detect words by whitespace, punctuation
- **URL Detection:** Double-click URL selects entire URL
- **File Paths:** Double-click path selects entire path
- **IP Addresses:** Double-click IP selects full address
- **Git Hashes:** Double-click hash selects full hash

**Implementation:**
```rust
pub struct Selection {
    start: GridPoint,
    end: GridPoint,
    mode: SelectionMode,
    semantic_type: Option<SemanticType>,  // URL, Path, Word, etc.
}

pub enum SelectionMode {
    Character,
    Word,
    Line,
    Block,   // Rectangular
}

impl Selection {
    pub fn expand_to_word(&mut self, grid: &Grid) {
        let start_word = grid.find_word_boundary(self.start, Direction::Backward);
        let end_word = grid.find_word_boundary(self.end, Direction::Forward);
        self.start = start_word;
        self.end = end_word;
        self.mode = SelectionMode::Word;
    }
    
    pub fn to_string(&self, grid: &Grid) -> String {
        match self.mode {
            SelectionMode::Block => grid.extract_block(self.start, self.end),
            _ => grid.extract_range(self.start, self.end),
        }
    }
}
```

**Selection Interactions:**
- **Single Click:** Clear selection, set cursor
- **Double Click:** Select word under cursor
- **Triple Click:** Select line under cursor
- **Click + Drag:** Character selection
- **Double Click + Drag:** Word-wise selection
- **Triple Click + Drag:** Line-wise selection
- **Alt + Drag:** Block selection
- **Shift + Click:** Extend selection to click point

### 5.2 Copy & Paste

#### Copy Behavior

**Copy Modes:**
```rust
pub enum CopyFormat {
    PlainText,          // Strip all formatting
    WithColors,         // ANSI escape codes preserved
    HTML,               // Rich text with colors
    Markdown,           // Code-fenced markdown
}
```

**Copy Triggers:**
- `Ctrl+C` (when text selected, otherwise sends SIGINT)
- `Ctrl+Shift+C` (always copy, never interrupt)
- Right-click → "Copy" in context menu
- Auto-copy on selection (optional setting)

**Copy Processing:**
```rust
pub struct CopyProcessor {
    format: CopyFormat,
    trim_whitespace: bool,    // Remove trailing spaces
    strip_prompts: bool,      // Remove PS1 prompts (smart detection)
}

impl CopyProcessor {
    pub fn process(&self, text: String, grid: &Grid) -> String {
        let mut result = text;
        
        if self.trim_whitespace {
            result = self.trim_trailing_spaces(&result);
        }
        
        if self.strip_prompts {
            result = self.detect_and_strip_prompt(&result, grid);
        }
        
        match self.format {
            CopyFormat::PlainText => result,
            CopyFormat::WithColors => self.preserve_ansi(&result, grid),
            CopyFormat::HTML => self.to_html(&result, grid),
            CopyFormat::Markdown => self.to_markdown(&result),
        }
    }
}
```

#### Paste Behavior

**Paste Modes:**
1. **Default:** `Ctrl+V` or `Ctrl+Shift+V`
2. **Bracketed Paste:** Wrap pasted content in `\e[200~...\e[201~`
   - Prevents accidental command execution
   - Most modern shells support this

**Paste Processing:**
- **Newline Handling:** Warn on multi-line paste (optional)
- **Smart Quote Conversion:** Convert curly quotes to straight quotes
- **Whitespace Normalization:** Convert tabs to spaces (configurable)

**Safety Feature:**
```rust
pub struct PasteGuard {
    warn_on_multiline: bool,
    max_lines: usize,        // Default: 10 (warn above)
}

impl PasteGuard {
    pub fn should_warn(&self, text: &str) -> Option<String> {
        let line_count = text.lines().count();
        if self.warn_on_multiline && line_count > self.max_lines {
            Some(format!(
                "About to paste {} lines. Continue?",
                line_count
            ))
        } else {
            None
        }
    }
}
```

### 5.3 Mouse Interaction

#### Hyperlink Detection & Clicking

**URL Pattern Recognition:**
```rust
pub struct HyperlinkDetector {
    patterns: Vec<Regex>,
}

impl Default for HyperlinkDetector {
    fn default() -> Self {
        Self {
            patterns: vec![
                // HTTP(S) URLs
                Regex::new(r"https?://[^\s]+").unwrap(),
                // File paths
                Regex::new(r"(?:~|\.|/)?[\w/\.-]+").unwrap(),
                // Git URLs
                Regex::new(r"git@[\w\.-]+:[\w/\.-]+\.git").unwrap(),
            ],
        }
    }
}
```

**Visual Feedback:**
```
Normal text: The quick brown fox
Hover URL:   https://example.com
             ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
             ↑ Underline on hover
             ↑ Cursor changes to pointer
```

**Click Action:**
- **`Ctrl+Click` or `Cmd+Click`:** Open URL in default browser
- **Hover:** Show tooltip with full URL (if truncated)
- **Right-Click:** Context menu
  ```
  Open Link
  Copy Link Address
  ───────────────
  Copy
  Paste
  ```

**OSC 8 Hyperlink Support:**
```rust
// Terminal escape sequences for explicit hyperlinks
// Format: \e]8;;URL\e\\TEXT\e]8;;\e\\

pub struct OscHyperlink {
    url: String,
    text: String,
    start_col: usize,
    end_col: usize,
}
```

#### Mouse Reporting

**Protocol Support:**
- **X10:** Basic click reporting
- **VT200:** Mouse tracking (press, release, drag)
- **VT200 Highlight:** Click + drag tracking
- **BTN Event:** Button event tracking
- **SGR:** Modern mouse protocol (1006)
- **URXVT:** Alternative encoding (1015)

**Implementation:**
```rust
pub struct MouseReporting {
    mode: Option<MouseMode>,
    encoding: MouseEncoding,
}

pub enum MouseMode {
    X10,           // 9
    VT200,         // 1000
    VT200Highlight,// 1001
    BtnEvent,      // 1002
    AnyEvent,      // 1003
}

pub enum MouseEncoding {
    Default,       // Legacy
    UTF8,          // 1005
    SGR,           // 1006 (best)
    URXVT,         // 1015
}

impl MouseReporting {
    pub fn encode_event(&self, event: MouseEvent) -> Option<Vec<u8>> {
        if self.mode.is_none() {
            return None; // Mouse reporting disabled, handle locally
        }
        
        Some(match self.encoding {
            MouseEncoding::SGR => self.encode_sgr(event),
            MouseEncoding::Default => self.encode_legacy(event),
            // ... other encodings
        })
    }
    
    pub fn should_report(&self, event: &MouseEvent) -> bool {
        match self.mode {
            Some(MouseMode::X10) => matches!(event, MouseEvent::Press(_)),
            Some(MouseMode::VT200) => true,
            Some(MouseMode::AnyEvent) => true,
            None => false,
        }
    }
}
```

**User Experience:**
- **Auto-Detection:** Terminal app (vim, tmux) enables mouse reporting
- **Seamless Switch:** 
  - Mouse reporting ON → clicks sent to app
  - Mouse reporting OFF → clicks handled by terminal (selection, links)
- **Override:** `Shift+Click` always handled by terminal (bypass app)

**Visual Indicator:**
```
Mouse Reporting OFF:     Mouse Reporting ON:
(Terminal handles)       (App handles)

[Normal cursor]          [Crosshair cursor]
```

#### Scrolling Behavior

**Scroll Events:**
```rust
pub enum ScrollMode {
    Normal,              // Scrollback buffer
    AlternateScreen,     // Send to application
    MouseReporting,      // Send scroll events as mouse reports
}

impl ScrollMode {
    pub fn determine(grid: &Grid, mouse_reporting: &MouseReporting) -> Self {
        if grid.in_alt_screen() {
            if mouse_reporting.mode.is_some() {
                ScrollMode::MouseReporting
            } else {
                ScrollMode::AlternateScreen  // Send arrow keys
            }
        } else {
            ScrollMode::Normal
        }
    }
}
```

**Scroll Behavior:**
- **Normal Mode:** Scroll through history (smooth, inertial on trackpad)
- **Alternate Screen Mode:** 
  - With mouse reporting: Send scroll events to app
  - Without mouse reporting: Send `↑`/`↓` arrow keys
- **Shift+Scroll:** Always scroll history (override app)

### 5.4 Keyboard Shortcuts

#### Essential Shortcuts

**Terminal Core:**
```
Ctrl+Shift+C       Copy selection
Ctrl+Shift+V       Paste
Ctrl+Shift+F       Find/Search in buffer
Ctrl+Shift+N       New window
Ctrl+Shift+T       New tab
Ctrl+Shift+W       Close tab
Ctrl+Tab           Next tab
Ctrl+Shift+Tab     Previous tab
Ctrl+[1-9]         Jump to tab 1-9
```

**Panes:**
```
Ctrl+Shift+D       Split vertically
Ctrl+Shift+Shift+D Split horizontally
Ctrl+W, H/J/K/L    Navigate panes (vim-style)
Ctrl+W, Q          Close current pane
```

**View:**
```
Ctrl+Plus          Zoom in (increase font size)
Ctrl+Minus         Zoom out (decrease font size)
Ctrl+0             Reset zoom
Ctrl+Shift+P       Command palette
F11                Fullscreen
```

**Selection:**
```
Shift+Arrow        Extend selection
Ctrl+Shift+Home    Select to top of buffer
Ctrl+Shift+End     Select to bottom of buffer
Ctrl+A             Select all (if no line editing active)
```

**Scroll:**
```
Shift+PgUp/PgDn    Scroll by page
Shift+Up/Down      Scroll by line
Ctrl+Home/End      Jump to top/bottom of buffer
```

**Customization:**
```rust
pub struct KeyBindings {
    mappings: HashMap<KeyCombo, Action>,
    mode: KeyBindingMode,
}

pub enum KeyBindingMode {
    Default,
    Vim,       // Vim-style navigation
    Emacs,     // Emacs-style bindings
    Custom,    // User-defined
}

pub struct KeyCombo {
    key: Key,
    modifiers: Modifiers,  // Ctrl, Shift, Alt, Super
}

pub enum Action {
    Copy,
    Paste,
    NewTab,
    CloseTab,
    SplitVertical,
    // ... all actions
}
```

---

## 6. Implementation Roadmap

### Phase 1: Core Rendering (Weeks 1-3)
- [ ] Set up Iced application shell
- [ ] Implement custom grid widget with `wgpu` backend
- [ ] Integrate cosmic-text for font rendering
- [ ] Basic text rendering pipeline (monospace, no ligatures)
- [ ] Color scheme support (ANSI 16 colors)
- [ ] Cursor rendering and positioning
- [ ] Input handling (keyboard to PTY)

### Phase 2: Visual Polish (Weeks 4-6)
- [ ] Ligature support via cosmic-text
- [ ] Emoji rendering and font fallback
- [ ] Theme engine (TOML loading)
- [ ] Hot-reload themes
- [ ] Window transparency and blur effects (platform-specific)
- [ ] Tab bar implementation
- [ ] Basic scrollbar

### Phase 3: Interaction (Weeks 7-9)
- [ ] Text selection (character, word, line, block modes)
- [ ] Copy/paste with bracketed paste support
- [ ] Hyperlink detection and OSC 8 support
- [ ] Clickable links
- [ ] Mouse reporting (all protocols)
- [ ] Scrolling (normal and alternate screen)
- [ ] Context menus

### Phase 4: Advanced UI (Weeks 10-12)
- [ ] Split panes (horizontal/vertical)
- [ ] Pane navigation and resizing
- [ ] Command palette
- [ ] Search in buffer
- [ ] Keyboard shortcut customization
- [ ] Settings UI
- [ ] Default theme collection (Catppuccin, Tokyo Night, etc.)

### Phase 5: Optimization & Polish (Weeks 13-14)
- [ ] Damage tracking optimization
- [ ] Glyph caching
- [ ] Benchmark and profile rendering performance
- [ ] Smooth animations (tab switch, theme transitions)
- [ ] Accessibility (screen reader support, high contrast mode)
- [ ] Platform-specific enhancements

### Phase 6: Testing & Documentation (Weeks 15-16)
- [ ] Unit tests for core components
- [ ] Integration tests for UI interactions
- [ ] Performance benchmarks vs. Alacritty/WezTerm
- [ ] User documentation
- [ ] Theme creation guide
- [ ] Plugin API design (future extensibility)

---

## Appendix A: Performance Targets

### Rendering Performance
- **Input Latency:** < 10ms (keystroke to pixel)
- **Frame Rate:** 60fps minimum (120fps on high-refresh displays)
- **Startup Time:** < 200ms (cold start)
- **Memory Usage:** < 100MB (single terminal session)
- **Grid Render:** < 2ms (1920x1080 grid, full redraw)

### Optimization Strategies
1. **Damage Tracking:** Only redraw changed cells (target: < 0.5ms for 10 changed cells)
2. **Glyph Caching:** Pre-render glyphs to GPU texture atlas
3. **Async Rendering:** PTY I/O on separate thread from rendering
4. **Smart Reflows:** Minimize layout recalculations
5. **Lazy Theme Loading:** Load theme assets on-demand

---

## Appendix B: Accessibility

### Screen Reader Support
- Expose terminal content via platform accessibility APIs
- Announce new output via accessible events
- Keyboard-only navigation for all features

### Visual Accessibility
- High contrast mode (boost color separation)
- Configurable cursor size and shape
- Minimum font size limits
- Respect system "reduce motion" settings

### Color Blindness Support
- Theme validation for color contrast (WCAG AA standard)
- Alternative color schemes optimized for protanopia, deuteranopia, tritanopia
- Visual indicators beyond color (e.g., bold, underline for emphasis)

---

## Appendix C: Technical Stack Summary

### Core Dependencies
```toml
[dependencies]
# GUI Framework
iced = { version = "0.12", features = ["canvas", "wgpu"] }

# Font Rendering
cosmic-text = "0.10"

# Terminal Core (assumed to exist)
# pty-process = "0.4"  # PTY management
# vte = "0.13"          # ANSI parser

# Theme/Config
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Utilities
anyhow = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }

# Platform-Specific (conditionally compiled)
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
objc = "0.2"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = ["Win32_Graphics_Dwm"] }
```

### Recommended Project Structure
```
termiemu/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Iced application logic
│   ├── terminal/            # Terminal core
│   │   ├── mod.rs
│   │   ├── grid.rs          # Text grid buffer
│   │   ├── pty.rs           # PTY interface
│   │   └── parser.rs        # VTE integration
│   ├── ui/                  # UI components
│   │   ├── mod.rs
│   │   ├── grid_widget.rs   # Custom grid canvas
│   │   ├── tab_bar.rs
│   │   ├── scrollbar.rs
│   │   ├── command_palette.rs
│   │   └── pane.rs
│   ├── theme/               # Theming engine
│   │   ├── mod.rs
│   │   ├── loader.rs
│   │   └── defaults.rs      # Built-in themes
│   ├── font/                # Font rendering
│   │   ├── mod.rs
│   │   ├── cache.rs
│   │   └── shaper.rs        # cosmic-text integration
│   ├── input/               # Input handling
│   │   ├── mod.rs
│   │   ├── keyboard.rs
│   │   └── mouse.rs
│   └── config/              # Configuration
│       ├── mod.rs
│       └── keybindings.rs
├── themes/                  # Default themes
│   ├── catppuccin-mocha.toml
│   ├── tokyo-night.toml
│   └── ...
└── assets/
    └── fonts/               # Bundled fallback fonts
```

---

## Conclusion

This design document provides a comprehensive blueprint for building TermiEmu, a modern, high-performance terminal emulator in Rust. The design balances **performance** (GPU acceleration, damage tracking), **aesthetics** (minimal chrome, smooth animations, beautiful themes), and **user experience** (intuitive selection, powerful command palette, seamless mouse integration).

**Key Differentiators:**
1. **Iced Framework:** Optimal balance of performance and developer ergonomics for reactive terminal UI
2. **cosmic-text:** World-class font rendering with ligatures, emoji, and complex scripts
3. **Fluid Minimalism:** Every pixel and millisecond optimized for user focus
4. **Extensible Architecture:** Theme engine, command palette, and planned plugin system for future growth

The phased roadmap ensures incremental delivery, with core rendering (Phase 1) enabling early testing, visual polish (Phase 2) establishing the aesthetic, and interaction layers (Phases 3-4) completing the user experience.

---

**Next Steps:**
1. **Prototype:** Implement Phase 1 (Core Rendering) to validate architecture
2. **User Testing:** Gather feedback on tab bar, scrollbar, and selection UX
3. **Performance Benchmarking:** Compare against Alacritty for rendering speed
4. **Theme Gallery:** Curate and test top 10 themes from community favorites

**For Questions or Contributions:**  
Refer to the project repository's CONTRIBUTING.md for development guidelines.

---

*Document Version: 1.0*  
*Last Updated: November 2025*  
*Author: TermiEmu Design Team*
