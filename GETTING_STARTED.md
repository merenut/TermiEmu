# Getting Started with TermiEmu Development

This guide helps developers get started implementing the TermiEmu design. It assumes you've read the [Design Summary](./DESIGN_SUMMARY.md) and provides practical steps for implementation.

## Prerequisites

### Required Knowledge
- **Rust:** Intermediate level (traits, async/await, error handling)
- **GUI Concepts:** Basic understanding of UI frameworks
- **Terminal Emulation:** Familiarity with ANSI escape codes (helpful but not required)

### Development Environment
```bash
# Rust toolchain (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Platform-specific dependencies
# macOS: Xcode Command Line Tools
xcode-select --install

# Linux (Ubuntu/Debian): Build essentials
sudo apt install build-essential pkg-config libfontconfig1-dev

# Windows: Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
```

## Quick Start: Minimal Prototype (Phase 1)

### Step 1: Initialize Project

```bash
# Create new Rust project
cargo new termiemu --bin
cd termiemu

# Add dependencies to Cargo.toml
```

**Cargo.toml:**
```toml
[package]
name = "termiemu"
version = "0.1.0"
edition = "2021"

[dependencies]
# GUI Framework
iced = { version = "0.12", features = ["canvas", "tokio"] }

# Font Rendering
cosmic-text = "0.10"

# Terminal Core
vte = "0.13"              # ANSI parser
portable-pty = "0.8"      # Cross-platform PTY

# Async Runtime
tokio = { version = "1", features = ["full"] }

# Utilities
anyhow = "1.0"
```

### Step 2: Create Basic Structure

```
termiemu/
├── src/
│   ├── main.rs              # Application entry
│   ├── app.rs               # Iced application
│   ├── terminal.rs          # Terminal logic
│   ├── grid.rs              # Text grid buffer
│   └── grid_widget.rs       # Custom Canvas widget
```

### Step 3: Implement Minimal Application

**src/main.rs:**
```rust
use iced::{Application, Settings};

mod app;
mod terminal;
mod grid;
mod grid_widget;

fn main() -> iced::Result {
    app::TermiEmu::run(Settings {
        window: iced::window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })
}
```

**src/app.rs (Minimal Iced App):**
```rust
use iced::{
    widget::{column, container, text},
    Application, Command, Element, Theme,
};

use crate::terminal::Terminal;
use crate::grid_widget::GridWidget;

pub struct TermiEmu {
    terminal: Terminal,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed(iced::keyboard::Key),
    TerminalOutput(Vec<u8>),
    Tick,
}

impl Application for TermiEmu {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let terminal = Terminal::new(80, 24);
        (Self { terminal }, Command::none())
    }

    fn title(&self) -> String {
        String::from("TermiEmu - Prototype")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KeyPressed(key) => {
                // TODO: Send to PTY
                println!("Key pressed: {:?}", key);
            }
            Message::TerminalOutput(data) => {
                // TODO: Parse and update grid
                self.terminal.process_output(&data);
            }
            Message::Tick => {
                // TODO: Check for PTY output
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                text("TermiEmu Prototype").size(20),
                GridWidget::new(&self.terminal.grid)
            ]
        )
        .padding(10)
        .into()
    }
}
```

**src/terminal.rs (Basic Terminal State):**
```rust
use crate::grid::Grid;

pub struct Terminal {
    pub grid: Grid,
    cursor_x: usize,
    cursor_y: usize,
}

impl Terminal {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            grid: Grid::new(cols, rows),
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn process_output(&mut self, _data: &[u8]) {
        // TODO: Parse with VTE and update grid
    }
}
```

**src/grid.rs (Text Grid Buffer):**
```rust
#[derive(Clone, Debug)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            bold: false,
            italic: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
}

pub struct Grid {
    cells: Vec<Cell>,
    cols: usize,
    rows: usize,
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cells: vec![Cell::default(); cols * rows],
            cols,
            rows,
        }
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&Cell> {
        if col < self.cols && row < self.rows {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, col: usize, row: usize, cell: Cell) {
        if col < self.cols && row < self.rows {
            self.cells[row * self.cols + col] = cell;
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }
}
```

**src/grid_widget.rs (Custom Rendering):**
```rust
use iced::{
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Text},
    Color, Point, Rectangle, Renderer, Size, Theme,
};

use crate::grid::Grid;

pub struct GridWidget<'a> {
    grid: &'a Grid,
}

impl<'a> GridWidget<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }
}

impl<'a> canvas::Program<Message> for GridWidget<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let (cols, rows) = self.grid.dimensions();
        let cell_width = bounds.width / cols as f32;
        let cell_height = bounds.height / rows as f32;

        // Simple text rendering (will be replaced with cosmic-text)
        for row in 0..rows {
            for col in 0..cols {
                if let Some(cell) = self.grid.get(col, row) {
                    let x = col as f32 * cell_width;
                    let y = row as f32 * cell_height;

                    // Draw background
                    let bg_rect = Rectangle {
                        x,
                        y,
                        width: cell_width,
                        height: cell_height,
                    };
                    frame.fill_rectangle(
                        bg_rect.position(),
                        bg_rect.size(),
                        Color::from_rgb8(cell.bg.r, cell.bg.g, cell.bg.b),
                    );

                    // Draw character (simplified - use cosmic-text in real impl)
                    if cell.c != ' ' {
                        frame.fill_text(Text {
                            content: cell.c.to_string(),
                            position: Point::new(x, y + cell_height * 0.8),
                            color: Color::from_rgb8(cell.fg.r, cell.fg.g, cell.fg.b),
                            size: cell_height,
                            ..Default::default()
                        });
                    }
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
pub enum Message {}
```

### Step 4: Run the Prototype

```bash
cargo run
```

You should see a window with "TermiEmu Prototype" and an empty grid!

## Next Steps: Building Real Features

### Phase 1: Core Rendering (Current)

#### 1. Integrate PTY
```rust
// In terminal.rs
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

pub struct Terminal {
    // ... existing fields
    pty_pair: portable_pty::PtyPair,
    child: Box<dyn portable_pty::Child + Send>,
}

impl Terminal {
    pub fn new_with_pty(cols: usize, rows: usize) -> anyhow::Result<Self> {
        let pty_system = native_pty_system();
        let pty_pair = pty_system.openpty(PtySize {
            rows: rows as u16,
            cols: cols as u16,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let cmd = CommandBuilder::new("bash");
        let child = pty_pair.slave.spawn_command(cmd)?;
        
        // TODO: Spawn async reader task
        
        Ok(Self {
            grid: Grid::new(cols, rows),
            cursor_x: 0,
            cursor_y: 0,
            pty_pair,
            child,
        })
    }
}
```

#### 2. Add VTE Parser
```rust
// In terminal.rs
use vte::{Perform, Parser};

impl Perform for Terminal {
    fn print(&mut self, c: char) {
        self.grid.set(self.cursor_x, self.cursor_y, Cell {
            c,
            ..Default::default()
        });
        self.cursor_x += 1;
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
            b'\r' => {
                self.cursor_x = 0;
            }
            _ => {}
        }
    }

    // Implement other Perform methods...
}
```

#### 3. Integrate cosmic-text
```rust
// In grid_widget.rs
use cosmic_text::{FontSystem, SwashCache, Buffer};

pub struct GridRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    // ... other fields
}

impl GridRenderer {
    pub fn render_cell(&mut self, cell: &Cell, x: f32, y: f32) {
        // Use cosmic-text to shape and render glyphs
        // This handles ligatures, emoji, complex scripts automatically
    }
}
```

### Phase 2: Visual Polish

#### 1. Theme System
```rust
// Create src/theme.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Theme {
    pub colors: ColorScheme,
    pub ui: UiColors,
}

pub fn load_theme(path: &str) -> anyhow::Result<Theme> {
    let contents = std::fs::read_to_string(path)?;
    let theme: Theme = toml::from_str(&contents)?;
    Ok(theme)
}
```

#### 2. Add Tab Bar
```rust
// Create src/tab_bar.rs
use iced::widget::{button, row, text};

pub fn tab_bar(tabs: &[String], active: usize) -> Element<Message> {
    let mut tabs_row = row![];
    
    for (i, name) in tabs.iter().enumerate() {
        let tab = button(text(name))
            .on_press(Message::SelectTab(i))
            .style(if i == active {
                TabStyle::Active
            } else {
                TabStyle::Inactive
            });
        tabs_row = tabs_row.push(tab);
    }
    
    tabs_row.into()
}
```

### Phase 3: Interaction

#### 1. Text Selection
```rust
// In terminal.rs
pub struct Selection {
    start: (usize, usize),  // (col, row)
    end: (usize, usize),
    active: bool,
}

impl Terminal {
    pub fn start_selection(&mut self, col: usize, row: usize) {
        self.selection = Some(Selection {
            start: (col, row),
            end: (col, row),
            active: true,
        });
    }

    pub fn extend_selection(&mut self, col: usize, row: usize) {
        if let Some(sel) = &mut self.selection {
            sel.end = (col, row);
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        // Extract text from grid within selection bounds
    }
}
```

#### 2. Mouse Handling
```rust
// In grid_widget.rs
impl canvas::Program<Message> for GridWidget<'_> {
    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: canvas::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse_event) => {
                if let Some(position) = cursor.position_in(bounds) {
                    let col = (position.x / cell_width) as usize;
                    let row = (position.y / cell_height) as usize;
                    
                    match mouse_event {
                        iced::mouse::Event::ButtonPressed(_) => {
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::SelectionStart(col, row))
                            );
                        }
                        // Handle other mouse events...
                    }
                }
            }
        }
        (canvas::event::Status::Ignored, None)
    }
}
```

## Development Workflow

### Build & Run
```bash
# Debug build (fast compilation, slower runtime)
cargo run

# Release build (slower compilation, fast runtime)
cargo run --release

# Check for errors without building
cargo check

# Run with logging
RUST_LOG=debug cargo run
```

### Testing
```bash
# Run unit tests
cargo test

# Run specific test
cargo test grid_tests

# Run with output
cargo test -- --nocapture
```

### Linting
```bash
# Install clippy (if not already)
rustup component add clippy

# Run linter
cargo clippy

# Run with stricter checks (what CI uses)
cargo clippy --all-targets --all-features -- -D warnings

# Fix automatically fixable issues
cargo clippy --fix
```

### Formatting
```bash
# Install rustfmt (if not already)
rustup component add rustfmt

# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

### Pre-commit Hooks (Optional)

TermiEmu provides optional pre-commit hooks that automatically check formatting and linting before each commit. This helps catch issues early in your development workflow.

To enable pre-commit hooks:
1. Uncomment the `cargo-husky` line in `Cargo.toml`
2. Run `cargo build` to install the hooks
3. See `.cargo-husky/hooks/README.md` for details

For complete information about code quality standards, see [CODE_QUALITY.md](./CODE_QUALITY.md).

## Common Issues & Solutions

### Issue: Iced won't compile
**Solution:** Make sure you have the latest stable Rust:
```bash
rustup update stable
```

### Issue: Font rendering looks blurry
**Solution:** Enable subpixel antialiasing in cosmic-text and check DPI scaling.

### Issue: Input latency is high
**Solution:** 
1. Use release builds for testing performance
2. Check that PTY reading is on separate thread
3. Ensure damage tracking is working (not redrawing entire grid)

### Issue: Mouse clicks aren't registering
**Solution:** Make sure the Canvas widget is capturing mouse events:
```rust
canvas::Event::Mouse(event) => {
    // Return Status::Captured for mouse events you handle
    (canvas::event::Status::Captured, Some(message))
}
```

## Resources

### Iced Framework
- **Docs:** https://docs.rs/iced/latest/iced/
- **Examples:** https://github.com/iced-rs/iced/tree/master/examples
- **Book:** https://book.iced.rs/

### cosmic-text
- **Docs:** https://docs.rs/cosmic-text/latest/cosmic_text/
- **GitHub:** https://github.com/pop-os/cosmic-text

### Terminal Emulation
- **VTE Spec:** https://vt100.net/docs/vt100-ug/
- **ANSI Escape Codes:** https://en.wikipedia.org/wiki/ANSI_escape_code
- **XTerm Control Sequences:** https://invisible-island.net/xterm/ctlseqs/ctlseqs.html

### Similar Projects (for reference)
- **Alacritty:** https://github.com/alacritty/alacritty
- **WezTerm:** https://github.com/wez/wezterm
- **Rio Terminal:** https://github.com/raphamorim/rio

## Getting Help

1. **Design Questions:** Review the [GUI/UX Design Document](./GUI_UX_DESIGN.md)
2. **Architecture Questions:** See [Architecture Overview](./ARCHITECTURE.md)
3. **Iced Issues:** Post on [Iced Discourse](https://discourse.iced.rs/)
4. **General Rust:** Check the [Rust Programming Language Book](https://doc.rust-lang.org/book/)

## Next Phase Checklist

After completing the minimal prototype, refer to the [Implementation Roadmap](./GUI_UX_DESIGN.md#6-implementation-roadmap) for Phase 2 tasks.

**Current Phase 1 Goals:**
- [ ] Working PTY integration
- [ ] VTE parser integrated with grid updates
- [ ] cosmic-text rendering (no ligatures yet)
- [ ] Basic keyboard input → PTY → output → display loop
- [ ] 16 ANSI colors working
- [ ] Cursor rendering

**Success Criteria:**
You can run `ls --color=auto` in TermiEmu and see colored, correctly formatted output! 🎉

---

*Happy coding! Remember: Start small, iterate fast, and reference the design docs often.*
