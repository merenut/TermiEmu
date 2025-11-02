# Terminal Emulator Implementation Comparison

**Side-by-Side Analysis of How Different Terminals Implement the Same Features**

## Overview

This document compares implementation approaches across major terminal emulators, showing how different design philosophies and technology choices lead to varying solutions for common problems.

---

## 1. Configuration Systems

### Approach Comparison

#### Alacritty - TOML (Static)
```toml
# ~/.config/alacritty/alacritty.toml

[window]
dimensions = { columns = 120, lines = 40 }
padding = { x = 10, y = 10 }
opacity = 0.9

[font]
normal = { family = "JetBrains Mono", style = "Regular" }
size = 12.0

[colors.primary]
background = "#1e1e1e"
foreground = "#d4d4d4"

[[keyboard.bindings]]
key = "N"
mods = "Control|Shift"
action = "SpawnNewInstance"
```

**Pros:**
- Type-safe
- Clear structure
- Easy to validate
- Hot reload supported

**Cons:**
- No logic/computation
- Static values only
- Limited flexibility

---

#### WezTerm - Lua (Dynamic)
```lua
-- ~/.config/wezterm/wezterm.lua

local wezterm = require 'wezterm'
local config = {}

-- Dynamic based on OS
if wezterm.target_triple == 'x86_64-pc-windows-msvc' then
  config.default_prog = {'powershell.exe'}
else
  config.default_prog = {'/bin/zsh', '-l'}
end

-- Computed values
local function scheme_for_appearance(appearance)
  if appearance:find 'Dark' then
    return 'Builtin Solarized Dark'
  else
    return 'Builtin Solarized Light'
  end
end

config.color_scheme = scheme_for_appearance(wezterm.gui.get_appearance())

-- Event handlers
wezterm.on('update-right-status', function(window, pane)
  local date = wezterm.strftime '%Y-%m-%d %H:%M:%S'
  window:set_right_status(wezterm.format {
    { Text = '🔋 ' .. date },
  })
end)

-- Conditional keybindings
config.keys = {
  {
    key = 't',
    mods = 'CTRL|SHIFT',
    action = wezterm.action_callback(function(window, pane)
      if pane:get_foreground_process_name():match('vim') then
        -- Don't interfere with vim
        window:perform_action(wezterm.action.SendKey{ key='t', mods='CTRL' }, pane)
      else
        window:perform_action(wezterm.action.SpawnTab 'CurrentPaneDomain', pane)
      end
    end),
  },
}

return config
```

**Pros:**
- Full programming language
- Dynamic configuration
- Event handling
- Conditional logic
- Modular (require)

**Cons:**
- More complex
- Potential for errors
- Slower to parse

---

#### Hyper - JavaScript (Web-based)
```javascript
// ~/.hyper.js

module.exports = {
  config: {
    fontSize: 12,
    fontFamily: '"JetBrains Mono", monospace',
    
    // Dynamic theme
    backgroundColor: () => {
      const hour = new Date().getHours();
      return hour >= 6 && hour < 18
        ? '#fafafa'  // Day
        : '#1e1e1e'; // Night
    },
    
    // npm dependencies
    hyperCustomTouchbar: ['edit', 'tab', 'search'],
    
    // Plugin config
    hyperStatusLine: {
      footerTransparent: false,
    },
  },
  
  // Plugins (from npm)
  plugins: [
    'hyper-snazzy',
    'hyper-statusline',
    'hypercwd',
    'hyperpower',
  ],
  
  // Custom code
  onWindow: (browserWindow) => {
    browserWindow.setVibrancy('dark');
  },
  
  // Middleware
  middleware: (store) => (next) => (action) => {
    if (action.type === 'SESSION_ADD') {
      console.log('New session opened');
    }
    next(action);
  },
};
```

**Pros:**
- JavaScript ecosystem (npm)
- React components possible
- Full web APIs
- DevTools debugging
- Familiar to web devs

**Cons:**
- Electron overhead
- Performance impact
- Larger footprint

---

#### iTerm2 - GUI + plist (Hybrid)
```xml
<!-- ~/Library/Preferences/com.googlecode.iterm2.plist -->
<dict>
    <key>Default Bookmark Guid</key>
    <string>...</string>
    <key>HotkeyTermAnimationDuration</key>
    <real>0.25</real>
    <key>Custom Color Presets</key>
    <dict>
        <key>My Theme</key>
        <dict>
            <key>Background Color</key>
            <dict>
                <key>Red Component</key>
                <real>0.11764705882352941</real>
                <key>Green Component</key>
                <real>0.11764705882352941</real>
                <key>Blue Component</key>
                <real>0.11764705882352941</real>
            </dict>
        </dict>
    </dict>
</dict>
```

Plus GUI preferences and AppleScript automation:

```applescript
-- iTerm2 automation
tell application "iTerm"
    create window with default profile
    tell current session of current window
        write text "cd ~/projects && npm start"
    end tell
end tell
```

**Pros:**
- User-friendly GUI
- Scriptable (AppleScript, Python)
- Export/import profiles
- Native macOS integration

**Cons:**
- Binary format (plist)
- Platform-specific
- GUI required for some settings

---

## 2. Shell Integration Implementation

### OSC 7 - Working Directory Tracking

#### bash/zsh Integration
```bash
# Alacritty/Kitty/WezTerm approach
precmd() {
  printf '\e]7;file://%s%s\e\\' "$HOSTNAME" "$PWD"
}

# Or automatic with PROMPT_COMMAND
PROMPT_COMMAND='printf "\e]7;file://%s%s\e\\" "$HOSTNAME" "$PWD"'
```

#### iTerm2 Shell Integration
```bash
# Comprehensive shell integration
source ~/.iterm2_shell_integration.bash

# Automatically provides:
# - Working directory tracking (OSC 7)
# - Command start/end markers (OSC 133)
# - Remote host tracking
# - Command duration
# - Exit status

# Custom marks
iterm2_prompt_mark

# Badge content
iterm2_set_user_var badge "$(git symbolic-ref --short HEAD 2>/dev/null)"
```

#### WezTerm Enhanced Integration
```lua
-- wezterm.lua
local wezterm = require 'wezterm'

wezterm.on('update-status', function(window, pane)
  -- Get current working directory
  local cwd_uri = pane:get_current_working_dir()
  local cwd = ''
  if cwd_uri then
    cwd = cwd_uri.file_path
    -- Shorten home directory
    cwd = cwd:gsub(wezterm.home_dir, '~')
  end

  -- Display in status bar
  window:set_right_status(wezterm.format {
    { Text = cwd },
  })
end)

-- New panes start in same directory
config.default_cwd = wezterm.home_dir .. '/projects'
```

---

## 3. Graphics Protocol Implementation

### Displaying Images

#### Kitty Protocol (Native)
```bash
# Using kitty's icat
kitty +kitten icat image.png

# Raw protocol
printf '\e_Ga=T,f=100;%s\e\\' "$(base64 < image.png)"
printf '\e_Ga=d,i=1\e\\'

# With placement
kitty +kitten icat --place 40x20@10x5 image.png
```

#### iTerm2 Protocol
```bash
# Using imgcat
imgcat image.png

# Raw protocol
printf '\e]1337;File=inline=1:'
base64 < image.png
printf '\a'

# With size
printf '\e]1337;File=inline=1;width=50%%:'
base64 < image.png
printf '\a'
```

#### Sixel (Universal)
```bash
# Using img2sixel (libsixel)
img2sixel image.png

# With ImageMagick
convert image.png sixel:-

# Direct output
printf '\eP0;0;0q'  # Start sixel
# ... sixel data ...
printf '\e\\'       # End sixel
```

---

## 4. Mouse Tracking Implementation

### Mouse Event Handling

#### Basic Mouse Tracking (X11 Mode)
```c
// Enable X11 mouse tracking
printf("\e[?1000h");

// Parse mouse events
// Format: ESC[M<btn><x><y>
// btn: button + modifiers + 32
// x, y: position + 32 (to make printable)

unsigned char btn = input[3] - 32;
unsigned char x = input[4] - 32;
unsigned char y = input[5] - 32;

bool left_click = (btn & 3) == 0;
bool middle_click = (btn & 3) == 1;
bool right_click = (btn & 3) == 2;
bool release = (btn & 3) == 3;
```

#### SGR Extended Mouse Mode
```c
// Enable SGR mode (better for large screens)
printf("\e[?1006h");

// Format: ESC[<btn;x;y M (press) or m (release)
// Example: ESC[<0;10;5M (left click at 10,5)

// Parse: ESC[<0;10;5M
sscanf(input, "\e[<%d;%d;%d%c", &btn, &x, &y, &action);
bool press = (action == 'M');
bool release = (action == 'm');
```

#### Kitty Pixel-Perfect Mouse Tracking
```bash
# Enable Kitty keyboard protocol with mouse
printf '\e[?1000h\e[>1u'

# Extended mouse event with pixel precision
# Format: ESC[<btn;x;y;pixelX;pixelYM
```

---

## 5. Multiplexing Implementations

### Built-in Multiplexing

#### WezTerm - SSH Domains
```lua
-- wezterm.lua
config.ssh_domains = {
  {
    name = 'production',
    remote_address = 'prod.example.com',
    username = 'deploy',
    multiplexing = 'WezTerm',  # Use WezTerm's multiplexing
    
    -- Or use tmux on remote
    -- multiplexing = 'None',
  },
}

-- Connect
-- wezterm connect SSHMUX:production

-- Auto-populate from ~/.ssh/config
config.ssh_domains = wezterm.enumerate_ssh_hosts()
```

#### Kitty - Built-in Sessions
```bash
# Start Kitty with session
kitty --session session.conf

# session.conf:
# launch zsh
# launch --cwd ~/projects vim
# launch --title=logs tail -f /var/log/app.log

# Or use @ kitten
kitty @ launch --type=tab
kitty @ focus-tab --match title:vim
kitty @ send-text "Hello\n"
```

#### tmux Integration (Universal)
```bash
# All terminals work with tmux
tmux new -s dev

# But some have better integration:

# iTerm2: tmux control mode
tmux -CC new -s dev  # Native tmux integration

# WezTerm: can detect and enhance tmux
# Kitty: can interact via remote control
```

---

## 6. Ligature Rendering

### Font Shaping Approaches

#### HarfBuzz (Kitty, WezTerm)
```rust
// Rust example using harfbuzz-sys
use harfbuzz_sys::*;

let font = hb_font_create(hb_face);
let buffer = hb_buffer_create();

// Add text
hb_buffer_add_utf8(buffer, text.as_ptr(), text.len(), 0, text.len());

// Set features
let features = vec![
    hb_feature_t {
        tag: hb_tag_from_string(b"liga"),  // Ligatures
        value: 1,
        start: 0,
        end: u32::MAX,
    },
    hb_feature_t {
        tag: hb_tag_from_string(b"calt"),  // Contextual alternates
        value: 1,
        start: 0,
        end: u32::MAX,
    },
];

// Shape
hb_shape(font, buffer, features.as_ptr(), features.len());

// Get result
let glyph_count = hb_buffer_get_length(buffer);
let glyph_infos = hb_buffer_get_glyph_infos(buffer, null_mut());
let glyph_positions = hb_buffer_get_glyph_positions(buffer, null_mut());

// Render glyphs...
```

#### DirectWrite (Windows Terminal)
```cpp
// Windows Terminal approach
IDWriteFontFace* fontFace;
DWRITE_GLYPH_RUN glyphRun = {};

// Enable ligatures
IDWriteTypography* typography;
dwriteFactory->CreateTypography(&typography);

DWRITE_FONT_FEATURE feature = {
    DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES,
    1  // Enable
};
typography->AddFontFeature(feature);

// Analyze and render
IDWriteTextAnalyzer* analyzer;
analyzer->GetGlyphs(
    textString,
    textLength,
    fontFace,
    false,  // isSideways
    false,  // isRightToLeft
    scriptAnalysis,
    localeName,
    nullptr,  // numberSubstitution
    nullptr,  // features
    nullptr,  // featureRangeLengths
    0,  // featureRanges
    maxGlyphCount,
    clusterMap,
    textProps,
    glyphIndices,
    glyphProps,
    &actualGlyphCount
);
```

#### Fallback for Alacritty (No Ligatures)
```rust
// Alacritty explicitly avoids ligatures for performance
// Simple 1:1 character-to-glyph mapping
for ch in text.chars() {
    let glyph_id = font.glyph_id_for_char(ch);
    let glyph_width = font.glyph_width(glyph_id);
    
    // Render glyph
    render_glyph(glyph_id, x, y);
    x += glyph_width;
}
```

---

## 7. Tab Management

### Implementation Approaches

#### Native Tab Bar (Windows Terminal, iTerm2)
- Uses OS-native tab controls
- System theme integration
- Right-click context menus
- Drag-and-drop between windows

**iTerm2 (macOS):**
```swift
// AppKit NSTabView
let tabView = NSTabView()
tabView.tabViewType = .topTabsBezelBorder
tabView.addTabViewItem(tabItem)
```

**Windows Terminal:**
```cpp
// WinUI 3 TabView
TabView tabView;
TabViewItem newTab;
newTab.Header(L"PowerShell");
newTab.Content(termControl);
tabView.TabItems().Append(newTab);
```

#### Custom Tab Bar (Kitty, WezTerm, Alacritty + plugin)
- Full control over appearance
- Custom rendering
- Flexible layouts
- Status information

**WezTerm:**
```lua
-- Custom tab bar
wezterm.on('format-tab-title', function(tab, tabs, panes, config, hover, max_width)
  local title = tab.active_pane.title
  
  -- Add icon
  if title:match("vim") then
    title = "📝 " .. title
  elseif title:match("git") then
    title = "🔀 " .. title
  end
  
  -- Color based on state
  local bg = tab.is_active and '#2e3440' or '#3b4252'
  local fg = tab.is_active and '#d8dee9' or '#88c0d0'
  
  return {
    { Background = { Color = bg } },
    { Foreground = { Color = fg } },
    { Text = ' ' .. title .. ' ' },
  }
end)
```

**Kitty:**
```python
# Custom tab bar formatting (kitty.conf)
tab_bar_style = custom
tab_title_template = "{fmt.fg.red}{bell_symbol}{activity_symbol}{fmt.fg.tab}{title}"

# Dynamic tab titles
def on_tab_title(self, tab, current_tab):
    if tab.active_window.title.startswith("vim"):
        return "📝 " + tab.active_window.title
    return tab.active_window.title
```

---

## 8. Performance Optimizations

### Dirty Rectangle Tracking

#### Alacritty (Aggressive)
```rust
// Track minimal changed region
struct DirtyRegion {
    start_line: usize,
    end_line: usize,
}

impl Terminal {
    fn update(&mut self, new_data: &[u8]) {
        let old_hash = self.compute_hash();
        self.process_data(new_data);
        let new_hash = self.compute_hash();
        
        if old_hash != new_hash {
            // Find exact changed lines
            let dirty = self.find_changed_lines();
            self.renderer.mark_dirty(dirty);
        }
    }
}
```

#### WezTerm (Per-Cell)
```rust
// Track changed cells
struct Cell {
    content: char,
    attrs: CellAttributes,
    dirty: bool,
}

impl Surface {
    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if self.cells[y][x] != cell {
            self.cells[y][x] = cell;
            self.cells[y][x].dirty = true;
        }
    }
    
    fn render_dirty(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if cell.dirty {
                    self.renderer.render_cell(x, y, cell);
                    cell.dirty = false;
                }
            }
        }
    }
}
```

### Glyph Atlas Management

#### Texture Atlas (GPU-based terminals)
```rust
struct GlyphAtlas {
    texture: Texture2D,
    atlas_size: (u32, u32),
    // Maps glyph ID to texture coordinates
    glyph_cache: HashMap<GlyphKey, Rect>,
    // LRU for eviction
    lru: LruCache<GlyphKey, ()>,
}

impl GlyphAtlas {
    fn get_or_render(&mut self, glyph_key: GlyphKey) -> Rect {
        if let Some(rect) = self.glyph_cache.get(&glyph_key) {
            self.lru.get(&glyph_key);  // Mark as used
            return *rect;
        }
        
        // Render glyph
        let bitmap = self.font.render_glyph(glyph_key.id);
        
        // Find space in atlas
        let rect = self.allocate_space(bitmap.width, bitmap.height);
        
        // Upload to texture
        self.texture.update(rect, &bitmap.data);
        
        // Cache
        self.glyph_cache.insert(glyph_key, rect);
        self.lru.put(glyph_key, ());
        
        rect
    }
}
```

---

## 9. Scrollback Buffer

### Data Structure Comparison

#### Rope (Efficient, Complex)
```rust
// Used by: Alacritty, WezTerm
use ropey::Rope;

struct Scrollback {
    rope: Rope,
    max_lines: usize,
}

impl Scrollback {
    fn push_line(&mut self, line: &str) {
        self.rope.insert(self.rope.len_chars(), line);
        
        // Trim if too long
        let line_count = self.rope.len_lines();
        if line_count > self.max_lines {
            let to_remove = line_count - self.max_lines;
            let char_idx = self.rope.line_to_char(to_remove);
            self.rope.remove(0..char_idx);
        }
    }
    
    fn get_line(&self, line_num: usize) -> String {
        let start = self.rope.line_to_char(line_num);
        let end = self.rope.line_to_char(line_num + 1);
        self.rope.slice(start..end).to_string()
    }
}
```

#### VecDeque (Simple, Fast for Recent Lines)
```rust
// Used by: Many terminals for simplicity
use std::collections::VecDeque;

struct Scrollback {
    lines: VecDeque<String>,
    max_lines: usize,
}

impl Scrollback {
    fn push_line(&mut self, line: String) {
        self.lines.push_back(line);
        if self.lines.len() > self.max_lines {
            self.lines.pop_front();
        }
    }
    
    fn get_line(&self, line_num: usize) -> &str {
        &self.lines[line_num]
    }
}
```

#### Compressed Blocks (Memory Efficient)
```rust
// GNOME Terminal VTE approach
struct ScrollbackChunk {
    lines: Vec<String>,
    compressed: Option<Vec<u8>>,  // lz4 compressed
}

struct Scrollback {
    chunks: Vec<ScrollbackChunk>,
    chunk_size: usize,
    compression_threshold: usize,
}

impl Scrollback {
    fn compress_old_chunks(&mut self) {
        for chunk in &mut self.chunks {
            if chunk.lines.len() >= self.compression_threshold && chunk.compressed.is_none() {
                let serialized = bincode::serialize(&chunk.lines).unwrap();
                chunk.compressed = Some(lz4::block::compress(&serialized, None, false).unwrap());
                chunk.lines.clear();  // Free memory
            }
        }
    }
    
    fn get_line(&mut self, line_num: usize) -> String {
        let chunk_idx = line_num / self.chunk_size;
        let line_idx = line_num % self.chunk_size;
        
        let chunk = &mut self.chunks[chunk_idx];
        
        // Decompress if needed
        if chunk.lines.is_empty() {
            if let Some(compressed) = &chunk.compressed {
                let decompressed = lz4::block::decompress(compressed, None).unwrap();
                chunk.lines = bincode::deserialize(&decompressed).unwrap();
            }
        }
        
        chunk.lines[line_idx].clone()
    }
}
```

---

## 10. Keyboard Protocol Evolution

### Traditional vs. Kitty Protocol

#### Traditional Keyboard Handling
```
# Problem: Ambiguity

Ctrl+I = Tab = ESC[I
Ctrl+M = Enter = ESC[M
Ctrl+[ = Escape

# Modifiers limited:
Ctrl+Shift+Key → Often same as Ctrl+Key
Alt+Key → ESC followed by Key (ambiguous)
```

#### Kitty Keyboard Protocol
```
# Enable protocol
printf '\e[>1u'

# Format: ESC[{unicode};{modifiers}u

# Clear disambiguation:
Ctrl+I      → ESC[105;5u  (unicode 105 = 'i', mod 5 = Ctrl)
Tab         → ESC[9u       (unicode 9 = tab)
Ctrl+Tab    → ESC[9;5u
Shift+Tab   → ESC[9;2u
Ctrl+Shift+I → ESC[105;6u  (mod 6 = Ctrl+Shift)

# Modifiers:
1 = Shift
2 = Alt
4 = Ctrl
8 = Meta

# Combined: Add values
# Ctrl+Shift = 5, Ctrl+Alt = 6, etc.
```

**Support:**
- ⚡ Full: Kitty, WezTerm, Ghostty
- 🔶 Partial: iTerm2
- ❌ None: Most others

---

## Conclusion

Different terminals make different trade-offs:

**Configuration:**
- **Static (TOML):** Simple, safe, fast (Alacritty)
- **Dynamic (Lua):** Flexible, powerful (WezTerm)
- **Web-based (JS):** Ecosystem, familiar (Hyper)

**Graphics:**
- **Kitty Protocol:** Most complete
- **iTerm2 Protocol:** Simple, widely adopted
- **Sixel:** Legacy compatibility

**Performance:**
- **Alacritty:** Aggressive optimization, minimal features
- **Ghostty:** Zig performance, native UI
- **Kitty/WezTerm:** Feature-rich while performant

**Multiplexing:**
- **WezTerm:** Best built-in SSH
- **Kitty:** Comprehensive local
- **Others:** Rely on tmux/screen

The "best" approach depends on priorities: performance, features, flexibility, ecosystem, or platform integration.

---

**Last Updated:** November 2, 2025
