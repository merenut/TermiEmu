# Terminal Standards & Protocols Reference

**Comprehensive Guide to Terminal Emulator Standards, Protocols, and Escape Sequences**

## Table of Contents

1. [Core Terminal Standards](#core-terminal-standards)
2. [Escape Sequence Standards](#escape-sequence-standards)
3. [Color and Graphics](#color-and-graphics)
4. [Font and Text Rendering](#font-and-text-rendering)
5. [Modern Extensions](#modern-extensions)
6. [Platform-Specific APIs](#platform-specific-apis)

---

## Core Terminal Standards

### VT Series (DEC)

#### VT100 (1978)
- **Standard:** DEC proprietary, influenced by ANSI X3.64
- **Key Features:**
  - 80x24 character display
  - ANSI escape sequences
  - Cursor positioning
  - Character attributes (bold, underline, reverse)
  - Alternate character set (line drawing)
  - 132-column mode
  
**Control Sequences:**
```
ESC[{n}A    Cursor Up
ESC[{n}B    Cursor Down
ESC[{n}C    Cursor Forward
ESC[{n}D    Cursor Backward
ESC[{row};{col}H  Cursor Position
ESC[2J      Clear Screen
ESC[K       Clear Line
ESC[{n}m    Select Graphic Rendition (SGR)
```

#### VT220 (1983)
- **Improvements over VT100:**
  - Additional character sets (DEC Technical, ISO Latin-1)
  - 8-bit control sequences
  - More function keys
  - Enhanced editing features
  - User-defined keys

**Additional Sequences:**
```
ESC[{n}@    Insert Character
ESC[{n}P    Delete Character
ESC[{n}L    Insert Line
ESC[{n}M    Delete Line
```

#### VT320 (1987) & VT420 (1990)
- Text macros
- Status line
- More character sets
- Printer support
- Enhanced keyboard

### ANSI/ISO Standards

#### ANSI X3.64 (1979) / ISO 6429 / ECMA-48
- **Official Standard:** Character sequences for terminal control
- **Scope:** Defines escape codes for:
  - Cursor movement
  - Text formatting
  - Screen manipulation
  - Device control

**Key Concepts:**
- **CSI (Control Sequence Introducer):** ESC[
- **OSC (Operating System Command):** ESC]
- **DCS (Device Control String):** ESCP
- **SGR (Select Graphic Rendition):** ESC[...m

---

## Escape Sequence Standards

### SGR (Select Graphic Rendition) - ANSI Color Codes

#### 16 Standard Colors (30-37, 40-47)
```
Foreground Colors:
30 - Black        90 - Bright Black (Gray)
31 - Red          91 - Bright Red
32 - Green        92 - Bright Green
33 - Yellow       93 - Bright Yellow
34 - Blue         94 - Bright Blue
35 - Magenta      95 - Bright Magenta
36 - Cyan         96 - Bright Cyan
37 - White        97 - Bright White

Background Colors:
40-47 (same as above)
100-107 (bright versions)
```

#### 256 Colors
```
ESC[38;5;{n}m   Set foreground color (n: 0-255)
ESC[48;5;{n}m   Set background color (n: 0-255)

Color Ranges:
0-15:   Standard colors
16-231: 6x6x6 RGB cube
232-255: Grayscale
```

#### True Color (24-bit)
```
ESC[38;2;{r};{g};{b}m   Set RGB foreground
ESC[48;2;{r};{g};{b}m   Set RGB background
```

### Text Attributes
```
0  - Reset/Normal
1  - Bold or increased intensity
2  - Faint/dim
3  - Italic
4  - Underline
5  - Slow blink
6  - Rapid blink
7  - Reverse video (swap fg/bg)
8  - Conceal/hide
9  - Crossed out/strikethrough
21 - Doubly underlined
22 - Normal intensity
23 - Not italic
24 - Not underlined
25 - Blink off
27 - Inverse off
28 - Reveal (conceal off)
29 - Not crossed out
```

### Underline Styles (Extended)
```
4:0 - No underline
4:1 - Straight underline
4:2 - Double underline
4:3 - Curly underline
4:4 - Dotted underline
4:5 - Dashed underline
```

---

## xterm Extensions

### Mouse Tracking

#### Mouse Reporting Modes
```
ESC[?9h     X10 mouse reporting
ESC[?9l     Disable X10

ESC[?1000h  VT200 mouse reporting
ESC[?1000l  Disable VT200

ESC[?1002h  Button-event tracking
ESC[?1003h  Any-event tracking
ESC[?1006h  SGR extended mouse mode
ESC[?1015h  URXVT mouse mode
```

#### Mouse Events Format (SGR Mode)
```
ESC[<{button};{x};{y}M   Mouse button press
ESC[<{button};{x};{y}m   Mouse button release
```

### Keyboard Protocol Extensions

#### Standard Function Keys
```
ESC[11~  F1
ESC[12~  F2
...
ESC[24~  F12
```

#### Kitty Keyboard Protocol
- Distinguishes all modifier combinations
- Supports Unicode keyboard events
- Disambiguates special keys

```
ESC[{unicode};{modifiers}u
```

### Alternative Screen Buffer
```
ESC[?1049h  Enable alternative screen buffer
ESC[?1049l  Restore main screen buffer
```

---

## Color and Graphics

### Sixel Graphics Protocol
- **Origin:** DEC VT200 series
- **Support:** Limited but growing
- **Format:** Compressed bitmap format using printable ASCII

```
ESC Pq "aspect ratio"; "raster attributes"
#{color};{rgb}    Define color
#{repeat count}   Repeat character
$                 Carriage return
-                 Newline
ESC\              End sixel data
```

**Terminals with Sixel Support:**
- xterm (compile flag)
- Kitty
- WezTerm  
- iTerm2
- Rio
- Ghostty
- Konsole

### Kitty Graphics Protocol
- **Most Advanced:** Full-featured image protocol
- **Features:**
  - PNG, JPEG, GIF support
  - Animation
  - Image placement (absolute, relative)
  - Alpha blending
  - Image IDs and reuse
  - Unicode placeholder support

```
ESC_G{key=value,...};{base64_data}ESC\

Keys:
a=t  - Transmit image
a=d  - Display image
f=<format> - Format (24=RGB, 32=RGBA, 100=PNG)
i=<id> - Image ID
```

**Terminals with Kitty Protocol:**
- Kitty (native)
- WezTerm
- Ghostty
- Rio (partial)

### iTerm2 Inline Images Protocol
```
ESC]1337;File=[arguments]:base64_data^G

Arguments:
name=<base64>       - Filename
size=<bytes>        - Data size
width=<N>           - Display width
height=<N>          - Display height
preserveAspectRatio=<0|1>
inline=<0|1>
```

**Terminals with iTerm2 Protocol:**
- iTerm2 (native)
- WezTerm
- Kitty
- Ghostty

### OSC 8 Hyperlinks
```
ESC]8;;{URI}ESC\{link_text}ESC]8;;ESC\

Example:
ESC]8;;https://example.com ESC\Click here ESC]8;;ESC\
```

**Universal Support:** Nearly all modern terminals

---

## Modern Extensions

### Shell Integration

#### OSC 7 - Current Directory
```
ESC]7;{URI}ESC\

Example:
ESC]7;file://hostname/path/to/directory ESC\
```

**Support:** Universal in modern terminals

#### OSC 133 - Semantic Prompt Markup
```
ESC]133;A ESC\        Start of prompt
ESC]133;B ESC\        Start of command input
ESC]133;C ESC\        Start of command output
ESC]133;D;{exit}ESC\  End of command (with exit status)
```

**Support:** iTerm2, WezTerm, Kitty, Warp, FishShell

#### OSC 1337 - Proprietary Extensions (iTerm2)
- Shell integration hooks
- Badge updates
- Notification triggers
- Custom user variables

### Terminal State Queries

#### Device Status Report (DSR)
```
ESC[6n      Report cursor position
Response: ESC[{row};{col}R

ESC[5n      Report status
Response: ESC[0n (OK) or ESC[3n (not OK)
```

#### XTVERSION
```
ESC[>q      Report terminal version
```

### Synchronized Updates
```
ESC[?2026h  Begin synchronized update
ESC[?2026l  End synchronized update
```

**Purpose:** Eliminate screen tearing during updates

---

## Font and Text Rendering

### OpenType Font Features

#### Common Features in Terminals
- **liga**: Standard ligatures (-> => !=)
- **calt**: Contextual alternates
- **dlig**: Discretionary ligatures
- **ss01-ss20**: Stylistic sets

#### Font Rendering Engines

**FreeType (Linux/Unix)**
- Hinting: autohint, interpreter
- Anti-aliasing: grayscale, LCD
- Subpixel rendering

**DirectWrite (Windows)**
- ClearType
- Natural font rendering
- Color emoji support

**Core Text (macOS)**
- Font smoothing
- LCD subpixel AA
- Emoji rendering

### Unicode Standards

#### UTF-8 Encoding
- Variable-length: 1-4 bytes
- Backward compatible with ASCII
- Self-synchronizing

#### Grapheme Clusters
- Base character + combining marks
- Emoji sequences (ZWJ, variation selectors)
- Regional indicators (flags)

#### Wide Characters (East Asian Width)
- Fullwidth: 2 cells (U+FF00-FFEF, CJK)
- Halfwidth: 1 cell (U+FF65-FFDC)
- Ambiguous: Context-dependent

---

## Platform-Specific APIs

### Windows

#### Console API
- `ReadConsoleInput` / `WriteConsole`
- `SetConsoleMode` - VT processing
- `GetConsoleScreenBufferInfo`

#### ConPTY (Console Pseudoterminal)
- Modern Windows terminal interface
- VT sequence support
- Used by Windows Terminal

### macOS

#### Terminal.app Integration
- AppleScript support
- `ttys` device files
- Core Foundation preferences

#### iTerm2 APIs
- Python API
- Custom escape sequences
- Shell integration helpers

### Linux/Unix

#### PTY (Pseudoterminal)
- Master/slave architecture
- `/dev/pts/*` device files
- `posix_openpt`, `grantpt`, `unlockpt`

#### termcap/terminfo
- Terminal capability database
- `/usr/share/terminfo/*`
- `tput` utility

---

## Input Method Editors (IME)

### Compose Key Sequences
- X11 Compose
- Custom compositions
- Dead key handling

### Platform IME Support
- **Windows:** TSF (Text Services Framework)
- **macOS:** Input Methods
- **Linux:** IBus, fcitx, SCIM

---

## Conformance Levels

### xterm Conformance Classes

1. **Base VT100**: Minimal compatibility
2. **VT220**: Common extended features
3. **VT320/420**: Advanced features
4. **xterm**: Modern extensions
5. **xterm+**: Experimental features

### Testing Conformance

**vttest** - Comprehensive VT100/VT220/VT320 test suite
- Cursor movement
- Screen features
- Character sets
- Color support

**Terminal.sexy** - Online color scheme tester
**ANSI.SYS Test** - DOS-era compatibility

---

## Protocol Evolution Timeline

| Year | Standard | Key Features |
|------|----------|--------------|
| 1978 | VT100 | ANSI sequences, cursor control |
| 1979 | ANSI X3.64 | Standardized escape codes |
| 1983 | VT220 | 8-bit mode, more charsets |
| 1987 | VT320 | Text macros, status line |
| 1991 | ISO 6429 | International standard |
| 1996 | xterm-256color | 256-color support |
| 2010s | True color | 24-bit RGB colors |
| 2014 | OSC 8 | Hyperlinks |
| 2016 | Kitty protocol | Advanced graphics |
| 2020 | Kitty keyboard | Enhanced keyboard protocol |
| 2020s | OSC 133 | Semantic prompts |

---

## Implementation Resources

### Specification Documents
- **ECMA-48**: Control Functions for Coded Character Sets
- **ISO/IEC 6429**: Control functions for character-imaging devices
- **DEC VT Series Manuals**: Historical terminal documentation
- **xterm ctlseqs.txt**: Comprehensive xterm control sequences

### Libraries
- **libvte**: GNOME terminal emulator widget
- **alacritty_terminal**: Alacritty's terminal emulation core
- **wezterm-term**: WezTerm's terminal library
- **crossterm**: Cross-platform terminal manipulation (Rust)
- **termion**: Terminal formatting (Rust)
- **blessed**: Terminal interface (Node.js)

### Testing Tools
- **vttest**: VT100/220/320 compatibility test
- **tack**: terminfo action checker
- **ttytest**: Terminal escape sequence testing
- **esctest**: xterm control sequence test

---

## Best Practices for Implementation

### Parsing Escape Sequences
1. Use state machine for robustness
2. Handle partial sequences
3. Validate parameters
4. Ignore unknown sequences gracefully

### Color Handling
1. Support all three color modes (16, 256, true color)
2. Provide fallback for unsupported modes
3. Honor COLORTERM environment variable
4. Test with `msgcat --color=test`

### Text Rendering
1. Properly handle Unicode grapheme clusters
2. Support combining characters
3. Implement proper emoji rendering
4. Handle wide/fullwidth characters correctly

### Performance
1. Batch screen updates
2. Use dirty rectangles
3. Implement synchronized updates
4. Optimize scrollback buffer

---

## References

- [ECMA-48 (ISO/IEC 6429)](https://www.ecma-international.org/publications-and-standards/standards/ecma-48/)
- [xterm Control Sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html)
- [VT100.net - Terminal Information](https://vt100.net/)
- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
- [iTerm2 Documentation](https://iterm2.com/documentation.html)
- [FreeType Documentation](https://freetype.org/freetype2/docs/)
- [Unicode Technical Reports](https://unicode.org/reports/)

---

**Last Updated:** November 2, 2025  
**Maintained by:** TermiEmu Research Project
