//! Terminal mode management
//!
//! This module provides terminal mode tracking and management.
//! Implements US-020: Terminal Mode Management from the roadmap.
//!
//! Supported modes:
//! - Application cursor keys (DECCKM)
//! - Application keypad (DECNKM)
//! - Auto-wrap mode (DECAWM)
//! - Origin mode (DECOM)
//! - Bracketed paste mode (2004)
//! - Mouse reporting modes (X10, VT200, SGR, URXVT)
//! - Focus reporting (1004)
//! - Alternate screen mode (1049)

use bitflags::bitflags;

bitflags! {
    /// Terminal mode flags
    ///
    /// These flags track which terminal modes are currently active.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct TerminalModes: u32 {
        /// Auto-wrap mode (DECAWM) - wrap cursor to next line at right margin
        const AUTO_WRAP = 0b0000_0001;
        
        /// Application cursor keys mode (DECCKM) - arrow keys send application sequences
        const CURSOR_KEYS_APP = 0b0000_0010;
        
        /// Origin mode (DECOM) - cursor positioning relative to scroll region
        const ORIGIN_MODE = 0b0000_0100;
        
        /// Bracketed paste mode (2004) - paste is bracketed with escape sequences
        const BRACKETED_PASTE = 0b0000_1000;
        
        /// Mouse reporting enabled (any mode)
        const MOUSE_REPORT = 0b0001_0000;
        
        /// Focus reporting mode (1004) - report focus in/out events
        const FOCUS_REPORT = 0b0010_0000;
        
        /// Alternate screen buffer active (1049)
        const ALT_SCREEN = 0b0100_0000;
        
        /// Application keypad mode (DECNKM) - numpad keys send application sequences
        const KEYPAD_APP = 0b1000_0000;
        
        /// Show cursor (DECTCEM)
        const SHOW_CURSOR = 0b0001_0000_0000;
        
        /// Line wrap mode
        const LINE_WRAP = 0b0010_0000_0000;
        
        /// Insert mode (IRM) - insert characters instead of replacing
        const INSERT_MODE = 0b0100_0000_0000;
        
        /// Mouse X10 mode
        const MOUSE_X10 = 0b1000_0000_0000;
        
        /// Mouse VT200 mode (button event tracking)
        const MOUSE_VT200 = 0b0001_0000_0000_0000;
        
        /// Mouse button event tracking
        const MOUSE_BUTTON_EVENT = 0b0010_0000_0000_0000;
        
        /// Mouse any event tracking
        const MOUSE_ANY_EVENT = 0b0100_0000_0000_0000;
        
        /// Mouse SGR extended mode
        const MOUSE_SGR = 0b1000_0000_0000_0000;
        
        /// Mouse URXVT extended mode
        const MOUSE_URXVT = 0b0001_0000_0000_0000_0000;
    }
}

impl TerminalModes {
    /// Create a new TerminalModes with default settings
    pub fn new() -> Self {
        // Default modes: auto-wrap, show cursor, line wrap enabled
        Self::AUTO_WRAP | Self::SHOW_CURSOR | Self::LINE_WRAP
    }

    /// Check if auto-wrap is enabled
    pub fn is_auto_wrap(&self) -> bool {
        self.contains(Self::AUTO_WRAP)
    }

    /// Check if application cursor keys mode is active
    pub fn is_cursor_keys_app(&self) -> bool {
        self.contains(Self::CURSOR_KEYS_APP)
    }

    /// Check if origin mode is active
    pub fn is_origin_mode(&self) -> bool {
        self.contains(Self::ORIGIN_MODE)
    }

    /// Check if bracketed paste mode is active
    pub fn is_bracketed_paste(&self) -> bool {
        self.contains(Self::BRACKETED_PASTE)
    }

    /// Check if mouse reporting is enabled
    pub fn is_mouse_report(&self) -> bool {
        self.contains(Self::MOUSE_REPORT)
    }

    /// Check if focus reporting is enabled
    pub fn is_focus_report(&self) -> bool {
        self.contains(Self::FOCUS_REPORT)
    }

    /// Check if alternate screen is active
    pub fn is_alt_screen(&self) -> bool {
        self.contains(Self::ALT_SCREEN)
    }

    /// Check if application keypad mode is active
    pub fn is_keypad_app(&self) -> bool {
        self.contains(Self::KEYPAD_APP)
    }

    /// Check if cursor is visible
    pub fn is_cursor_visible(&self) -> bool {
        self.contains(Self::SHOW_CURSOR)
    }

    /// Check if insert mode is active
    pub fn is_insert_mode(&self) -> bool {
        self.contains(Self::INSERT_MODE)
    }

    /// Get the active mouse tracking mode
    pub fn mouse_mode(&self) -> MouseMode {
        if self.contains(Self::MOUSE_SGR) {
            MouseMode::Sgr
        } else if self.contains(Self::MOUSE_URXVT) {
            MouseMode::Urxvt
        } else if self.contains(Self::MOUSE_ANY_EVENT) {
            MouseMode::AnyEvent
        } else if self.contains(Self::MOUSE_BUTTON_EVENT) {
            MouseMode::ButtonEvent
        } else if self.contains(Self::MOUSE_VT200) {
            MouseMode::Vt200
        } else if self.contains(Self::MOUSE_X10) {
            MouseMode::X10
        } else {
            MouseMode::None
        }
    }

    /// Set mouse tracking mode
    pub fn set_mouse_mode(&mut self, mode: MouseMode) {
        // Clear all mouse mode flags
        self.remove(
            Self::MOUSE_X10
                | Self::MOUSE_VT200
                | Self::MOUSE_BUTTON_EVENT
                | Self::MOUSE_ANY_EVENT
                | Self::MOUSE_SGR
                | Self::MOUSE_URXVT,
        );

        // Set the new mode
        match mode {
            MouseMode::None => {
                self.remove(Self::MOUSE_REPORT);
            }
            MouseMode::X10 => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_X10);
            }
            MouseMode::Vt200 => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_VT200);
            }
            MouseMode::ButtonEvent => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_BUTTON_EVENT);
            }
            MouseMode::AnyEvent => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_ANY_EVENT);
            }
            MouseMode::Sgr => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_SGR);
            }
            MouseMode::Urxvt => {
                self.insert(Self::MOUSE_REPORT | Self::MOUSE_URXVT);
            }
        }
    }
}

/// Mouse tracking mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseMode {
    /// No mouse tracking
    None,
    /// X10 mouse mode (limited to 223x223)
    X10,
    /// VT200 mouse mode (button event tracking)
    Vt200,
    /// Button event tracking mode
    ButtonEvent,
    /// Any event tracking mode (motion events)
    AnyEvent,
    /// SGR extended mouse mode (unlimited coordinates)
    Sgr,
    /// URXVT extended mouse mode
    Urxvt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_modes() {
        let modes = TerminalModes::new();
        assert!(modes.is_auto_wrap());
        assert!(modes.is_cursor_visible());
        assert!(!modes.is_alt_screen());
        assert!(!modes.is_bracketed_paste());
    }

    #[test]
    fn test_mode_toggle() {
        let mut modes = TerminalModes::new();
        assert!(!modes.is_bracketed_paste());
        
        modes.insert(TerminalModes::BRACKETED_PASTE);
        assert!(modes.is_bracketed_paste());
        
        modes.remove(TerminalModes::BRACKETED_PASTE);
        assert!(!modes.is_bracketed_paste());
    }

    #[test]
    fn test_mouse_modes() {
        let mut modes = TerminalModes::new();
        assert_eq!(modes.mouse_mode(), MouseMode::None);
        
        modes.set_mouse_mode(MouseMode::Sgr);
        assert_eq!(modes.mouse_mode(), MouseMode::Sgr);
        assert!(modes.is_mouse_report());
        
        modes.set_mouse_mode(MouseMode::None);
        assert_eq!(modes.mouse_mode(), MouseMode::None);
        assert!(!modes.is_mouse_report());
    }

    #[test]
    fn test_alt_screen_mode() {
        let mut modes = TerminalModes::new();
        assert!(!modes.is_alt_screen());
        
        modes.insert(TerminalModes::ALT_SCREEN);
        assert!(modes.is_alt_screen());
    }

    #[test]
    fn test_application_cursor_keys() {
        let mut modes = TerminalModes::new();
        assert!(!modes.is_cursor_keys_app());
        
        modes.insert(TerminalModes::CURSOR_KEYS_APP);
        assert!(modes.is_cursor_keys_app());
    }
}
