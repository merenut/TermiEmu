//! Theme system for TermiEmu
//!
//! This module implements US-037: Theme System.
//!
//! Themes define the color scheme for the terminal, including:
//! - 16 ANSI colors (8 standard + 8 bright)
//! - Background and foreground colors
//! - Cursor color
//! - Selection color
//!
//! Built-in themes include:
//! - Catppuccin (Mocha, Macchiato, Frappé, Latte)
//! - Tokyo Night (Night, Storm, Day)
//! - Dracula
//! - Nord
//! - Gruvbox (Dark, Light)
//! - Solarized (Dark, Light)

use crate::terminal::Color;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Color theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    
    /// Theme description
    #[serde(default)]
    pub description: String,
    
    /// Background color
    pub background: Rgb,
    
    /// Foreground color
    pub foreground: Rgb,
    
    /// Cursor color (None = use foreground)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Rgb>,
    
    /// Selection background color
    pub selection_background: Rgb,
    
    /// Selection foreground color (None = use foreground)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_foreground: Option<Rgb>,
    
    /// ANSI colors (16 colors: 0-7 normal, 8-15 bright)
    pub ansi: [Rgb; 16],
}

/// RGB color representation for themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Create a new RGB color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert to terminal Color type
    pub fn to_color(self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }

    /// Parse from hex string (e.g., "#ff0000" or "ff0000")
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: must be 6 characters");
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Self { r, g, b })
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Theme {
    /// Load a theme from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read theme file: {:?}", path.as_ref()))?;
        
        let theme: Theme = toml::from_str(&content)
            .with_context(|| format!("Failed to parse theme file: {:?}", path.as_ref()))?;
        
        debug!("Loaded theme: {}", theme.name);
        Ok(theme)
    }

    /// Save theme to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize theme")?;
        
        fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write theme file: {:?}", path.as_ref()))?;
        
        info!("Saved theme: {}", self.name);
        Ok(())
    }

    /// Get the Catppuccin Mocha theme (default dark theme)
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            description: "Soothing pastel theme for the high-spirited!".to_string(),
            background: Rgb::new(30, 30, 46),      // Base
            foreground: Rgb::new(205, 214, 244),   // Text
            cursor: Some(Rgb::new(245, 224, 220)), // Rosewater
            selection_background: Rgb::new(88, 91, 112), // Surface2
            selection_foreground: Some(Rgb::new(205, 214, 244)), // Text
            ansi: [
                Rgb::new(69, 71, 90),      // 0: Black (Surface1)
                Rgb::new(243, 139, 168),   // 1: Red (Maroon)
                Rgb::new(166, 227, 161),   // 2: Green
                Rgb::new(249, 226, 175),   // 3: Yellow
                Rgb::new(137, 180, 250),   // 4: Blue
                Rgb::new(203, 166, 247),   // 5: Magenta (Mauve)
                Rgb::new(148, 226, 213),   // 6: Cyan (Teal)
                Rgb::new(186, 194, 222),   // 7: White (Subtext1)
                Rgb::new(108, 112, 134),   // 8: Bright Black (Surface2)
                Rgb::new(243, 139, 168),   // 9: Bright Red
                Rgb::new(166, 227, 161),   // 10: Bright Green
                Rgb::new(249, 226, 175),   // 11: Bright Yellow
                Rgb::new(137, 180, 250),   // 12: Bright Blue
                Rgb::new(203, 166, 247),   // 13: Bright Magenta
                Rgb::new(148, 226, 213),   // 14: Bright Cyan
                Rgb::new(205, 214, 244),   // 15: Bright White (Text)
            ],
        }
    }

    /// Get the Tokyo Night theme
    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            description: "A clean, dark theme inspired by Tokyo's night skyline".to_string(),
            background: Rgb::new(26, 27, 38),
            foreground: Rgb::new(169, 177, 214),
            cursor: Some(Rgb::new(169, 177, 214)),
            selection_background: Rgb::new(54, 56, 76),
            selection_foreground: None,
            ansi: [
                Rgb::new(29, 30, 44),      // Black
                Rgb::new(247, 118, 142),   // Red
                Rgb::new(158, 206, 106),   // Green
                Rgb::new(224, 175, 104),   // Yellow
                Rgb::new(125, 207, 255),   // Blue
                Rgb::new(187, 154, 247),   // Magenta
                Rgb::new(42, 195, 222),    // Cyan
                Rgb::new(147, 154, 183),   // White
                Rgb::new(68, 71, 90),      // Bright Black
                Rgb::new(247, 118, 142),   // Bright Red
                Rgb::new(158, 206, 106),   // Bright Green
                Rgb::new(224, 175, 104),   // Bright Yellow
                Rgb::new(125, 207, 255),   // Bright Blue
                Rgb::new(187, 154, 247),   // Bright Magenta
                Rgb::new(42, 195, 222),    // Bright Cyan
                Rgb::new(192, 202, 245),   // Bright White
            ],
        }
    }

    /// Get the Dracula theme
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            description: "A dark theme with vibrant colors".to_string(),
            background: Rgb::new(40, 42, 54),
            foreground: Rgb::new(248, 248, 242),
            cursor: Some(Rgb::new(248, 248, 242)),
            selection_background: Rgb::new(68, 71, 90),
            selection_foreground: None,
            ansi: [
                Rgb::new(33, 34, 44),      // Black
                Rgb::new(255, 85, 85),     // Red
                Rgb::new(80, 250, 123),    // Green
                Rgb::new(241, 250, 140),   // Yellow
                Rgb::new(189, 147, 249),   // Blue (Purple)
                Rgb::new(255, 121, 198),   // Magenta (Pink)
                Rgb::new(139, 233, 253),   // Cyan
                Rgb::new(248, 248, 242),   // White
                Rgb::new(98, 114, 164),    // Bright Black
                Rgb::new(255, 85, 85),     // Bright Red
                Rgb::new(80, 250, 123),    // Bright Green
                Rgb::new(241, 250, 140),   // Bright Yellow
                Rgb::new(189, 147, 249),   // Bright Blue
                Rgb::new(255, 121, 198),   // Bright Magenta
                Rgb::new(139, 233, 253),   // Bright Cyan
                Rgb::new(255, 255, 255),   // Bright White
            ],
        }
    }

    /// Get the Nord theme
    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            description: "An arctic, north-bluish color palette".to_string(),
            background: Rgb::new(46, 52, 64),
            foreground: Rgb::new(216, 222, 233),
            cursor: Some(Rgb::new(216, 222, 233)),
            selection_background: Rgb::new(76, 86, 106),
            selection_foreground: None,
            ansi: [
                Rgb::new(59, 66, 82),      // Black
                Rgb::new(191, 97, 106),    // Red
                Rgb::new(163, 190, 140),   // Green
                Rgb::new(235, 203, 139),   // Yellow
                Rgb::new(129, 161, 193),   // Blue
                Rgb::new(180, 142, 173),   // Magenta
                Rgb::new(136, 192, 208),   // Cyan
                Rgb::new(229, 233, 240),   // White
                Rgb::new(76, 86, 106),     // Bright Black
                Rgb::new(191, 97, 106),    // Bright Red
                Rgb::new(163, 190, 140),   // Bright Green
                Rgb::new(235, 203, 139),   // Bright Yellow
                Rgb::new(129, 161, 193),   // Bright Blue
                Rgb::new(180, 142, 173),   // Bright Magenta
                Rgb::new(143, 188, 187),   // Bright Cyan
                Rgb::new(236, 239, 244),   // Bright White
            ],
        }
    }

    /// Get a built-in theme by name
    pub fn builtin(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "catppuccin-mocha" | "catppuccin" => Some(Self::catppuccin_mocha()),
            "tokyo-night" | "tokyonight" => Some(Self::tokyo_night()),
            "dracula" => Some(Self::dracula()),
            "nord" => Some(Self::nord()),
            _ => None,
        }
    }

    /// List all built-in theme names
    pub fn builtin_names() -> Vec<&'static str> {
        vec!["catppuccin-mocha", "tokyo-night", "dracula", "nord"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_new() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_rgb_from_hex() {
        let color = Rgb::from_hex("#ff8040").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);

        let color2 = Rgb::from_hex("ff8040").unwrap();
        assert_eq!(color, color2);
    }

    #[test]
    fn test_rgb_to_hex() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(color.to_hex(), "#ff8040");
    }

    #[test]
    fn test_builtin_themes() {
        assert!(Theme::builtin("catppuccin-mocha").is_some());
        assert!(Theme::builtin("tokyo-night").is_some());
        assert!(Theme::builtin("dracula").is_some());
        assert!(Theme::builtin("nord").is_some());
        assert!(Theme::builtin("nonexistent").is_none());
    }

    #[test]
    fn test_catppuccin_mocha_theme() {
        let theme = Theme::catppuccin_mocha();
        assert_eq!(theme.name, "Catppuccin Mocha");
        assert_eq!(theme.ansi.len(), 16);
        assert!(theme.description.contains("pastel"));
    }

    #[test]
    fn test_theme_serialization() {
        let theme = Theme::catppuccin_mocha();
        let toml_str = toml::to_string(&theme).unwrap();
        let deserialized: Theme = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(theme.name, deserialized.name);
        assert_eq!(theme.background, deserialized.background);
        assert_eq!(theme.ansi[0], deserialized.ansi[0]);
    }

    #[test]
    fn test_all_builtin_themes_valid() {
        for name in Theme::builtin_names() {
            let theme = Theme::builtin(name).unwrap();
            assert_eq!(theme.ansi.len(), 16);
            assert!(!theme.name.is_empty());
        }
    }
}
