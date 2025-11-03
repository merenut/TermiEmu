# Phase 2 (Beta) GUI Implementation Plan
# Advanced Features & Visual Polish

**Project:** TermiEmu - Modern Terminal Emulator (Rust + Iced + cosmic-text)  
**Phase:** 2 - Beta (Weeks 29-44, 12-16 weeks)  
**Status:** Planning Complete ✅  
**Date:** November 3, 2024  
**Version:** 1.0

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current Status](#current-status)
3. [Phase 2 Goals & Objectives](#phase-2-goals--objectives)
4. [User Story Details](#user-story-details)
5. [Technical Architecture](#technical-architecture)
6. [Dependencies & Sequencing](#dependencies--sequencing)
7. [Timeline & Estimates](#timeline--estimates)
8. [Risk Assessment](#risk-assessment)
9. [Success Criteria](#success-criteria)
10. [GitHub Issue Templates](#github-issue-templates)

---

## Executive Summary

Phase 2 represents the transition from a functional terminal emulator to a production-ready application with advanced GUI features, performance optimization, and cross-platform polish. This phase builds upon the solid foundation of Phase 0 (infrastructure) and Phase 1 (core features) to deliver a complete, competitive terminal emulator.

### Key Priorities

1. **Custom Iced Terminal Widget** (Foundation) - XXL effort, 3-4 weeks
2. **Split Panes** (US-034) - XL effort, 2-3 weeks, CRITICAL
3. **Performance Optimization** (US-046) - L effort, 1-2 weeks, CRITICAL
4. **Cross-Platform Support** (US-038, US-039, US-040) - Combined L effort
5. **Advanced Features** (US-042, US-043, US-044, US-045, US-049) - Medium priority

### Phase 2 Deliverables

- ✅ Custom canvas-based terminal widget with GPU acceleration
- ✅ Split panes with keyboard and mouse navigation
- ✅ Hyperlink support (OSC 8) with hover and click
- ✅ Search in buffer with regex support
- ✅ Command palette with fuzzy search
- ✅ Performance benchmarking UI
- ✅ <10ms input latency achieved
- ✅ 60fps rendering maintained
- ✅ Cross-platform builds (Linux, macOS, Windows)
- ✅ Comprehensive user documentation

---

## Current Status

### ✅ Phase 0 Complete (Foundation - Weeks 1-8)

**Completed Features:**
- Project infrastructure and CI/CD
- PTY integration (portable-pty)
- VTE parser (ANSI/ECMA-48)
- Grid buffer with scrollback
- Basic Iced application shell
- Keyboard input handling
- Color support (16/256/truecolor)
- Unicode and UTF-8 support

**Test Coverage:** 92 tests (100% passing)  
**Code Quality:** All clippy lints addressed  
**Documentation:** ARCHITECTURE.md, GETTING_STARTED.md, API docs

### ✅ Phase 1 Complete (Alpha - Weeks 9-28)

**Completed Features:**
- Alternate screen buffer (US-018)
- Terminal mode management (US-020)
- Configuration system (US-036) with TOML
- Theme system (US-037) with 4 built-in themes
- Clipboard integration (US-031) cross-platform
- Text selection infrastructure (US-041)

**Test Coverage:** 92 tests (100% passing)  
**Memory Footprint:** <1MB typical  
**Cross-Platform:** Linux, macOS, Windows CI passing

### 🔄 Phase 2 Starting (Beta - Weeks 29-44)

**Current Sprint:** Week 29  
**Team Size:** 2-3 developers  
**Focus:** GUI widget foundation and advanced features

---

## Phase 2 Goals & Objectives

### Primary Goals

1. **Build Custom Terminal Widget**
   - Canvas-based rendering with wgpu
   - cosmic-text integration for text shaping
   - GPU-accelerated rendering
   - Glyph cache with texture atlas
   - Damage tracking for efficiency
   - Mouse event handling
   - Selection rendering

2. **Implement Advanced GUI Features**
   - Split panes (horizontal/vertical)
   - Search in buffer
   - Hyperlink support
   - Command palette
   - Performance overlay

3. **Achieve Performance Targets**
   - <10ms input latency (P95)
   - 60fps rendering (16.67ms frame time)
   - <100MB memory usage
   - <200ms startup time

4. **Cross-Platform Production Quality**
   - Linux (X11/Wayland)
   - macOS (Intel/Apple Silicon)
   - Windows 10+ (ConPTY)

5. **Documentation & Polish**
   - User documentation
   - Visual guides with screenshots
   - Keyboard shortcuts reference
   - Troubleshooting guide

### Secondary Goals

- Mouse support integration (infrastructure ready)
- Scrollbar widget
- Tab management UI
- Community engagement

---

## User Story Details

### Foundation: Custom Iced Terminal Widget

**Priority:** CRITICAL (Blocks all other GUI work)  
**Effort:** XXL (3-4 weeks)  
**Complexity:** Very High  
**Assigned To:** Lead developer(s)

#### As a developer
**I want** a custom canvas-based widget for terminal rendering  
**So that** I have full control over rendering and can achieve performance targets

#### Acceptance Criteria

- [ ] Custom widget implements `iced::widget::canvas::Program` trait
- [ ] Renders to wgpu surface with GPU acceleration
- [ ] Integrates cosmic-text for text shaping and rendering
- [ ] Implements glyph cache with LRU eviction
- [ ] Supports texture atlas for GPU-resident glyphs
- [ ] Damage tracking to minimize redraws
- [ ] Mouse event handling (clicks, drags, hovers)
- [ ] Selection rendering with visual feedback
- [ ] Cursor rendering with configurable blink animation
- [ ] Handles window resize efficiently
- [ ] Layer rendering: background → text → cursor → selection → overlays
- [ ] HiDPI/Retina display support with proper scaling
- [ ] <16ms frame time for full screen updates
- [ ] <2ms frame time for typical updates (10-100 cells)

#### Technical Design

**Architecture:**

```rust
// src/ui/terminal_widget.rs

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Program};
use cosmic_text::{FontSystem, SwashCache, Buffer as TextBuffer};
use wgpu;

pub struct TerminalWidget {
    // Core state
    grid: Arc<Mutex<Grid>>,
    font_renderer: FontRenderer,
    glyph_cache: GlyphCache,
    
    // Rendering state
    cache: canvas::Cache,
    damage_tracker: DamageTracker,
    
    // Visual state
    cursor_blink_state: bool,
    selection: Option<Selection>,
}

pub struct FontRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    font_size: f32,
    cell_width: f32,
    cell_height: f32,
    line_height: f32,
}

pub struct GlyphCache {
    cache: LruCache<GlyphKey, CachedGlyph>,
    atlas: TextureAtlas,
    max_size: usize,
}

#[derive(Hash, Eq, PartialEq)]
struct GlyphKey {
    glyph_id: u32,
    font_id: u32,
    size: u32,
    flags: CellFlags,  // bold, italic, etc.
}

struct CachedGlyph {
    texture_coords: Rectangle,
    metrics: GlyphMetrics,
}

pub struct TextureAtlas {
    texture: wgpu::Texture,
    width: u32,
    height: u32,
    allocator: RectAllocator,
}

impl Program<Message> for TerminalWidget {
    type State = ();
    
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Layer 1: Background
            self.draw_background(frame, bounds);
            
            // Layer 2: Cell backgrounds (for colored cells)
            self.draw_cell_backgrounds(frame, bounds);
            
            // Layer 3: Text (via glyph cache)
            self.draw_text(frame, bounds);
            
            // Layer 4: Cursor
            self.draw_cursor(frame, bounds);
            
            // Layer 5: Selection overlay
            if let Some(selection) = &self.selection {
                self.draw_selection(frame, bounds, selection);
            }
            
            // Layer 6: Overlays (hyperlink underlines, etc.)
            self.draw_overlays(frame, bounds);
        });
        
        vec![geometry]
    }
    
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: canvas::Cursor,
    ) -> (event::Status, Option<Message>) {
        // Handle mouse events
        // Handle keyboard events (forwarded from parent)
        // Update damage tracker
        // Invalidate cache if needed
    }
}
```

**Glyph Cache Implementation:**

```rust
impl GlyphCache {
    pub fn get_or_render(&mut self, key: GlyphKey) -> &CachedGlyph {
        if let Some(glyph) = self.cache.get(&key) {
            return glyph;
        }
        
        // Render glyph using cosmic-text/swash
        let rendered = self.render_glyph(key);
        
        // Allocate space in texture atlas
        let texture_coords = self.atlas.allocate(
            rendered.width,
            rendered.height
        );
        
        // Upload to GPU
        self.atlas.upload(texture_coords, &rendered.pixels);
        
        let cached = CachedGlyph {
            texture_coords,
            metrics: rendered.metrics,
        };
        
        self.cache.put(key, cached);
        self.cache.get(&key).unwrap()
    }
    
    fn render_glyph(&self, key: GlyphKey) -> RenderedGlyph {
        // Use cosmic-text SwashCache to render glyph
        // Apply attributes (bold, italic) via font variations
        // Return rasterized pixels + metrics
    }
}
```

**Damage Tracking:**

```rust
pub struct DamageTracker {
    dirty_cells: BitSet,
    dirty_regions: Vec<Rectangle>,
    full_redraw: bool,
    cols: usize,
    rows: usize,
}

impl DamageTracker {
    pub fn mark_dirty(&mut self, col: usize, row: usize) {
        if self.full_redraw {
            return;
        }
        let index = row * self.cols + col;
        self.dirty_cells.insert(index);
    }
    
    pub fn mark_dirty_line(&mut self, row: usize) {
        for col in 0..self.cols {
            self.mark_dirty(col, row);
        }
    }
    
    pub fn get_dirty_regions(&mut self) -> Vec<Rectangle> {
        if self.full_redraw {
            return vec![Rectangle::new(0, 0, self.cols, self.rows)];
        }
        
        // Merge adjacent dirty cells into regions for efficiency
        let mut regions = Vec::new();
        // ... region merging algorithm
        regions
    }
    
    pub fn clear(&mut self) {
        self.dirty_cells.clear();
        self.dirty_regions.clear();
        self.full_redraw = false;
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Basic Canvas Widget (Week 29, 5 days)**
- [ ] Create `src/ui/terminal_widget.rs`
- [ ] Implement basic `Program` trait
- [ ] Render simple colored rectangles for cells
- [ ] Handle window resize
- [ ] Test: Widget appears in Iced app

**Task 2: cosmic-text Integration (Week 29-30, 5 days)**
- [ ] Add `FontRenderer` with cosmic-text `FontSystem`
- [ ] Load system fonts
- [ ] Calculate cell metrics (width, height)
- [ ] Shape simple ASCII text
- [ ] Render shaped glyphs to buffer
- [ ] Test: "Hello, World!" appears in terminal

**Task 3: Glyph Cache & Texture Atlas (Week 30-31, 7 days)**
- [ ] Implement `GlyphCache` with LRU
- [ ] Implement `TextureAtlas` with wgpu
- [ ] Rectangle packing algorithm (guillotine or shelf)
- [ ] Upload glyphs to GPU texture
- [ ] Render from atlas to canvas
- [ ] Test: Cache hit rate >95%

**Task 4: Full Grid Rendering (Week 31, 5 days)**
- [ ] Integrate with existing `Grid` structure
- [ ] Render all visible cells
- [ ] Apply foreground/background colors
- [ ] Apply text attributes (bold, italic, underline)
- [ ] Handle wide characters (CJK)
- [ ] Test: Can display output from `ls --color`

**Task 5: Damage Tracking (Week 32, 3 days)**
- [ ] Implement `DamageTracker`
- [ ] Mark dirty cells on grid updates
- [ ] Optimize redraw to dirty regions only
- [ ] Benchmark: <2ms for 10-cell updates
- [ ] Test: Full screen scroll is efficient

**Task 6: Mouse Events (Week 32, 2 days)**
- [ ] Capture mouse clicks in widget
- [ ] Convert pixel coordinates to grid coordinates
- [ ] Emit messages for clicks/drags
- [ ] Test: Click detection works

**Task 7: Selection Rendering (Week 32, 2 days)**
- [ ] Render selection overlay
- [ ] Handle different selection types
- [ ] Visual feedback on drag
- [ ] Test: Selection visible

**Task 8: Cursor Rendering (Week 32, 2 days)**
- [ ] Render cursor with current style
- [ ] Implement blink animation
- [ ] Different cursor styles (block, beam, underline)
- [ ] Test: Cursor blinks at configurable rate

**Task 9: HiDPI/Retina Support (Week 33, 2 days)**
- [ ] Detect display scale factor
- [ ] Scale font sizes appropriately
- [ ] Scale texture atlas resolution
- [ ] Test on HiDPI displays

**Task 10: Performance Optimization (Week 33, 3 days)**
- [ ] Profile rendering pipeline
- [ ] Optimize hot paths
- [ ] Reduce allocations
- [ ] Target: <16ms frame time
- [ ] Test: Run vtebench

#### Dependencies

- **Required:** Phase 0 & Phase 1 complete
- **Required:** cosmic-text, wgpu, iced canvas feature
- **Blocks:** All other Phase 2 GUI work

#### Rust Crates

- `iced` (canvas feature)
- `cosmic-text` = "0.12"
- `wgpu` (via iced)
- `swash` (via cosmic-text)
- `lru` = "0.12"
- `bitset` = "0.7"

#### Testing Strategy

**Unit Tests:**
- [ ] Glyph cache hit/miss scenarios
- [ ] Texture atlas allocation
- [ ] Damage tracking mark/clear
- [ ] Cell coordinate conversion

**Integration Tests:**
- [ ] Render full grid
- [ ] Render with colors and attributes
- [ ] Render wide characters
- [ ] Render selection
- [ ] Render cursor

**Performance Tests:**
- [ ] Benchmark full screen redraw (<16ms)
- [ ] Benchmark partial redraw (<2ms)
- [ ] Benchmark glyph cache performance
- [ ] Memory usage under load (<100MB)

**Visual Tests:**
- [ ] Screenshot comparison with reference
- [ ] Manual inspection of rendering quality
- [ ] Font rendering quality check

#### Risk Assessment

**Risk 1: cosmic-text complexity**
- **Impact:** High
- **Mitigation:** Start early, engage with community
- **Contingency:** Fall back to simpler text rendering

**Risk 2: GPU memory limits**
- **Impact:** Medium
- **Mitigation:** Implement texture atlas eviction
- **Contingency:** Reduce atlas size, cache fewer glyphs

**Risk 3: Performance targets not met**
- **Impact:** High
- **Mitigation:** Profile early and often, optimize iteratively
- **Contingency:** Adjust targets based on hardware

#### Acceptance Criteria Summary

- [ ] Widget renders full terminal grid correctly
- [ ] Supports all text attributes and colors
- [ ] Handles wide characters correctly
- [ ] Selection rendering works
- [ ] Cursor rendering with blink
- [ ] HiDPI displays supported
- [ ] <16ms full screen redraw
- [ ] <2ms typical updates
- [ ] Glyph cache hit rate >95%
- [ ] Memory usage <100MB
- [ ] All tests passing

---

### US-034: Split Panes

**Priority:** HIGH (CRITICAL for Phase 2)  
**Effort:** XL (2-3 weeks)  
**Complexity:** High  
**Dependencies:** Custom Terminal Widget complete

#### As a terminal user
**I want** to split the terminal window into multiple panes  
**So that** I can view and interact with multiple terminals simultaneously

#### Acceptance Criteria

- [ ] Split horizontally (Ctrl+Shift+D)
- [ ] Split vertically (Ctrl+Shift+E)
- [ ] Close individual panes (Ctrl+Shift+W)
- [ ] Navigate between panes (Alt+Arrow keys)
- [ ] Resize panes by dragging dividers
- [ ] Resize panes with keyboard (Alt+Shift+Arrow)
- [ ] Visual focus indicator on active pane
- [ ] Each pane has independent PTY and terminal state
- [ ] Pane layouts persist in session configuration
- [ ] Smooth divider dragging with live preview
- [ ] Minimum pane size enforced (e.g., 10 cols × 3 rows)
- [ ] Works with tab system (tabs can contain split layouts)

#### Technical Design

**Data Structure:**

```rust
// src/ui/panes.rs

use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum PaneLayout {
    Terminal(PaneId),
    Horizontal { top: Box<PaneLayout>, bottom: Box<PaneLayout>, ratio: f32 },
    Vertical { left: Box<PaneLayout>, right: Box<PaneLayout>, ratio: f32 },
}

pub struct PaneTree {
    root: PaneLayout,
    panes: HashMap<PaneId, Pane>,
    active_pane: PaneId,
    next_id: usize,
}

pub struct Pane {
    id: PaneId,
    terminal: Arc<Mutex<Terminal>>,
    pty: Arc<Mutex<Pty>>,
    bounds: Rectangle,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PaneId(usize);

impl PaneTree {
    pub fn new() -> Self {
        let initial_pane = Pane::new(PaneId(0));
        let mut panes = HashMap::new();
        panes.insert(PaneId(0), initial_pane);
        
        Self {
            root: PaneLayout::Terminal(PaneId(0)),
            panes,
            active_pane: PaneId(0),
            next_id: 1,
        }
    }
    
    pub fn split_horizontal(&mut self, pane_id: PaneId) -> PaneId {
        let new_id = self.allocate_pane();
        self.root = self.split_layout_horizontal(self.root.clone(), pane_id, new_id, 0.5);
        self.active_pane = new_id;
        new_id
    }
    
    pub fn split_vertical(&mut self, pane_id: PaneId) -> PaneId {
        let new_id = self.allocate_pane();
        self.root = self.split_layout_vertical(self.root.clone(), pane_id, new_id, 0.5);
        self.active_pane = new_id;
        new_id
    }
    
    pub fn close_pane(&mut self, pane_id: PaneId) -> bool {
        // Remove pane and simplify tree
        // Return false if last pane
    }
    
    pub fn navigate(&mut self, direction: Direction) {
        // Find pane in given direction from active pane
        // Update active_pane
    }
    
    pub fn resize_divider(&mut self, divider_id: DividerId, delta: f32) {
        // Update ratio in layout tree
    }
    
    pub fn calculate_bounds(&mut self, container: Rectangle) {
        // Recursively calculate bounds for all panes
        self.calculate_layout_bounds(&self.root, container);
    }
    
    fn calculate_layout_bounds(&mut self, layout: &PaneLayout, bounds: Rectangle) {
        match layout {
            PaneLayout::Terminal(id) => {
                if let Some(pane) = self.panes.get_mut(id) {
                    pane.bounds = bounds;
                }
            }
            PaneLayout::Horizontal { top, bottom, ratio } => {
                let split_y = bounds.y + (bounds.height * ratio);
                let top_bounds = Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width,
                    height: split_y - bounds.y,
                };
                let bottom_bounds = Rectangle {
                    x: bounds.x,
                    y: split_y,
                    width: bounds.width,
                    height: bounds.y + bounds.height - split_y,
                };
                self.calculate_layout_bounds(top, top_bounds);
                self.calculate_layout_bounds(bottom, bottom_bounds);
            }
            PaneLayout::Vertical { left, right, ratio } => {
                // Similar to horizontal
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Copy, Clone, Debug)]
pub struct DividerId {
    layout_path: Vec<usize>,  // Path through tree to divider
}
```

**Rendering:**

```rust
// Integration with Iced

impl Application for TermiEmuApp {
    fn view(&self) -> Element<Message> {
        let pane_tree = &self.pane_tree;
        
        // Recursively build Iced widget tree from pane layout
        self.build_pane_layout_widget(&pane_tree.root, pane_tree)
    }
    
    fn build_pane_layout_widget(
        &self,
        layout: &PaneLayout,
        tree: &PaneTree,
    ) -> Element<Message> {
        match layout {
            PaneLayout::Terminal(id) => {
                let pane = &tree.panes[id];
                let is_active = *id == tree.active_pane;
                
                // Build terminal widget with focus indicator
                container(TerminalWidget::new(pane.terminal.clone()))
                    .style(if is_active {
                        FocusedPaneStyle
                    } else {
                        UnfocusedPaneStyle
                    })
                    .into()
            }
            PaneLayout::Horizontal { top, bottom, ratio } => {
                // Create vertical split with draggable divider
                pane_grid::PaneGrid::new()
                    .push(self.build_pane_layout_widget(top, tree))
                    .push(divider())
                    .push(self.build_pane_layout_widget(bottom, tree))
                    .into()
            }
            PaneLayout::Vertical { left, right, ratio } => {
                // Create horizontal split with draggable divider
                // Similar to horizontal
            }
        }
    }
}
```

**Custom Divider Widget:**

```rust
pub struct Divider {
    orientation: Orientation,
    hover_state: bool,
}

pub enum Orientation {
    Horizontal,  // Divides top/bottom
    Vertical,    // Divides left/right
}

impl Widget for Divider {
    fn on_event(&mut self, event: Event) -> Status {
        match event {
            Event::Mouse(MouseEvent::CursorMoved { position }) => {
                // Update hover state
                // Change cursor to resize cursor
            }
            Event::Mouse(MouseEvent::ButtonPressed(MouseButton::Left)) => {
                // Start drag
            }
            Event::Mouse(MouseEvent::ButtonReleased(MouseButton::Left)) => {
                // End drag
            }
            Event::Mouse(MouseEvent::CursorMoved { position }) if dragging => {
                // Emit resize message
            }
            _ => {}
        }
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Pane Tree Data Structure (Week 34, 3 days)**
- [ ] Implement `PaneLayout` enum
- [ ] Implement `PaneTree` with split/close operations
- [ ] Implement bounds calculation
- [ ] Unit tests for tree operations

**Task 2: Split Operations (Week 34, 3 days)**
- [ ] Implement horizontal split
- [ ] Implement vertical split
- [ ] Each split creates new PTY + Terminal
- [ ] Test: Tree structure correct after splits

**Task 3: Pane Rendering (Week 35, 4 days)**
- [ ] Integrate with Iced layout
- [ ] Render each pane with TerminalWidget
- [ ] Apply focus indicator styling
- [ ] Test: Multiple panes render correctly

**Task 4: Divider Widget (Week 35, 3 days)**
- [ ] Create custom divider widget
- [ ] Handle mouse hover (cursor change)
- [ ] Handle drag start/drag/drop
- [ ] Emit resize messages
- [ ] Test: Divider responds to mouse

**Task 5: Divider Resizing (Week 36, 2 days)**
- [ ] Update ratio on drag
- [ ] Enforce minimum pane sizes
- [ ] Live preview during drag
- [ ] Test: Panes resize smoothly

**Task 6: Keyboard Navigation (Week 36, 2 days)**
- [ ] Implement navigate() method
- [ ] Find pane in direction from active
- [ ] Update active pane
- [ ] Test: Can navigate with Alt+Arrows

**Task 7: Keyboard Resizing (Week 36, 2 days)**
- [ ] Implement resize with Alt+Shift+Arrows
- [ ] Adjust ratio incrementally
- [ ] Test: Keyboard resize works

**Task 8: Pane Close (Week 36, 2 days)**
- [ ] Implement close operation
- [ ] Simplify tree after close
- [ ] Clean up PTY/Terminal resources
- [ ] Prevent closing last pane
- [ ] Test: Can close panes, tree simplifies

**Task 9: Session Persistence (Week 36, 2 days)**
- [ ] Serialize pane layout to config
- [ ] Deserialize on load
- [ ] Restore PTYs and terminals
- [ ] Test: Layout persists across restarts

**Task 10: Tab Integration (Week 37, 2 days)**
- [ ] Each tab has its own PaneTree
- [ ] Tab switching preserves pane state
- [ ] Test: Multiple tabs with different layouts

#### Dependencies

- **Required:** Custom Terminal Widget complete
- **Required:** US-033 (Tab Management) UI
- **Optional:** Configuration system (for persistence)

#### Rust Crates

- `iced` (existing)
- Consider: `iced_aw` for advanced widgets

#### Testing Strategy

**Unit Tests:**
- [ ] PaneTree split operations
- [ ] PaneTree close operations
- [ ] Bounds calculation accuracy
- [ ] Navigate direction finding

**Integration Tests:**
- [ ] Create multiple panes
- [ ] Navigate between panes
- [ ] Resize panes
- [ ] Close panes
- [ ] Session persistence

**Manual Tests:**
- [ ] Visual focus indicator
- [ ] Smooth divider dragging
- [ ] Minimum size enforcement
- [ ] Complex layouts (many splits)

#### Risk Assessment

**Risk 1: Iced layout complexity**
- **Impact:** High
- **Mitigation:** Prototype early with simple layouts
- **Contingency:** Simplify layout options (e.g., grid-only)

**Risk 2: PTY resource management**
- **Impact:** Medium
- **Mitigation:** Proper cleanup on pane close
- **Contingency:** Limit maximum panes

**Risk 3: Performance with many panes**
- **Impact:** Medium
- **Mitigation:** Damage tracking per pane
- **Contingency:** Warn user if too many panes

#### Acceptance Criteria Summary

- [ ] Can split horizontally and vertically
- [ ] Can close panes (except last)
- [ ] Can navigate with keyboard
- [ ] Can resize with mouse and keyboard
- [ ] Visual focus indicator
- [ ] Independent PTY per pane
- [ ] Layouts persist
- [ ] Works with tabs
- [ ] Smooth and responsive
- [ ] All tests passing

---

### US-042: Hyperlink Support (OSC 8)

**Priority:** MEDIUM  
**Effort:** M (1 week)  
**Complexity:** Medium  
**Dependencies:** Custom Terminal Widget complete

#### As a terminal user
**I want** clickable hyperlinks in terminal output  
**So that** I can open URLs and files directly from the terminal

#### Acceptance Criteria

- [ ] Parse OSC 8 hyperlink sequences: `ESC]8;;URL\ESCTEXT\ESC]8;;\ESC`
- [ ] Auto-detect URLs with regex (http://, https://, file://)
- [ ] Store hyperlink metadata in Grid cells
- [ ] Hover effect: underline + color change
- [ ] Cursor changes to pointer on hover
- [ ] Click to open link (Ctrl+Click or Cmd+Click on macOS)
- [ ] Right-click context menu: "Copy Link", "Open Link"
- [ ] Configurable URL regex patterns
- [ ] Security: URL validation before opening
- [ ] Visual differentiation: auto-detected vs OSC 8 links

#### Technical Design

**Cell Enhancement:**

```rust
// Extend Cell structure in src/terminal/cell.rs

#[derive(Clone, Debug)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
    pub flags: CellFlags,
    pub hyperlink: Option<Arc<Hyperlink>>,  // NEW
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hyperlink {
    pub url: String,
    pub id: Option<String>,  // Optional link ID from OSC 8
    pub source: HyperlinkSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HyperlinkSource {
    OSC8,          // Explicit OSC 8 sequence
    AutoDetected,  // Regex-detected URL
}
```

**URL Detection:**

```rust
// src/terminal/hyperlink.rs

use regex::Regex;
use once_cell::sync::Lazy;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        r"(?:https?|ftp|file)://",  // Protocol
        r"[-A-Za-z0-9+&@#/%?=~_|!:,.;]*",  // Domain/path
        r"[-A-Za-z0-9+&@#/%=~_|]",  // No trailing punctuation
    )).unwrap()
});

pub struct HyperlinkDetector {
    patterns: Vec<Regex>,
}

impl HyperlinkDetector {
    pub fn new() -> Self {
        Self {
            patterns: vec![URL_REGEX.clone()],
        }
    }
    
    pub fn add_pattern(&mut self, pattern: &str) -> Result<()> {
        let regex = Regex::new(pattern)?;
        self.patterns.push(regex);
        Ok(())
    }
    
    pub fn detect_in_line(&self, line: &[Cell]) -> Vec<DetectedLink> {
        let text: String = line.iter().map(|c| c.c).collect();
        let mut links = Vec::new();
        
        for pattern in &self.patterns {
            for m in pattern.find_iter(&text) {
                links.push(DetectedLink {
                    start_col: m.start(),
                    end_col: m.end(),
                    url: m.as_str().to_string(),
                });
            }
        }
        
        links
    }
}

#[derive(Debug)]
pub struct DetectedLink {
    pub start_col: usize,
    pub end_col: usize,
    pub url: String,
}
```

**Parser Integration:**

```rust
// In src/terminal/parser.rs

impl Perform for TerminalParser {
    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        match params.get(0) {
            Some(b"8") => {
                // OSC 8 hyperlink
                // Format: ESC]8;id;URL\ESC\ or \007
                if params.len() >= 3 {
                    let id = std::str::from_utf8(params[1]).ok();
                    let url = std::str::from_utf8(params[2]).ok();
                    
                    match (id, url) {
                        (Some(id), Some(url)) if !url.is_empty() => {
                            self.current_hyperlink = Some(Arc::new(Hyperlink {
                                url: url.to_string(),
                                id: if id.is_empty() { None } else { Some(id.to_string()) },
                                source: HyperlinkSource::OSC8,
                            }));
                        }
                        _ => {
                            // Empty URL terminates hyperlink
                            self.current_hyperlink = None;
                        }
                    }
                }
            }
            // ... other OSC sequences
        }
    }
    
    fn print(&mut self, c: char) {
        // When writing character, attach current hyperlink
        let mut cell = Cell::new(c);
        cell.hyperlink = self.current_hyperlink.clone();
        self.grid.set_cell(self.cursor.col, self.cursor.row, cell);
        // ... rest of print logic
    }
}
```

**Widget Integration:**

```rust
// In src/ui/terminal_widget.rs

impl TerminalWidget {
    fn handle_mouse_move(&mut self, position: Point) -> Option<Message> {
        let (col, row) = self.pixel_to_grid(position);
        let cell = self.grid.get_cell(col, row);
        
        if let Some(hyperlink) = &cell.hyperlink {
            // Mouse over link
            self.hovered_link = Some(hyperlink.clone());
            return Some(Message::SetCursor(CursorIcon::Hand));
        } else {
            self.hovered_link = None;
            return Some(Message::SetCursor(CursorIcon::Text));
        }
    }
    
    fn handle_mouse_click(&mut self, position: Point, button: MouseButton, modifiers: Modifiers) {
        let (col, row) = self.pixel_to_grid(position);
        let cell = self.grid.get_cell(col, row);
        
        if let Some(hyperlink) = &cell.hyperlink {
            match button {
                MouseButton::Left if modifiers.control() || modifiers.command() => {
                    // Open link
                    self.open_url(&hyperlink.url);
                }
                MouseButton::Right => {
                    // Show context menu
                    return Some(Message::ShowLinkContextMenu {
                        url: hyperlink.url.clone(),
                        position,
                    });
                }
                _ => {}
            }
        }
    }
    
    fn open_url(&self, url: &str) {
        // Validate URL for security
        if self.is_safe_url(url) {
            // Platform-specific URL opening
            #[cfg(target_os = "linux")]
            let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            
            #[cfg(target_os = "macos")]
            let _ = std::process::Command::new("open").arg(url).spawn();
            
            #[cfg(target_os = "windows")]
            let _ = std::process::Command::new("cmd")
                .args(&["/C", "start", url])
                .spawn();
        }
    }
    
    fn is_safe_url(&self, url: &str) -> bool {
        // Basic URL validation
        url.starts_with("http://") ||
        url.starts_with("https://") ||
        url.starts_with("file://")
    }
    
    fn draw_hyperlinks(&self, frame: &mut Frame, bounds: Rectangle) {
        // Draw underlines for hovered links
        if let Some(link) = &self.hovered_link {
            // Find all cells with this link
            for row in 0..self.grid.rows() {
                for col in 0..self.grid.cols() {
                    let cell = self.grid.get_cell(col, row);
                    if cell.hyperlink.as_ref() == Some(link) {
                        let cell_bounds = self.cell_bounds(col, row);
                        // Draw underline
                        frame.stroke_line(
                            Point::new(cell_bounds.x, cell_bounds.y + cell_bounds.height - 1.0),
                            Point::new(cell_bounds.x + cell_bounds.width, cell_bounds.y + cell_bounds.height - 1.0),
                            Stroke::default()
                                .with_color(self.theme.link_color)
                                .with_width(1.0),
                        );
                    }
                }
            }
        }
    }
}
```

**Context Menu:**

```rust
#[derive(Clone, Debug)]
pub enum ContextMenuAction {
    CopyLink,
    OpenLink,
}

pub struct LinkContextMenu {
    url: String,
    position: Point,
}

impl LinkContextMenu {
    pub fn view(&self) -> Element<Message> {
        column![
            button("Open Link")
                .on_press(Message::OpenLink(self.url.clone())),
            button("Copy Link")
                .on_press(Message::CopyLink(self.url.clone())),
        ]
        .padding(5)
        .into()
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Cell Hyperlink Support (Week 37, 1 day)**
- [ ] Extend Cell structure
- [ ] Update Grid to preserve hyperlinks
- [ ] Test: Hyperlink stored in cell

**Task 2: OSC 8 Parsing (Week 37, 2 days)**
- [ ] Implement OSC 8 dispatcher
- [ ] Track current hyperlink state
- [ ] Apply to printed characters
- [ ] Test: OSC 8 sequences parsed

**Task 3: URL Auto-Detection (Week 37, 2 days)**
- [ ] Implement regex-based URL detection
- [ ] Scan lines for URLs
- [ ] Store as auto-detected hyperlinks
- [ ] Configurable patterns
- [ ] Test: Common URLs detected

**Task 4: Hover Effects (Week 38, 2 days)**
- [ ] Detect mouse hover over hyperlink cells
- [ ] Change cursor icon
- [ ] Draw underline
- [ ] Test: Hover works

**Task 5: Click Handling (Week 38, 1 day)**
- [ ] Handle Ctrl+Click (Cmd+Click on macOS)
- [ ] Open URL with platform-specific command
- [ ] Test: Links open in browser

**Task 6: Context Menu (Week 38, 2 days)**
- [ ] Show menu on right-click
- [ ] Implement Copy Link action
- [ ] Implement Open Link action
- [ ] Test: Context menu works

**Task 7: Configuration (Week 38, 1 day)**
- [ ] Add hyperlink config to Config
- [ ] Enable/disable auto-detection
- [ ] Custom URL patterns
- [ ] Test: Configuration respected

#### Dependencies

- **Required:** Custom Terminal Widget
- **Required:** Mouse event handling
- **Optional:** Context menu widget

#### Rust Crates

- `regex` = "1.10"
- `once_cell` = "1.19"
- `url` = "2.5" (for validation)

#### Testing Strategy

**Unit Tests:**
- [ ] OSC 8 sequence parsing
- [ ] URL regex patterns
- [ ] URL validation

**Integration Tests:**
- [ ] Links stored in grid
- [ ] Multiple links in one line
- [ ] Link spanning multiple lines (wrap)
- [ ] Auto-detected vs OSC 8 links

**Manual Tests:**
- [ ] Hover effect
- [ ] Click to open
- [ ] Context menu
- [ ] Various URL types

#### Acceptance Criteria Summary

- [ ] OSC 8 sequences parsed
- [ ] URLs auto-detected
- [ ] Hover shows underline
- [ ] Cursor changes on hover
- [ ] Click opens link
- [ ] Context menu works
- [ ] Configuration options
- [ ] Security validation
- [ ] All tests passing

---

### US-043: Search in Buffer

**Priority:** MEDIUM  
**Effort:** M (1 week)  
**Complexity:** Medium  
**Dependencies:** Custom Terminal Widget, Grid

#### As a terminal user
**I want** to search for text in terminal output  
**So that** I can quickly find specific information

#### Acceptance Criteria

- [ ] Search overlay/dialog (Ctrl+Shift+F)
- [ ] Incremental search with live highlighting
- [ ] Case-sensitive toggle
- [ ] Case-insensitive mode
- [ ] Regular expression support (optional)
- [ ] Navigate results: Next (F3 or Enter), Previous (Shift+F3 or Shift+Enter)
- [ ] Visual indicators: match count, current match highlighted differently
- [ ] Search in scrollback buffer
- [ ] Close search (Escape)
- [ ] Search UI: floating overlay or bottom bar

#### Technical Design

**Search Engine:**

```rust
// src/terminal/search.rs

use regex::Regex;

pub struct SearchEngine {
    query: String,
    case_sensitive: bool,
    use_regex: bool,
    matches: Vec<SearchMatch>,
    current_match_index: usize,
}

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            case_sensitive: false,
            use_regex: false,
            matches: Vec::new(),
            current_match_index: 0,
        }
    }
    
    pub fn set_query(&mut self, query: String, case_sensitive: bool, use_regex: bool) {
        self.query = query;
        self.case_sensitive = case_sensitive;
        self.use_regex = use_regex;
    }
    
    pub fn search(&mut self, grid: &Grid, scrollback: &VecDeque<Vec<Cell>>) -> usize {
        self.matches.clear();
        self.current_match_index = 0;
        
        if self.query.is_empty() {
            return 0;
        }
        
        if self.use_regex {
            self.search_regex(grid, scrollback)
        } else {
            self.search_plain(grid, scrollback)
        }
        
        self.matches.len()
    }
    
    fn search_plain(&mut self, grid: &Grid, scrollback: &VecDeque<Vec<Cell>>) {
        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };
        
        // Search scrollback
        for (row_offset, row) in scrollback.iter().enumerate() {
            let row_text: String = row.iter().map(|c| c.c).collect();
            let search_text = if self.case_sensitive {
                row_text
            } else {
                row_text.to_lowercase()
            };
            
            let mut start = 0;
            while let Some(pos) = search_text[start..].find(&query) {
                let actual_pos = start + pos;
                self.matches.push(SearchMatch {
                    row: row_offset,  // Negative index for scrollback
                    start_col: actual_pos,
                    end_col: actual_pos + query.len(),
                });
                start = actual_pos + 1;
            }
        }
        
        // Search visible grid
        for row in 0..grid.rows() {
            let row_text: String = (0..grid.cols())
                .map(|col| grid.get_cell(col, row).c)
                .collect();
            let search_text = if self.case_sensitive {
                row_text
            } else {
                row_text.to_lowercase()
            };
            
            let mut start = 0;
            while let Some(pos) = search_text[start..].find(&query) {
                let actual_pos = start + pos;
                self.matches.push(SearchMatch {
                    row: scrollback.len() + row,  // Offset by scrollback size
                    start_col: actual_pos,
                    end_col: actual_pos + query.len(),
                });
                start = actual_pos + 1;
            }
        }
    }
    
    fn search_regex(&mut self, grid: &Grid, scrollback: &VecDeque<Vec<Cell>>) {
        let pattern = if self.case_sensitive {
            self.query.clone()
        } else {
            format!("(?i){}", self.query)
        };
        
        let re = match Regex::new(&pattern) {
            Ok(r) => r,
            Err(_) => return,  // Invalid regex
        };
        
        // Similar to plain search, but using regex
        // ... regex matching logic
    }
    
    pub fn next_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_match_index = (self.current_match_index + 1) % self.matches.len();
        Some(&self.matches[self.current_match_index])
    }
    
    pub fn prev_match(&mut self) -> Option<&SearchMatch> {
        if self.matches.is_empty() {
            return None;
        }
        self.current_match_index = if self.current_match_index == 0 {
            self.matches.len() - 1
        } else {
            self.current_match_index - 1
        };
        Some(&self.matches[self.current_match_index])
    }
    
    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.matches.get(self.current_match_index)
    }
    
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }
    
    pub fn current_index(&self) -> usize {
        if self.matches.is_empty() {
            0
        } else {
            self.current_match_index + 1
        }
    }
}
```

**Search UI:**

```rust
// src/ui/search_overlay.rs

pub struct SearchOverlay {
    query: String,
    case_sensitive: bool,
    use_regex: bool,
    match_count: usize,
    current_match: usize,
    visible: bool,
}

impl SearchOverlay {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            case_sensitive: false,
            use_regex: false,
            match_count: 0,
            current_match: 0,
            visible: false,
        }
    }
    
    pub fn show(&mut self) {
        self.visible = true;
        self.query.clear();
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    pub fn view(&self) -> Element<Message> {
        if !self.visible {
            return container(text("")).into();
        }
        
        let search_input = text_input("Search...", &self.query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchNext)
            .width(Length::Fixed(400.0));
        
        let case_checkbox = checkbox("Case sensitive", self.case_sensitive)
            .on_toggle(Message::SearchCaseSensitiveToggled);
        
        let regex_checkbox = checkbox("Regex", self.use_regex)
            .on_toggle(Message::SearchRegexToggled);
        
        let match_info = if self.match_count > 0 {
            text(format!("{}/{}", self.current_match, self.match_count))
        } else {
            text("No matches")
        };
        
        let prev_button = button("Previous")
            .on_press(Message::SearchPrevious);
        
        let next_button = button("Next")
            .on_press(Message::SearchNext);
        
        let close_button = button("×")
            .on_press(Message::SearchClose);
        
        container(
            row![
                search_input,
                case_checkbox,
                regex_checkbox,
                match_info,
                prev_button,
                next_button,
                close_button,
            ]
            .spacing(10)
            .padding(10)
        )
        .style(SearchOverlayStyle)
        .width(Length::Fill)
        .into()
    }
}
```

**Widget Integration:**

```rust
// In src/ui/terminal_widget.rs

impl TerminalWidget {
    fn draw_search_highlights(&self, frame: &mut Frame, bounds: Rectangle) {
        if let Some(search) = &self.search_engine {
            // Draw all matches with highlight color
            for (i, match_) in search.matches.iter().enumerate() {
                let is_current = i == search.current_match_index;
                let color = if is_current {
                    self.theme.search_current_match
                } else {
                    self.theme.search_match
                };
                
                // Calculate screen position
                let (screen_row, screen_col) = self.match_to_screen(match_);
                
                if self.is_visible(screen_row) {
                    for col in match_.start_col..match_.end_col {
                        let cell_bounds = self.cell_bounds(col, screen_row);
                        frame.fill_rectangle(cell_bounds, color);
                    }
                }
            }
        }
    }
    
    fn scroll_to_match(&mut self, match_: &SearchMatch) {
        // Scroll to make match visible
        let target_row = match_.row;
        let visible_rows = self.grid.rows();
        
        if target_row < self.scroll_offset {
            self.scroll_offset = target_row;
        } else if target_row >= self.scroll_offset + visible_rows {
            self.scroll_offset = target_row - visible_rows + 1;
        }
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Search Engine (Week 38, 3 days)**
- [ ] Implement SearchEngine struct
- [ ] Plain text search
- [ ] Case-sensitive/insensitive
- [ ] Match storage and navigation
- [ ] Test: Search finds matches

**Task 2: Regex Support (Week 39, 1 day)**
- [ ] Implement regex search
- [ ] Handle invalid patterns gracefully
- [ ] Test: Regex patterns work

**Task 3: Search UI Widget (Week 39, 2 days)**
- [ ] Create SearchOverlay widget
- [ ] Text input with live update
- [ ] Checkboxes for options
- [ ] Match count display
- [ ] Navigation buttons
- [ ] Test: UI appears and responds

**Task 4: Widget Integration (Week 39, 2 days)**
- [ ] Integrate search with terminal widget
- [ ] Trigger search on Ctrl+Shift+F
- [ ] Highlight matches on canvas
- [ ] Scroll to current match
- [ ] Close on Escape
- [ ] Test: End-to-end search works

**Task 5: Performance Optimization (Week 39, 1 day)**
- [ ] Incremental search (don't restart on every keystroke)
- [ ] Cache search results
- [ ] Benchmark large scrollback
- [ ] Test: Search is responsive

#### Dependencies

- **Required:** Custom Terminal Widget
- **Required:** Grid with scrollback access

#### Rust Crates

- `regex` = "1.10"

#### Testing Strategy

**Unit Tests:**
- [ ] Plain text search
- [ ] Case sensitivity
- [ ] Regex search
- [ ] Match navigation

**Integration Tests:**
- [ ] Search in visible grid
- [ ] Search in scrollback
- [ ] Multiple matches
- [ ] No matches

**Manual Tests:**
- [ ] Incremental search
- [ ] Navigation with F3/Shift+F3
- [ ] Visual highlighting
- [ ] Scroll to match

#### Acceptance Criteria Summary

- [ ] Search overlay appears on Ctrl+Shift+F
- [ ] Incremental search works
- [ ] Case-sensitive toggle
- [ ] Regex support
- [ ] Navigate matches
- [ ] Match count displayed
- [ ] Current match highlighted differently
- [ ] Search in scrollback
- [ ] Close on Escape
- [ ] All tests passing

---

### US-044: Command Palette

**Priority:** MEDIUM  
**Effort:** L (1-2 weeks)  
**Complexity:** Medium  
**Dependencies:** Custom Terminal Widget

#### As a terminal user
**I want** a fuzzy-search command launcher  
**So that** I can quickly access features without memorizing shortcuts

#### Acceptance Criteria

- [ ] Command palette opens with Ctrl+Shift+P
- [ ] Fuzzy search through all available commands
- [ ] Commands include:
  - Tab operations: New Tab, Close Tab, Next Tab, Previous Tab
  - Pane operations: Split Horizontal, Split Vertical, Close Pane
  - Theme selection: Change Theme: [theme name]
  - Font size: Increase Font Size, Decrease Font Size, Reset Font Size
  - Terminal: Clear Screen, Clear Scrollback
  - Clipboard: Copy Selection, Paste, Select All
  - Settings: Open Settings, Reload Configuration
- [ ] Shows keyboard shortcuts next to commands
- [ ] Recently used commands prioritized/at top
- [ ] Keyboard navigation (Up/Down arrows, Enter to execute, Escape to close)
- [ ] Extensible command registry for future features
- [ ] Visual design: centered floating overlay with shadow/backdrop

#### Technical Design

**Command Registry:**

```rust
// src/ui/command_palette.rs

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

#[derive(Clone, Debug)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub shortcut: Option<String>,
    pub category: CommandCategory,
    pub action: CommandAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandCategory {
    Tabs,
    Panes,
    Theme,
    Font,
    Terminal,
    Clipboard,
    Settings,
}

#[derive(Clone, Debug)]
pub enum CommandAction {
    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,
    SplitHorizontal,
    SplitVertical,
    ClosePane,
    ChangeTheme(String),
    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,
    ClearScreen,
    ClearScrollback,
    Copy,
    Paste,
    SelectAll,
    OpenSettings,
    ReloadConfig,
}

pub struct CommandRegistry {
    commands: Vec<Command>,
    recent: VecDeque<String>,  // Recent command IDs
    max_recent: usize,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
            recent: VecDeque::new(),
            max_recent: 10,
        };
        registry.register_default_commands();
        registry
    }
    
    fn register_default_commands(&mut self) {
        // Tab commands
        self.register(Command {
            id: "tab.new".into(),
            name: "New Tab".into(),
            description: Some("Open a new terminal tab".into()),
            shortcut: Some("Ctrl+Shift+T".into()),
            category: CommandCategory::Tabs,
            action: CommandAction::NewTab,
        });
        
        self.register(Command {
            id: "tab.close".into(),
            name: "Close Tab".into(),
            description: Some("Close the current tab".into()),
            shortcut: Some("Ctrl+Shift+W".into()),
            category: CommandCategory::Tabs,
            action: CommandAction::CloseTab,
        });
        
        // Pane commands
        self.register(Command {
            id: "pane.split_horizontal".into(),
            name: "Split Horizontal".into(),
            description: Some("Split pane horizontally".into()),
            shortcut: Some("Ctrl+Shift+D".into()),
            category: CommandCategory::Panes,
            action: CommandAction::SplitHorizontal,
        });
        
        self.register(Command {
            id: "pane.split_vertical".into(),
            name: "Split Vertical".into(),
            description: Some("Split pane vertically".into()),
            shortcut: Some("Ctrl+Shift+E".into()),
            category: CommandCategory::Panes,
            action: CommandAction::SplitVertical,
        });
        
        // Theme commands - dynamically generated
        // Font commands
        // Terminal commands
        // Clipboard commands
        // Settings commands
        // ... (register all commands)
    }
    
    pub fn register(&mut self, command: Command) {
        self.commands.push(command);
    }
    
    pub fn register_theme_commands(&mut self, themes: &[String]) {
        for theme in themes {
            self.register(Command {
                id: format!("theme.{}", theme),
                name: format!("Change Theme: {}", theme),
                description: Some(format!("Switch to {} theme", theme)),
                shortcut: None,
                category: CommandCategory::Theme,
                action: CommandAction::ChangeTheme(theme.clone()),
            });
        }
    }
    
    pub fn search(&self, query: &str) -> Vec<ScoredCommand> {
        if query.is_empty() {
            // Return recent commands + all commands
            return self.get_recent_commands();
        }
        
        let matcher = SkimMatcherV2::default();
        let mut results: Vec<_> = self.commands
            .iter()
            .filter_map(|cmd| {
                matcher.fuzzy_match(&cmd.name, query)
                    .map(|score| ScoredCommand {
                        command: cmd.clone(),
                        score,
                    })
            })
            .collect();
        
        // Sort by score (higher is better)
        results.sort_by(|a, b| b.score.cmp(&a.score));
        
        results
    }
    
    fn get_recent_commands(&self) -> Vec<ScoredCommand> {
        let mut results = Vec::new();
        
        // Add recent commands first
        for id in &self.recent {
            if let Some(cmd) = self.commands.iter().find(|c| c.id == *id) {
                results.push(ScoredCommand {
                    command: cmd.clone(),
                    score: 0,
                });
            }
        }
        
        // Add all other commands
        for cmd in &self.commands {
            if !self.recent.contains(&cmd.id) {
                results.push(ScoredCommand {
                    command: cmd.clone(),
                    score: 0,
                });
            }
        }
        
        results
    }
    
    pub fn record_execution(&mut self, command_id: &str) {
        // Add to recent, remove oldest if at max
        self.recent.retain(|id| id != command_id);
        self.recent.push_front(command_id.to_string());
        if self.recent.len() > self.max_recent {
            self.recent.pop_back();
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScoredCommand {
    pub command: Command,
    pub score: i64,
}
```

**Palette UI:**

```rust
pub struct CommandPalette {
    registry: CommandRegistry,
    query: String,
    results: Vec<ScoredCommand>,
    selected_index: usize,
    visible: bool,
}

impl CommandPalette {
    pub fn new(registry: CommandRegistry) -> Self {
        Self {
            registry,
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            visible: false,
        }
    }
    
    pub fn show(&mut self) {
        self.visible = true;
        self.query.clear();
        self.update_results();
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    pub fn update_query(&mut self, query: String) {
        self.query = query;
        self.selected_index = 0;
        self.update_results();
    }
    
    fn update_results(&mut self) {
        self.results = self.registry.search(&self.query);
    }
    
    pub fn select_previous(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.results.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
    
    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.results.len();
        }
    }
    
    pub fn execute_selected(&mut self) -> Option<CommandAction> {
        if let Some(scored) = self.results.get(self.selected_index) {
            self.registry.record_execution(&scored.command.id);
            Some(scored.command.action.clone())
        } else {
            None
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        if !self.visible {
            return container(text("")).into();
        }
        
        // Input field
        let input = text_input("Type a command...", &self.query)
            .on_input(Message::CommandPaletteQueryChanged)
            .on_submit(Message::CommandPaletteExecute)
            .size(20)
            .padding(10)
            .width(Length::Fixed(600.0));
        
        // Results list
        let results_list = column(
            self.results
                .iter()
                .enumerate()
                .take(10)  // Show max 10 results
                .map(|(i, scored)| {
                    let is_selected = i == self.selected_index;
                    self.command_row(&scored.command, is_selected)
                })
                .collect()
        )
        .spacing(2);
        
        let content = column![
            input,
            container(results_list)
                .max_height(400.0)
                .style(ResultsContainerStyle),
        ]
        .spacing(0);
        
        // Centered overlay with backdrop
        container(
            container(content)
                .style(PaletteStyle)
                .width(Length::Fixed(600.0))
        )
        .center_x()
        .padding(Padding::from([100, 0, 0, 0]))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(BackdropStyle)
        .into()
    }
    
    fn command_row(&self, command: &Command, selected: bool) -> Element<Message> {
        let name_text = text(&command.name)
            .size(16);
        
        let shortcut_text = if let Some(shortcut) = &command.shortcut {
            text(shortcut)
                .size(14)
                .style(ShortcutTextStyle)
        } else {
            text("")
        };
        
        row![
            name_text,
            horizontal_space(Length::Fill),
            shortcut_text,
        ]
        .padding(8)
        .width(Length::Fill)
        .style(if selected {
            SelectedCommandRowStyle
        } else {
            CommandRowStyle
        })
        .into()
    }
}

// Keyboard handling in main app
impl Application for TermiEmuApp {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KeyPressed(key, modifiers) => {
                if self.command_palette.visible {
                    match key {
                        Key::Escape => {
                            self.command_palette.hide();
                        }
                        Key::Up => {
                            self.command_palette.select_previous();
                        }
                        Key::Down => {
                            self.command_palette.select_next();
                        }
                        Key::Enter => {
                            if let Some(action) = self.command_palette.execute_selected() {
                                self.command_palette.hide();
                                return self.handle_command_action(action);
                            }
                        }
                        _ => {}
                    }
                } else if modifiers.ctrl() && modifiers.shift() && key == Key::P {
                    self.command_palette.show();
                }
            }
            // ... other messages
        }
    }
    
    fn handle_command_action(&mut self, action: CommandAction) -> Command<Message> {
        match action {
            CommandAction::NewTab => self.new_tab(),
            CommandAction::CloseTab => self.close_tab(),
            CommandAction::SplitHorizontal => self.split_horizontal(),
            CommandAction::ChangeTheme(theme) => self.change_theme(&theme),
            CommandAction::IncreaseFontSize => self.increase_font_size(),
            // ... handle all actions
        }
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Command Registry (Week 40, 2 days)**
- [ ] Implement Command and CommandAction types
- [ ] Implement CommandRegistry
- [ ] Register default commands
- [ ] Test: Registry stores commands

**Task 2: Fuzzy Search (Week 40, 2 days)**
- [ ] Integrate fuzzy-matcher crate
- [ ] Implement search() method
- [ ] Score and sort results
- [ ] Test: Fuzzy matching works

**Task 3: Recent Commands (Week 40, 1 day)**
- [ ] Track command execution history
- [ ] Prioritize recent in results
- [ ] Test: Recent tracking works

**Task 4: Palette UI (Week 40-41, 3 days)**
- [ ] Create CommandPalette widget
- [ ] Input field with live search
- [ ] Results list with selection
- [ ] Visual styling
- [ ] Test: UI renders correctly

**Task 5: Keyboard Navigation (Week 41, 1 day)**
- [ ] Handle Up/Down arrow keys
- [ ] Handle Enter to execute
- [ ] Handle Escape to close
- [ ] Test: Navigation works

**Task 6: Command Execution (Week 41, 2 days)**
- [ ] Execute selected command
- [ ] Route to appropriate handlers
- [ ] Close palette after execution
- [ ] Test: Commands execute correctly

**Task 7: Dynamic Commands (Week 41, 1 day)**
- [ ] Generate theme commands from available themes
- [ ] Update registry when themes change
- [ ] Test: Dynamic commands appear

#### Dependencies

- **Required:** Custom Terminal Widget
- **Required:** Configuration system (for themes, settings)
- **Blocks:** None (can be done in parallel with other features)

#### Rust Crates

- `fuzzy-matcher` = "0.3"

#### Testing Strategy

**Unit Tests:**
- [ ] Command registry operations
- [ ] Fuzzy search accuracy
- [ ] Recent command tracking
- [ ] Result sorting

**Integration Tests:**
- [ ] Command execution
- [ ] Keyboard navigation
- [ ] Search query updates

**Manual Tests:**
- [ ] Fuzzy search quality
- [ ] Visual appearance
- [ ] Responsiveness

#### Acceptance Criteria Summary

- [ ] Opens with Ctrl+Shift+P
- [ ] Fuzzy search works
- [ ] All command categories present
- [ ] Keyboard shortcuts displayed
- [ ] Recent commands prioritized
- [ ] Keyboard navigation works
- [ ] Commands execute correctly
- [ ] Extensible registry
- [ ] Visual design polished
- [ ] All tests passing

---

### US-045: Performance Benchmarking UI

**Priority:** HIGH  
**Effort:** M (1 week)  
**Complexity:** Medium  
**Dependencies:** Custom Terminal Widget

#### As a developer/user
**I want** real-time performance metrics displayed  
**So that** I can monitor and diagnose performance issues

#### Acceptance Criteria

- [ ] Debug overlay toggled with Ctrl+Shift+I (or configurable)
- [ ] Metrics displayed:
  - Input latency (milliseconds)
  - Frame time / FPS
  - Grid render time
  - PTY read time
  - Memory usage (current/peak)
  - Number of dirty cells (damage tracking)
  - Glyph cache hit rate
- [ ] Visual: Semi-transparent overlay in corner (configurable position)
- [ ] Configurable: Enable/disable via config or command-line flag
- [ ] Export benchmark results to file (JSON format)
- [ ] Minimal performance impact (<0.1ms overhead)

#### Technical Design

**Metrics Collection:**

```rust
// src/performance/metrics.rs

use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct PerformanceMetrics {
    // Latency tracking
    last_input_time: Arc<AtomicU64>,
    last_display_time: Arc<AtomicU64>,
    
    // Frame time tracking
    frame_times: RingBuffer<Duration>,
    
    // Component timing
    grid_render_time: Arc<AtomicU64>,
    pty_read_time: Arc<AtomicU64>,
    
    // Memory usage
    current_memory: Arc<AtomicUsize>,
    peak_memory: Arc<AtomicUsize>,
    
    // Damage tracking stats
    dirty_cells_count: Arc<AtomicUsize>,
    total_cells: usize,
    
    // Cache stats
    cache_hits: Arc<AtomicUsize>,
    cache_misses: Arc<AtomicUsize>,
}

impl PerformanceMetrics {
    pub fn new(total_cells: usize) -> Self {
        Self {
            last_input_time: Arc::new(AtomicU64::new(0)),
            last_display_time: Arc::new(AtomicU64::new(0)),
            frame_times: RingBuffer::new(60),  // Track last 60 frames
            grid_render_time: Arc::new(AtomicU64::new(0)),
            pty_read_time: Arc::new(AtomicU64::new(0)),
            current_memory: Arc::new(AtomicUsize::new(0)),
            peak_memory: Arc::new(AtomicUsize::new(0)),
            dirty_cells_count: Arc::new(AtomicUsize::new(0)),
            total_cells,
            cache_hits: Arc::new(AtomicUsize::new(0)),
            cache_misses: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn record_input(&self) {
        let now = Instant::now().elapsed().as_nanos() as u64;
        self.last_input_time.store(now, Ordering::Relaxed);
    }
    
    pub fn record_display(&self) {
        let now = Instant::now().elapsed().as_nanos() as u64;
        self.last_display_time.store(now, Ordering::Relaxed);
    }
    
    pub fn input_latency_ms(&self) -> f64 {
        let input = self.last_input_time.load(Ordering::Relaxed);
        let display = self.last_display_time.load(Ordering::Relaxed);
        if display > input {
            (display - input) as f64 / 1_000_000.0
        } else {
            0.0
        }
    }
    
    pub fn record_frame_time(&mut self, duration: Duration) {
        self.frame_times.push(duration);
    }
    
    pub fn average_frame_time_ms(&self) -> f64 {
        let sum: Duration = self.frame_times.iter().sum();
        let count = self.frame_times.len();
        if count > 0 {
            sum.as_secs_f64() * 1000.0 / count as f64
        } else {
            0.0
        }
    }
    
    pub fn fps(&self) -> f64 {
        let avg = self.average_frame_time_ms();
        if avg > 0.0 {
            1000.0 / avg
        } else {
            0.0
        }
    }
    
    pub fn update_memory_usage(&self, current: usize) {
        self.current_memory.store(current, Ordering::Relaxed);
        let peak = self.peak_memory.load(Ordering::Relaxed);
        if current > peak {
            self.peak_memory.store(current, Ordering::Relaxed);
        }
    }
    
    pub fn memory_mb(&self) -> f64 {
        self.current_memory.load(Ordering::Relaxed) as f64 / 1_048_576.0
    }
    
    pub fn peak_memory_mb(&self) -> f64 {
        self.peak_memory.load(Ordering::Relaxed) as f64 / 1_048_576.0
    }
    
    pub fn dirty_cell_percentage(&self) -> f64 {
        let dirty = self.dirty_cells_count.load(Ordering::Relaxed);
        (dirty as f64 / self.total_cells as f64) * 100.0
    }
    
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }
    
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            input_latency_ms: self.input_latency_ms(),
            frame_time_ms: self.average_frame_time_ms(),
            fps: self.fps(),
            grid_render_ms: self.grid_render_time.load(Ordering::Relaxed) as f64 / 1_000_000.0,
            pty_read_ms: self.pty_read_time.load(Ordering::Relaxed) as f64 / 1_000_000.0,
            memory_mb: self.memory_mb(),
            peak_memory_mb: self.peak_memory_mb(),
            dirty_cell_percentage: self.dirty_cell_percentage(),
            cache_hit_rate: self.cache_hit_rate(),
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MetricsSnapshot {
    pub input_latency_ms: f64,
    pub frame_time_ms: f64,
    pub fps: f64,
    pub grid_render_ms: f64,
    pub pty_read_ms: f64,
    pub memory_mb: f64,
    pub peak_memory_mb: f64,
    pub dirty_cell_percentage: f64,
    pub cache_hit_rate: f64,
    pub timestamp: Instant,
}

struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    index: usize,
    size: usize,
}

impl<T: Copy> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            index: 0,
            size: 0,
        }
    }
    
    fn push(&mut self, value: T) {
        self.buffer[self.index] = Some(value);
        self.index = (self.index + 1) % self.buffer.len();
        if self.size < self.buffer.len() {
            self.size += 1;
        }
    }
    
    fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.buffer.iter().filter_map(|&v| v)
    }
    
    fn len(&self) -> usize {
        self.size
    }
}
```

**Debug Overlay UI:**

```rust
// src/ui/debug_overlay.rs

pub struct DebugOverlay {
    visible: bool,
    position: OverlayPosition,
    metrics: Arc<PerformanceMetrics>,
}

pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DebugOverlay {
    pub fn new(metrics: Arc<PerformanceMetrics>) -> Self {
        Self {
            visible: false,
            position: OverlayPosition::TopRight,
            metrics,
        }
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    pub fn view(&self) -> Element<Message> {
        if !self.visible {
            return container(text("")).into();
        }
        
        let snapshot = self.metrics.snapshot();
        
        let content = column![
            text(format!("Input Latency: {:.2}ms", snapshot.input_latency_ms))
                .size(14)
                .style(if snapshot.input_latency_ms < 10.0 {
                    GoodMetricStyle
                } else {
                    BadMetricStyle
                }),
            text(format!("Frame Time: {:.2}ms ({:.1} FPS)", 
                snapshot.frame_time_ms, snapshot.fps))
                .size(14)
                .style(if snapshot.frame_time_ms < 16.67 {
                    GoodMetricStyle
                } else {
                    BadMetricStyle
                }),
            text(format!("Grid Render: {:.2}ms", snapshot.grid_render_ms))
                .size(14),
            text(format!("PTY Read: {:.2}ms", snapshot.pty_read_ms))
                .size(14),
            text(format!("Memory: {:.1}MB (Peak: {:.1}MB)", 
                snapshot.memory_mb, snapshot.peak_memory_mb))
                .size(14),
            text(format!("Dirty Cells: {:.1}%", snapshot.dirty_cell_percentage))
                .size(14),
            text(format!("Cache Hit Rate: {:.1}%", snapshot.cache_hit_rate))
                .size(14)
                .style(if snapshot.cache_hit_rate > 95.0 {
                    GoodMetricStyle
                } else {
                    BadMetricStyle
                }),
        ]
        .spacing(4)
        .padding(10);
        
        let positioned = match self.position {
            OverlayPosition::TopLeft => {
                container(content)
                    .align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            }
            OverlayPosition::TopRight => {
                container(content)
                    .align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            }
            OverlayPosition::BottomLeft => {
                container(content)
                    .align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            }
            OverlayPosition::BottomRight => {
                container(content)
                    .align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
            }
        };
        
        positioned
            .width(Length::Fill)
            .height(Length::Fill)
            .style(OverlayContainerStyle)  // Semi-transparent background
            .into()
    }
    
    pub fn export_to_file(&self, path: &Path) -> Result<()> {
        let snapshot = self.metrics.snapshot();
        let json = serde_json::to_string_pretty(&snapshot)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
```

#### Implementation Sub-Tasks

**Task 1: Metrics Collection (Week 39-40, 3 days)**
- [ ] Implement PerformanceMetrics struct
- [ ] Add instrumentation points
- [ ] Atomic operations for thread safety
- [ ] Test: Metrics collected accurately

**Task 2: Debug Overlay UI (Week 40, 2 days)**
- [ ] Create DebugOverlay widget
- [ ] Format metrics display
- [ ] Color coding for good/bad metrics
- [ ] Test: UI renders correctly

**Task 3: Integration (Week 40, 2 days)**
- [ ] Integrate with terminal widget
- [ ] Toggle with Ctrl+Shift+I
- [ ] Update metrics on each frame
- [ ] Test: Overlay works in app

**Task 4: Export Functionality (Week 40, 1 day)**
- [ ] Implement export to JSON
- [ ] Command or button to trigger export
- [ ] Test: Export creates valid JSON

#### Dependencies

- **Required:** Custom Terminal Widget
- **Optional:** File I/O for export

#### Rust Crates

- `serde` = "1.0" (for JSON export)
- `serde_json` = "1.0"

#### Testing Strategy

**Unit Tests:**
- [ ] Metrics calculation accuracy
- [ ] Ring buffer operations
- [ ] Snapshot creation

**Integration Tests:**
- [ ] Metrics updated on events
- [ ] Overlay toggles correctly
- [ ] Export produces valid JSON

**Performance Tests:**
- [ ] Overhead < 0.1ms
- [ ] No memory leaks

#### Acceptance Criteria Summary

- [ ] Toggled with Ctrl+Shift+I
- [ ] All required metrics displayed
- [ ] Semi-transparent overlay
- [ ] Configurable enable/disable
- [ ] Export to JSON
- [ ] Minimal overhead
- [ ] Color-coded metrics
- [ ] All tests passing

---

### US-046: Achieve <10ms Input Latency

**Priority:** CRITICAL  
**Effort:** L (1-2 weeks)  
**Complexity:** High  
**Dependencies:** Custom Terminal Widget, PTY, Parser

#### As a terminal user
**I want** instant feedback when I type  
**So that** the terminal feels responsive and natural

#### Acceptance Criteria

- [ ] Key press to PTY write: <2ms (P95)
- [ ] PTY read to grid update: <2ms (P95)
- [ ] Grid update to display: <6ms (P95)
- [ ] Total input latency <10ms (P95 measured)
- [ ] Latency stays low under heavy output (cat large file)
- [ ] 99th percentile latency <15ms
- [ ] Benchmarking tool to measure latency with high-speed camera simulation
- [ ] Regression testing in CI

#### Technical Design

**Optimization Strategies:**

1. **Direct PTY Write (Fast Path)**
```rust
// src/terminal/input.rs

pub struct InputHandler {
    pty: Arc<Mutex<Pty>>,
    fast_path_enabled: bool,
}

impl InputHandler {
    pub fn handle_key(&mut self, key: KeyEvent) -> Duration {
        let start = Instant::now();
        
        // Fast path for printable ASCII
        if self.fast_path_enabled && is_printable_ascii(key) {
            let bytes = key_to_bytes_fast(key);
            // Direct write, no buffering
            if let Ok(mut pty) = self.pty.try_lock() {
                let _ = pty.write_all(&bytes);
            }
        } else {
            // Slow path for complex keys
            let bytes = key_to_bytes_full(key);
            if let Ok(mut pty) = self.pty.lock() {
                let _ = pty.write_all(&bytes);
            }
        }
        
        start.elapsed()
    }
}

fn is_printable_ascii(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char(c) if c.is_ascii() && !key.modifiers.ctrl())
}

fn key_to_bytes_fast(key: &KeyEvent) -> Vec<u8> {
    // Simple, fast conversion for ASCII
    match key.code {
        KeyCode::Char(c) => vec![c as u8],
        _ => unreachable!(),
    }
}
```

2. **Async Rendering Pipeline**
```rust
// Non-blocking render updates

impl TerminalWidget {
    async fn render_loop(&mut self) {
        loop {
            // Wait for render trigger
            self.render_signal.notified().await;
            
            // Quick damage check
            if self.damage_tracker.is_dirty() {
                let start = Instant::now();
                
                // Render only dirty regions
                self.render_damaged_regions();
                
                let elapsed = start.elapsed();
                self.metrics.record_frame_time(elapsed);
            }
        }
    }
}
```

3. **Optimized Parser (Hot Path)**
```rust
// Profile-guided optimization for common sequences

impl TerminalParser {
    #[inline(always)]
    pub fn parse_byte_fast(&mut self, byte: u8) {
        // Optimized for common printable ASCII
        if byte >= 0x20 && byte <= 0x7E {
            self.print(byte as char);
            return;
        }
        
        // Fall back to full parser
        self.parser.advance(&mut self, byte);
    }
}
```

4. **Cache-Friendly Data Structures**
```rust
// Improve cache locality

pub struct Grid {
    // Store cells contiguously for better cache performance
    cells: Vec<Cell>,  // Flat array, not Vec<Vec<Cell>>
    cols: usize,
    rows: usize,
}

impl Grid {
    #[inline(always)]
    pub fn cell_index(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }
    
    #[inline(always)]
    pub fn get_cell(&self, col: usize, row: usize) -> &Cell {
        &self.cells[self.cell_index(col, row)]
    }
}
```

**Latency Measurement:**

```rust
// src/performance/latency.rs

pub struct LatencyBenchmark {
    samples: Vec<Duration>,
    percentiles: LatencyPercentiles,
}

pub struct LatencyPercentiles {
    p50: Duration,
    p95: Duration,
    p99: Duration,
    max: Duration,
}

impl LatencyBenchmark {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            percentiles: LatencyPercentiles::default(),
        }
    }
    
    pub fn record(&mut self, latency: Duration) {
        self.samples.push(latency);
    }
    
    pub fn calculate_percentiles(&mut self) {
        if self.samples.is_empty() {
            return;
        }
        
        self.samples.sort();
        let len = self.samples.len();
        
        self.percentiles = LatencyPercentiles {
            p50: self.samples[len * 50 / 100],
            p95: self.samples[len * 95 / 100],
            p99: self.samples[len * 99 / 100],
            max: *self.samples.last().unwrap(),
        };
    }
    
    pub fn report(&self) -> String {
        format!(
            "Latency: P50={:.2}ms, P95={:.2}ms, P99={:.2}ms, Max={:.2}ms",
            self.percentiles.p50.as_secs_f64() * 1000.0,
            self.percentiles.p95.as_secs_f64() * 1000.0,
            self.percentiles.p99.as_secs_f64() * 1000.0,
            self.percentiles.max.as_secs_f64() * 1000.0,
        )
    }
}

// Automated latency test
pub async fn run_latency_benchmark() -> Result<LatencyBenchmark> {
    let mut benchmark = LatencyBenchmark::new();
    
    for _ in 0..1000 {
        let start = Instant::now();
        
        // Simulate key press
        send_key_event();
        
        // Wait for display update
        wait_for_display_update().await;
        
        let latency = start.elapsed();
        benchmark.record(latency);
    }
    
    benchmark.calculate_percentiles();
    Ok(benchmark)
}
```

#### Implementation Sub-Tasks

**Task 1: Profiling Setup (Week 41, 1 day)**
- [ ] Set up flamegraph / perf profiling
- [ ] Identify hot paths
- [ ] Measure baseline latency
- [ ] Test: Can profile application

**Task 2: Fast Path for Input (Week 41, 2 days)**
- [ ] Implement direct PTY write
- [ ] Optimize ASCII character handling
- [ ] Bypass unnecessary processing
- [ ] Test: Fast path < 2ms

**Task 3: Parser Optimization (Week 41-42, 3 days)**
- [ ] Inline hot functions
- [ ] Optimize common sequences
- [ ] Reduce allocations
- [ ] Test: Parser throughput improved

**Task 4: Damage Tracking Optimization (Week 42, 2 days)**
- [ ] Fine-tune dirty region tracking
- [ ] Minimize invalidation
- [ ] Test: Only dirty cells redrawn

**Task 5: Cache Optimization (Week 42, 2 days)**
- [ ] Flatten data structures
- [ ] Improve memory layout
- [ ] Benchmark cache performance
- [ ] Test: Cache hit rate high

**Task 6: Async Rendering (Week 42, 2 days)**
- [ ] Separate render thread
- [ ] Non-blocking updates
- [ ] Test: Rendering doesn't block input

**Task 7: Latency Benchmarking (Week 43, 2 days)**
- [ ] Implement latency measurement
- [ ] Automated benchmark suite
- [ ] CI integration
- [ ] Test: Measures accurately

**Task 8: Final Optimization Pass (Week 43, 2 days)**
- [ ] Profile again
- [ ] Address remaining hot spots
- [ ] Verify targets met
- [ ] Test: <10ms P95 achieved

#### Dependencies

- **Required:** All terminal core components
- **Required:** Custom Terminal Widget
- **Required:** PTY integration

#### Rust Crates

- `criterion` = "0.5" (benchmarking)
- `perf-event` = "0.4" (profiling)

#### Testing Strategy

**Benchmarks:**
- [ ] Input latency benchmark
- [ ] Parser throughput benchmark
- [ ] Render performance benchmark
- [ ] End-to-end latency test

**Profiling:**
- [ ] CPU flamegraph
- [ ] Memory profiler
- [ ] Cache profiler

**Regression Tests:**
- [ ] CI latency benchmark
- [ ] Alert on regression >10%

#### Risk Assessment

**Risk:** Performance targets not achievable
- **Mitigation:** Early profiling, iterative optimization
- **Contingency:** Adjust targets based on hardware capabilities

#### Acceptance Criteria Summary

- [ ] Input to PTY < 2ms
- [ ] PTY to grid < 2ms
- [ ] Grid to display < 6ms
- [ ] Total latency < 10ms (P95)
- [ ] P99 latency < 15ms
- [ ] Benchmarking suite complete
- [ ] Regression tests in CI
- [ ] All optimizations tested

---

### US-049: User Documentation for GUI

**Priority:** HIGH  
**Effort:** M (1 week)  
**Complexity:** Low  
**Dependencies:** All Phase 2 features complete

#### As a terminal user
**I want** comprehensive documentation  
**So that** I can learn and use all features effectively

#### Acceptance Criteria

- [ ] User guide sections:
  - Installation instructions
  - Getting started tutorial
  - Tab management (shortcuts, behavior)
  - Split panes (layouts, navigation, resizing)
  - Search functionality
  - Command palette
  - Hyperlinks
  - Configuration guide
  - Theme customization
- [ ] Keyboard shortcuts reference (comprehensive table)
- [ ] Screenshots and visual guides
- [ ] Animated GIFs for complex features
- [ ] Troubleshooting section
- [ ] FAQ
- [ ] Published as docs/ directory or GitHub Wiki

#### Implementation Sub-Tasks

**Task 1: Structure & Setup (Week 43, 1 day)**
- [ ] Create docs/ directory structure
- [ ] Set up mdbook or similar
- [ ] Create table of contents
- [ ] Test: Docs build correctly

**Task 2: Getting Started (Week 43, 1 day)**
- [ ] Installation guide
- [ ] First launch walkthrough
- [ ] Basic usage examples
- [ ] Test: New user can follow

**Task 3: Feature Documentation (Week 43-44, 3 days)**
- [ ] Tab management guide
- [ ] Split panes guide
- [ ] Search guide
- [ ] Command palette guide
- [ ] Hyperlinks guide
- [ ] Configuration guide
- [ ] Test: All features covered

**Task 4: Keyboard Shortcuts Reference (Week 44, 1 day)**
- [ ] Comprehensive shortcut table
- [ ] Organized by category
- [ ] Platform-specific notes
- [ ] Test: All shortcuts documented

**Task 5: Visual Content (Week 44, 1 day)**
- [ ] Screenshots of key features
- [ ] GIFs for complex interactions
- [ ] Diagrams for pane layouts
- [ ] Test: Images render correctly

**Task 6: Troubleshooting & FAQ (Week 44, 1 day)**
- [ ] Common issues and solutions
- [ ] FAQ answers
- [ ] Debugging tips
- [ ] Test: Covers common questions

#### Dependencies

- **Required:** All Phase 2 GUI features complete

#### Rust Crates

- None (documentation only)
- Optional: `mdbook` for static site generation

#### Acceptance Criteria Summary

- [ ] All features documented
- [ ] Keyboard shortcuts complete
- [ ] Screenshots and GIFs included
- [ ] Troubleshooting guide
- [ ] FAQ section
- [ ] Published and accessible
- [ ] Reviewed for accuracy

---

## Technical Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Iced Application                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  Application State                     │  │
│  │  ┌────────────┐ ┌────────────┐ ┌─────────────────┐   │  │
│  │  │ Tab Manager│ │Command     │ │ Search Overlay  │   │  │
│  │  │            │ │Palette     │ │                 │   │  │
│  │  └────────────┘ └────────────┘ └─────────────────┘   │  │
│  │                                                        │  │
│  │  ┌──────────────────────────────────────────────────┐ │  │
│  │  │            Pane Tree Manager                     │ │  │
│  │  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐   │ │  │
│  │  │  │ Pane 1 │ │ Pane 2 │ │ Pane 3 │ │ Pane 4 │   │ │  │
│  │  │  └────────┘ └────────┘ └────────┘ └────────┘   │ │  │
│  │  └──────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
            │                        │
            ├── Per Pane Components ─┤
            ▼                        ▼
┌────────────────────┐    ┌─────────────────────┐
│ Terminal Widget    │    │  PTY + Terminal     │
│                    │    │                     │
│ ┌────────────────┐ │    │ ┌─────────────────┐ │
│ │ Canvas Renderer│ │◄───┤ │ Terminal Grid   │ │
│ │                │ │    │ │                 │ │
│ │ • Glyph Cache  │ │    │ ├─────────────────┤ │
│ │ • Texture Atlas│ │    │ │ VTE Parser      │ │
│ │ • Damage Track │ │    │ │                 │ │
│ │ • Mouse Events │ │    │ ├─────────────────┤ │
│ └────────────────┘ │    │ │ PTY Process     │ │
│                    │    │ └─────────────────┘ │
│ ┌────────────────┐ │    └─────────────────────┘
│ │ Overlays       │ │
│ │ • Selection    │ │
│ │ • Hyperlinks   │ │
│ │ • Search       │ │
│ │ • Debug Metrics│ │
│ └────────────────┘ │
└────────────────────┘
```

### Data Flow

**Input Flow:**
```
Keyboard Event
  → Iced Event Handler
  → Active Pane Terminal Widget
  → PTY Write (Fast Path <2ms)
  → Shell Process
```

**Output Flow:**
```
Shell Output
  → PTY Read (Async)
  → VTE Parser (Optimized)
  → Grid Update (Cache-friendly)
  → Damage Tracking (Mark dirty)
  → Render Trigger
  → Canvas Redraw (GPU-accelerated)
  → Display (<6ms)
```

### Module Structure

```
src/
├── main.rs                  # Application entry point
├── lib.rs                   # Library exports
├── terminal/
│   ├── mod.rs
│   ├── grid.rs              # Grid buffer (Phase 0/1)
│   ├── cell.rs              # Cell with hyperlink support
│   ├── parser.rs            # VTE parser (optimized)
│   ├── cursor.rs            # Cursor management
│   ├── modes.rs             # Terminal modes (Phase 1)
│   ├── selection.rs         # Selection (Phase 1)
│   ├── search.rs            # NEW: Search engine
│   └── hyperlink.rs         # NEW: Hyperlink detection
├── ui/
│   ├── mod.rs
│   ├── terminal_widget.rs   # NEW: Custom canvas widget
│   ├── panes.rs             # NEW: Pane management
│   ├── search_overlay.rs    # NEW: Search UI
│   ├── command_palette.rs   # NEW: Command palette
│   ├── debug_overlay.rs     # NEW: Performance metrics UI
│   └── styles.rs            # Visual styles
├── performance/
│   ├── mod.rs
│   ├── metrics.rs           # NEW: Performance tracking
│   └── latency.rs           # NEW: Latency benchmarking
├── config/
│   ├── mod.rs               # Configuration (Phase 1)
│   └── theme.rs             # Theme system (Phase 1)
├── clipboard/
│   └── mod.rs               # Clipboard (Phase 1)
├── pty/
│   └── mod.rs               # PTY integration (Phase 0)
├── error.rs                 # Error types
└── logging.rs               # Logging setup
```

---

## Dependencies & Sequencing

### Dependency Graph

```
Phase 2 Dependencies:

[Custom Terminal Widget] (Week 29-33)
  ├─ Required: Phase 0 & 1 complete
  ├─ Blocks: All Phase 2 GUI features
  └─ Critical Path: 4 weeks

[Split Panes] (Week 34-37)
  ├─ Depends: Custom Terminal Widget
  ├─ Optional: Tab management UI
  └─ Duration: 4 weeks

[Hyperlinks] (Week 37-38)
  ├─ Depends: Custom Terminal Widget
  ├─ Depends: Mouse event handling
  └─ Duration: 2 weeks

[Search] (Week 38-39)
  ├─ Depends: Custom Terminal Widget
  ├─ Independent: Can parallel with others
  └─ Duration: 2 weeks

[Command Palette] (Week 40-41)
  ├─ Depends: Custom Terminal Widget
  ├─ Optional: All commands available
  └─ Duration: 2 weeks

[Performance Metrics] (Week 39-40)
  ├─ Depends: Custom Terminal Widget
  ├─ Independent: Can parallel with others
  └─ Duration: 1.5 weeks

[Latency Optimization] (Week 41-43)
  ├─ Depends: Custom Terminal Widget
  ├─ Depends: All core components
  └─ Duration: 2-3 weeks

[Documentation] (Week 43-44)
  ├─ Depends: All features complete
  └─ Duration: 1 week
```

### Parallel Work Streams

**Stream 1: Foundation (Weeks 29-33)**
- Custom Terminal Widget (CRITICAL PATH)
- No parallel work possible

**Stream 2: Core Features (Weeks 34-39)**
- Split Panes (Weeks 34-37) - Dev 1
- Hyperlinks (Weeks 37-38) - Dev 2
- Search (Weeks 38-39) - Dev 2
- Performance Metrics (Weeks 39-40) - Dev 1 or 2

**Stream 3: Polish & Optimization (Weeks 40-44)**
- Command Palette (Weeks 40-41) - Dev 2
- Latency Optimization (Weeks 41-43) - Dev 1
- Documentation (Weeks 43-44) - Any dev

### Milestone Timeline

| Week | Milestone | Deliverables |
|------|-----------|-------------|
| 29 | Widget Foundation | Basic canvas rendering |
| 30 | cosmic-text Integration | Text shaping working |
| 31 | Glyph Cache | Performance optimized |
| 32 | Full Rendering | Complete grid display |
| 33 | Widget Complete ✓ | Ready for features |
| 35 | Split Panes Alpha | Basic splitting works |
| 37 | Split Panes Complete ✓ | Full feature set |
| 38 | Hyperlinks Complete ✓ | Clickable links |
| 39 | Search Complete ✓ | Search overlay working |
| 40 | Metrics UI Complete ✓ | Debug overlay |
| 41 | Command Palette Complete ✓ | Fuzzy launcher |
| 43 | <10ms Latency Achieved ✓ | Performance target met |
| 44 | Documentation Complete ✓ | User guides done |
| 44 | **Phase 2 Complete** 🎉 | Ready for Beta |

---

## Timeline & Estimates

### Overall Phase 2 Timeline

**Duration:** 16 weeks (Weeks 29-44)  
**Team Size:** 2-3 developers  
**Effort:** ~35 person-weeks

### Weekly Breakdown

| Weeks | Focus Area | Effort | Team |
|-------|-----------|--------|------|
| 29-33 | Custom Terminal Widget | XXL (20 days) | 2 devs |
| 34-37 | Split Panes | XL (15 days) | 1 dev |
| 37-38 | Hyperlinks | M (5 days) | 1 dev |
| 38-39 | Search | M (5 days) | 1 dev |
| 39-40 | Performance Metrics UI | M (7 days) | 1 dev |
| 40-41 | Command Palette | L (10 days) | 1 dev |
| 41-43 | Latency Optimization | L (12 days) | 1 dev |
| 43-44 | Documentation | M (5 days) | 1 dev |

### Effort Distribution

| Component | Effort | % of Phase 2 |
|-----------|--------|--------------|
| Custom Terminal Widget | XXL (4 weeks) | 40% |
| Split Panes | XL (2-3 weeks) | 20% |
| Latency Optimization | L (1-2 weeks) | 15% |
| Command Palette | L (1-2 weeks) | 10% |
| Search | M (1 week) | 5% |
| Hyperlinks | M (1 week) | 5% |
| Performance Metrics | M (1 week) | 3% |
| Documentation | M (1 week) | 2% |

### Critical Path

**Minimum Time to Complete:** 14 weeks (optimistic)
1. Custom Terminal Widget: 4 weeks
2. Split Panes (sequential): 3 weeks
3. Latency Optimization: 2 weeks
4. Final Polish: 1 week

**Realistic Time to Complete:** 16 weeks (planned)
- Includes buffer for debugging, testing, and integration

**Maximum Time (Worst Case):** 20 weeks
- If major blockers or rework needed

---

## Risk Assessment

### High-Priority Risks

#### Risk P2-001: Custom Widget Complexity
**Description:** Implementing custom canvas widget with cosmic-text may be more complex than anticipated  
**Probability:** Medium (40%)  
**Impact:** High (Blocks all GUI work)  
**Mitigation Strategies:**
- Start with minimal viable widget (just rectangles)
- Incremental cosmic-text integration
- Early prototyping and community consultation
- Budget extra time (4 weeks vs 2 weeks)
**Contingency Plan:**
- Fall back to simpler text rendering
- Use pre-existing terminal widget if available
- Reduce scope of custom features
**Status Indicators:**
- ✅ Green: Basic canvas renders by Week 29
- ⚠️ Yellow: Canvas working but cosmic-text issues by Week 30
- ❌ Red: No working canvas by Week 31 → Trigger contingency

---

#### Risk P2-002: Performance Targets Not Achievable
**Description:** May not reach <10ms latency or 60fps targets  
**Probability:** Medium (30%)  
**Impact:** High (Core product value)  
**Mitigation Strategies:**
- Early profiling and benchmarking
- Iterative optimization approach
- Damage tracking implementation
- Compare with Alacritty/WezTerm baselines
**Contingency Plan:**
- Adjust targets based on hardware reality
- Document hardware requirements
- Focus on "good enough" performance
**Status Indicators:**
- ✅ Green: <15ms latency by Week 35
- ⚠️ Yellow: 15-20ms latency by Week 40
- ❌ Red: >20ms latency by Week 43 → Adjust targets

---

#### Risk P2-003: Split Panes Layout Complexity
**Description:** Tree-based pane layout may be difficult to implement correctly  
**Probability:** Low (20%)  
**Impact:** High (CRITICAL feature)  
**Mitigation Strategies:**
- Prototype tree structure early
- Study existing implementations (tmux, wezterm)
- Start with simple horizontal/vertical splits
- Add complex nesting later
**Contingency Plan:**
- Simplify to grid-only layout
- Defer arbitrary nesting to Phase 3
**Status Indicators:**
- ✅ Green: Basic splits working by Week 35
- ⚠️ Yellow: Splits buggy by Week 36
- ❌ Red: No splits by Week 37 → Simplify approach

---

### Medium-Priority Risks

#### Risk P2-004: GPU Memory Constraints
**Description:** Texture atlas may exceed GPU memory on low-end hardware  
**Probability:** Low (15%)  
**Impact:** Medium  
**Mitigation:** Implement LRU eviction, monitor memory usage  
**Contingency:** Reduce atlas size, cache fewer glyphs

#### Risk P2-005: Cross-Platform Rendering Differences
**Description:** Rendering may look/perform differently on different platforms  
**Probability:** Medium (30%)  
**Impact:** Medium  
**Mitigation:** Test on all platforms early, CI visual regression tests  
**Contingency:** Platform-specific tuning

#### Risk P2-006: Iced Framework Limitations
**Description:** Iced may not support all required features  
**Probability:** Low (15%)  
**Impact:** Medium to High  
**Mitigation:** Engage with Iced community, contribute upstream if needed  
**Contingency:** Custom wgpu rendering

---

### Low-Priority Risks

#### Risk P2-007: Documentation Completeness
**Description:** Documentation may be incomplete or unclear  
**Probability:** Low (10%)  
**Impact:** Low  
**Mitigation:** User testing of docs, community feedback

#### Risk P2-008: Fuzzy Search Performance
**Description:** Command palette search may be slow with many commands  
**Probability:** Low (10%)  
**Impact:** Low  
**Mitigation:** Benchmark with 1000+ commands, optimize if needed

---

## Success Criteria

### Phase 2 Definition of Done

**Technical Criteria:**
- [ ] All Phase 1 criteria still met
- [ ] Custom terminal widget renders full grid correctly
- [ ] Split panes functional (create, navigate, resize, close)
- [ ] Hyperlinks clickable (OSC 8 + auto-detection)
- [ ] Search works (plain text + regex)
- [ ] Command palette accessible with fuzzy search
- [ ] Performance metrics overlay toggleable
- [ ] <10ms input latency achieved (P95)
- [ ] <16ms frame time / 60fps maintained
- [ ] All platforms build and run (Linux, macOS, Windows)

**Quality Criteria:**
- [ ] Code coverage >80%
- [ ] All tests passing
- [ ] No critical or high severity bugs
- [ ] Clippy warnings addressed
- [ ] Documentation complete

**User Experience Criteria:**
- [ ] Visual polish (smooth animations, consistent styling)
- [ ] Keyboard shortcuts work as documented
- [ ] Mouse interactions responsive
- [ ] Error messages helpful
- [ ] Settings accessible and intuitive

### Performance Benchmarks

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Input Latency (P95) | <10ms | High-frequency timestamp tracking |
| Frame Time (Average) | <16.67ms | Frame time ring buffer |
| Frame Rate | 60 FPS | 1000ms / average frame time |
| Memory Usage | <100MB | System allocator tracking |
| Startup Time | <200ms | Time from exec to first render |
| Glyph Cache Hit Rate | >95% | Cache statistics |

### Feature Completeness

| Feature | Requirement | Acceptance |
|---------|-------------|------------|
| Custom Widget | Renders correctly | ✓ Visual inspection |
| Split Panes | All operations work | ✓ Manual testing |
| Hyperlinks | Links open correctly | ✓ Click test |
| Search | Finds matches | ✓ Regex patterns |
| Command Palette | All commands present | ✓ Audit commands |
| Performance Metrics | Accurate display | ✓ Compare with profiler |
| Latency | <10ms P95 | ✓ Benchmark suite |
| Documentation | All features covered | ✓ Review checklist |

### Cross-Platform Verification

**Linux:**
- [ ] Builds on Ubuntu 22.04, Fedora 39, Arch (latest)
- [ ] Works on X11 and Wayland
- [ ] Clipboard integration works
- [ ] Performance targets met

**macOS:**
- [ ] Builds on macOS 13+ (Intel and Apple Silicon)
- [ ] Native clipboard works
- [ ] Metal GPU acceleration
- [ ] Performance targets met

**Windows:**
- [ ] Builds on Windows 10 1809+
- [ ] ConPTY integration works
- [ ] DirectX GPU acceleration
- [ ] Performance targets met

### Beta Release Readiness

- [ ] All Phase 2 features complete
- [ ] Performance targets achieved
- [ ] No known critical bugs
- [ ] Documentation published
- [ ] Release notes prepared
- [ ] Community announcement ready

---

## GitHub Issue Templates

### Issue Template: Custom Terminal Widget

```markdown
---
name: Custom Terminal Widget Implementation
about: Track implementation of custom Iced canvas widget for terminal rendering
title: "[Phase 2] Custom Terminal Widget with cosmic-text"
labels: phase-2, widget, critical, enhancement
assignees: ''
---

## Overview
Implement custom canvas-based terminal widget using Iced canvas API and cosmic-text for text rendering. This is the foundation for all Phase 2 GUI features.

## User Story
As a developer, I want a custom canvas-based widget for terminal rendering, so that I have full control over rendering and can achieve performance targets.

**Links:**
- Design Doc: PHASE2_IMPLEMENTATION_PLAN.md#custom-iced-terminal-widget
- Related: #XXX (list related issues)

## Acceptance Criteria
- [ ] Custom widget implements `iced::widget::canvas::Program` trait
- [ ] Renders to wgpu surface with GPU acceleration
- [ ] Integrates cosmic-text for text shaping and rendering
- [ ] Implements glyph cache with LRU eviction
- [ ] Supports texture atlas for GPU-resident glyphs
- [ ] Damage tracking to minimize redraws
- [ ] Mouse event handling (clicks, drags, hovers)
- [ ] Selection rendering with visual feedback
- [ ] Cursor rendering with configurable blink
- [ ] Handles window resize efficiently
- [ ] HiDPI/Retina display support
- [ ] <16ms frame time for full screen updates
- [ ] <2ms frame time for typical updates

## Technical Details

**New Files:**
- `src/ui/terminal_widget.rs`
- `src/ui/font_renderer.rs`
- `src/ui/glyph_cache.rs`
- `src/ui/damage_tracker.rs`

**Dependencies:**
- `iced` (canvas feature)
- `cosmic-text` = "0.12"
- `wgpu` (via iced)
- `lru` = "0.12"

## Implementation Tasks
- [ ] Task 1: Basic Canvas Widget (5 days)
- [ ] Task 2: cosmic-text Integration (5 days)
- [ ] Task 3: Glyph Cache & Texture Atlas (7 days)
- [ ] Task 4: Full Grid Rendering (5 days)
- [ ] Task 5: Damage Tracking (3 days)
- [ ] Task 6: Mouse Events (2 days)
- [ ] Task 7: Selection Rendering (2 days)
- [ ] Task 8: Cursor Rendering (2 days)
- [ ] Task 9: HiDPI Support (2 days)
- [ ] Task 10: Performance Optimization (3 days)

## Testing Requirements
- [ ] Unit tests for glyph cache
- [ ] Unit tests for damage tracking
- [ ] Integration tests for rendering
- [ ] Performance benchmarks
- [ ] Visual regression tests

## Estimated Timeline
**Effort:** XXL (3-4 weeks)  
**Start:** Week 29  
**Target Completion:** Week 33

## Risks
- Risk P2-001: Widget complexity higher than expected
- Mitigation: Incremental implementation, early prototyping

## Success Metrics
- [ ] Widget renders full grid correctly
- [ ] <16ms frame time
- [ ] Glyph cache hit rate >95%
- [ ] All tests passing

## Resources
- [Iced Canvas Documentation](https://docs.iced.rs/)
- [cosmic-text Examples](https://github.com/pop-os/cosmic-text)
- Alacritty rendering code for reference
```

---

### Issue Template: Split Panes

```markdown
---
name: Split Panes Feature
about: Implement horizontal and vertical pane splitting
title: "[Phase 2] Split Panes with Keyboard/Mouse Navigation"
labels: phase-2, panes, high-priority, enhancement
assignees: ''
---

## Overview
Implement split panes functionality allowing users to divide the terminal window into multiple independently-scrollable regions.

## User Story
As a terminal user, I want to split the terminal window into multiple panes, so that I can view and interact with multiple terminals simultaneously.

**Links:**
- Design Doc: PHASE2_IMPLEMENTATION_PLAN.md#us-034-split-panes
- Depends On: #XXX (Custom Terminal Widget)
- Blocks: None

## Acceptance Criteria
- [ ] Split horizontally (Ctrl+Shift+D)
- [ ] Split vertically (Ctrl+Shift+E)
- [ ] Close individual panes (Ctrl+Shift+W)
- [ ] Navigate between panes (Alt+Arrow keys)
- [ ] Resize panes by dragging dividers
- [ ] Resize panes with keyboard (Alt+Shift+Arrow)
- [ ] Visual focus indicator on active pane
- [ ] Each pane has independent PTY and terminal state
- [ ] Pane layouts persist in session configuration
- [ ] Minimum pane size enforced
- [ ] Works with tab system

## Technical Details

**New Files:**
- `src/ui/panes.rs`
- `src/ui/divider.rs`

**Key Components:**
- PaneTree (tree-based layout structure)
- PaneLayout (enum: Terminal | Horizontal | Vertical)
- Divider widget for resizing

## Implementation Tasks
- [ ] Task 1: Pane Tree Data Structure (3 days)
- [ ] Task 2: Split Operations (3 days)
- [ ] Task 3: Pane Rendering (4 days)
- [ ] Task 4: Divider Widget (3 days)
- [ ] Task 5: Divider Resizing (2 days)
- [ ] Task 6: Keyboard Navigation (2 days)
- [ ] Task 7: Keyboard Resizing (2 days)
- [ ] Task 8: Pane Close (2 days)
- [ ] Task 9: Session Persistence (2 days)
- [ ] Task 10: Tab Integration (2 days)

## Testing Requirements
- [ ] Unit tests for pane tree operations
- [ ] Integration tests for split/close
- [ ] Manual tests for visual feedback

## Estimated Timeline
**Effort:** XL (2-3 weeks)  
**Start:** Week 34  
**Target Completion:** Week 37

## Success Metrics
- [ ] Can create 4+ panes
- [ ] Navigation is intuitive
- [ ] Resizing is smooth
- [ ] No resource leaks

## Resources
- tmux pane implementation for reference
- WezTerm split panes architecture
```

---

### Issue Template: Performance Optimization

```markdown
---
name: Input Latency Optimization
about: Achieve <10ms P95 input latency
title: "[Phase 2] Achieve <10ms Input Latency (CRITICAL)"
labels: phase-2, performance, critical
assignees: ''
---

## Overview
Optimize the entire rendering pipeline to achieve <10ms P95 input latency from key press to screen display.

## User Story
As a terminal user, I want instant feedback when I type, so that the terminal feels responsive and natural.

**Links:**
- Design Doc: PHASE2_IMPLEMENTATION_PLAN.md#us-046-achieve-10ms-input-latency
- Depends On: All core components

## Acceptance Criteria
- [ ] Key press to PTY write: <2ms (P95)
- [ ] PTY read to grid update: <2ms (P95)
- [ ] Grid update to display: <6ms (P95)
- [ ] Total input latency <10ms (P95)
- [ ] P99 latency <15ms
- [ ] Latency stays low under heavy output
- [ ] Benchmarking tool measures accurately
- [ ] Regression tests in CI

## Technical Details

**Optimization Areas:**
1. Fast path for ASCII input
2. Parser hot path optimization
3. Damage tracking fine-tuning
4. Cache-friendly data structures
5. Async rendering pipeline

**Profiling Tools:**
- Flamegraph
- perf
- Instruments (macOS)

## Implementation Tasks
- [ ] Task 1: Profiling Setup (1 day)
- [ ] Task 2: Fast Path for Input (2 days)
- [ ] Task 3: Parser Optimization (3 days)
- [ ] Task 4: Damage Tracking Optimization (2 days)
- [ ] Task 5: Cache Optimization (2 days)
- [ ] Task 6: Async Rendering (2 days)
- [ ] Task 7: Latency Benchmarking (2 days)
- [ ] Task 8: Final Optimization Pass (2 days)

## Testing Requirements
- [ ] Input latency benchmark
- [ ] Parser throughput benchmark
- [ ] Render performance benchmark
- [ ] CI regression tests

## Estimated Timeline
**Effort:** L (1-2 weeks)  
**Start:** Week 41  
**Target Completion:** Week 43

## Success Metrics
- [ ] <10ms P95 latency achieved
- [ ] Benchmarks pass in CI
- [ ] No performance regressions

## Resources
- Alacritty latency optimizations
- Terminal latency measurement techniques
```

---

## Appendix

### Additional Considerations

#### Cross-Platform GUI Notes

**Linux:**
- Test on both X11 and Wayland
- Handle different window managers (GNOME, KDE, i3, etc.)
- Support various DPI settings
- Clipboard via `xclip` / `wl-clipboard`

**macOS:**
- Test on both Intel and Apple Silicon
- Retina display support crucial
- Use Cmd instead of Ctrl for shortcuts
- Native window decorations
- macOS-specific blur/vibrancy effects

**Windows:**
- Test on Windows 10 1809+ (ConPTY requirement)
- High DPI support
- Windows Terminal integration
- Different font rendering (DirectWrite)

---

### Scrollbar Widget (Deferred)

**Status:** Deferred from Phase 1, can be added in Phase 2 if time permits

**Effort:** S (2-3 days)  
**Priority:** Low

This is a nice-to-have feature that can be implemented after core GUI components are complete.

---

### Mouse Support Integration (Infrastructure Ready)

**Status:** Infrastructure exists (mouse modes in Phase 1), needs UI integration

**Effort:** M (3-5 days)  
**Priority:** Medium

Mouse reporting modes are implemented in terminal core. Needs integration with terminal widget for event routing.

---

### Recommended Development Workflow

1. **Week 29-30:** Focus 100% on Custom Terminal Widget foundation
2. **Week 31-33:** Complete widget with all features and optimization
3. **Week 34:** Start split panes (Dev 1), others can start other features
4. **Week 35-39:** Parallel work on split panes, hyperlinks, search, metrics
5. **Week 40-43:** Command palette and performance optimization
6. **Week 43-44:** Final polish, documentation, and testing

---

### Resource Allocation Suggestion

**2-Developer Team:**
- Dev 1 (Senior): Custom widget, split panes, latency optimization
- Dev 2 (Mid/Senior): Hyperlinks, search, command palette, metrics, docs

**3-Developer Team:**
- Dev 1 (Senior): Custom widget, performance optimization
- Dev 2 (Mid/Senior): Split panes, command palette
- Dev 3 (Mid): Hyperlinks, search, metrics, docs

---

### Community Engagement

**Beta Testing:**
- Recruit beta testers starting Week 40
- Provide early access builds
- Gather feedback on performance and UX
- Prioritize critical bugs

**Documentation:**
- Write user guide iteratively
- Record GIFs/videos of features
- Create troubleshooting FAQ from beta feedback

---

## Conclusion

Phase 2 represents a significant undertaking with the custom terminal widget as the foundation. The plan is ambitious but achievable with focused effort and proper risk management.

**Key Success Factors:**
1. ✅ Start with minimal viable widget and iterate
2. ✅ Early profiling and performance monitoring
3. ✅ Parallel work streams after widget completion
4. ✅ Continuous testing and integration
5. ✅ Regular check-ins on progress and risks

**Phase 2 Completion Checklist:**
- [ ] All user stories implemented
- [ ] Performance targets met
- [ ] Cross-platform builds working
- [ ] Documentation complete
- [ ] Beta testing successful
- [ ] Ready for Phase 3 (RC) or public beta release

---

**End of Phase 2 Implementation Plan**

**Document Status:** ✅ Complete  
**Next Steps:** Begin Week 29 with Custom Terminal Widget implementation  
**Questions/Feedback:** Open GitHub issue or discussion

*Good luck building the future of terminal emulators! 🚀*
