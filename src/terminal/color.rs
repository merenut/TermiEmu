//! Color representation for terminal cells
//!
//! Supports:
//! - 16 ANSI colors (8 standard + 8 bright)
//! - 256-color palette (xterm colors)
//! - 24-bit truecolor (RGB)

/// Color enum supporting named colors, indexed colors, and RGB
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    /// Named ANSI colors (16 colors)
    Named(NamedColor),
    /// Indexed 256-color palette
    Indexed(u8),
    /// 24-bit truecolor RGB
    Rgb(u8, u8, u8),
}

/// Named ANSI colors
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NamedColor {
    /// Black (ANSI 0)
    Black,
    /// Red (ANSI 1)
    Red,
    /// Green (ANSI 2)
    Green,
    /// Yellow (ANSI 3)
    Yellow,
    /// Blue (ANSI 4)
    Blue,
    /// Magenta (ANSI 5)
    Magenta,
    /// Cyan (ANSI 6)
    Cyan,
    /// White (ANSI 7)
    White,
    /// Bright black / gray (ANSI 8)
    BrightBlack,
    /// Bright red (ANSI 9)
    BrightRed,
    /// Bright green (ANSI 10)
    BrightGreen,
    /// Bright yellow (ANSI 11)
    BrightYellow,
    /// Bright blue (ANSI 12)
    BrightBlue,
    /// Bright magenta (ANSI 13)
    BrightMagenta,
    /// Bright cyan (ANSI 14)
    BrightCyan,
    /// Bright white (ANSI 15)
    BrightWhite,
    /// Default foreground color
    Foreground,
    /// Default background color
    Background,
}

impl Default for Color {
    fn default() -> Self {
        Color::Named(NamedColor::Foreground)
    }
}

impl Color {
    /// Create a color from an ANSI color code (0-15)
    pub fn from_ansi(code: u8) -> Self {
        match code {
            0 => Color::Named(NamedColor::Black),
            1 => Color::Named(NamedColor::Red),
            2 => Color::Named(NamedColor::Green),
            3 => Color::Named(NamedColor::Yellow),
            4 => Color::Named(NamedColor::Blue),
            5 => Color::Named(NamedColor::Magenta),
            6 => Color::Named(NamedColor::Cyan),
            7 => Color::Named(NamedColor::White),
            8 => Color::Named(NamedColor::BrightBlack),
            9 => Color::Named(NamedColor::BrightRed),
            10 => Color::Named(NamedColor::BrightGreen),
            11 => Color::Named(NamedColor::BrightYellow),
            12 => Color::Named(NamedColor::BrightBlue),
            13 => Color::Named(NamedColor::BrightMagenta),
            14 => Color::Named(NamedColor::BrightCyan),
            15 => Color::Named(NamedColor::BrightWhite),
            _ => Color::Indexed(code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_colors() {
        assert_eq!(Color::from_ansi(0), Color::Named(NamedColor::Black));
        assert_eq!(Color::from_ansi(1), Color::Named(NamedColor::Red));
        assert_eq!(Color::from_ansi(9), Color::Named(NamedColor::BrightRed));
    }

    #[test]
    fn test_indexed_color() {
        assert_eq!(Color::from_ansi(200), Color::Indexed(200));
    }

    #[test]
    fn test_rgb_color() {
        let color = Color::Rgb(255, 128, 0);
        matches!(color, Color::Rgb(255, 128, 0));
    }
}
