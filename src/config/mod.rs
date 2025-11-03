//! Configuration system for TermiEmu
//!
//! This module implements US-036: Configuration File System.
//!
//! Configuration is stored in TOML format at:
//! - Linux/macOS: `~/.config/termiemu/config.toml`
//! - Windows: `%APPDATA%\termiemu\config.toml`
//!
//! Features:
//! - Default configuration generated on first run
//! - TOML format for easy human editing
//! - Validation with helpful error messages
//! - Hot-reload support (future feature)

pub mod theme;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, warn};

pub use theme::Theme;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Font configuration
    pub font: FontConfig,
    
    /// Color theme name (references a theme file)
    pub theme: String,
    
    /// Terminal configuration
    pub terminal: TerminalConfig,
    
    /// Key bindings configuration
    pub keybindings: KeyBindings,
    
    /// Window configuration
    pub window: WindowConfig,
    
    /// Scrollback configuration
    pub scrollback: ScrollbackConfig,
}

/// Font configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    /// Font family name
    pub family: String,
    
    /// Font size in points
    pub size: f32,
    
    /// Enable font ligatures
    pub ligatures: bool,
    
    /// Fallback font families (in order of preference)
    #[serde(default)]
    pub fallback: Vec<String>,
}

/// Terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    /// Number of columns (0 = auto-detect from window size)
    #[serde(default)]
    pub cols: usize,
    
    /// Number of rows (0 = auto-detect from window size)
    #[serde(default)]
    pub rows: usize,
    
    /// Shell command to run (empty = auto-detect from $SHELL)
    #[serde(default)]
    pub shell: String,
    
    /// Working directory (empty = home directory)
    #[serde(default)]
    pub working_directory: String,
}

/// Key bindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    /// Copy selection (default: Ctrl+Shift+C)
    pub copy: String,
    
    /// Paste from clipboard (default: Ctrl+Shift+V)
    pub paste: String,
    
    /// New tab (default: Ctrl+Shift+T)
    pub new_tab: String,
    
    /// Close tab (default: Ctrl+Shift+W)
    pub close_tab: String,
    
    /// Next tab (default: Ctrl+Tab)
    pub next_tab: String,
    
    /// Previous tab (default: Ctrl+Shift+Tab)
    pub prev_tab: String,
    
    /// Search (default: Ctrl+Shift+F)
    pub search: String,
    
    /// Command palette (default: Ctrl+Shift+P)
    pub command_palette: String,
}

/// Window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width in pixels (0 = default)
    #[serde(default)]
    pub width: u32,
    
    /// Window height in pixels (0 = default)
    #[serde(default)]
    pub height: u32,
    
    /// Window opacity (0.0 - 1.0, 1.0 = opaque)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    
    /// Enable blur effect (platform-dependent)
    #[serde(default)]
    pub blur: bool,
    
    /// Padding in pixels
    #[serde(default = "default_padding")]
    pub padding: u32,
}

/// Scrollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollbackConfig {
    /// Maximum scrollback lines
    #[serde(default = "default_scrollback")]
    pub max_lines: usize,
    
    /// Scroll speed (lines per wheel event)
    #[serde(default = "default_scroll_speed")]
    pub scroll_speed: usize,
}

// Default value functions for serde
fn default_opacity() -> f32 {
    1.0
}

fn default_padding() -> u32 {
    4
}

fn default_scrollback() -> usize {
    10_000
}

fn default_scroll_speed() -> usize {
    3
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font: FontConfig {
                family: "JetBrains Mono".to_string(),
                size: 12.0,
                ligatures: true,
                fallback: vec![
                    "Fira Code".to_string(),
                    "Cascadia Code".to_string(),
                    "Menlo".to_string(),
                    "Monaco".to_string(),
                    "Consolas".to_string(),
                    "monospace".to_string(),
                ],
            },
            theme: "catppuccin-mocha".to_string(),
            terminal: TerminalConfig { cols: 0, rows: 0, shell: String::new(), working_directory: String::new() },
            keybindings: KeyBindings {
                copy: "Ctrl+Shift+C".to_string(),
                paste: "Ctrl+Shift+V".to_string(),
                new_tab: "Ctrl+Shift+T".to_string(),
                close_tab: "Ctrl+Shift+W".to_string(),
                next_tab: "Ctrl+Tab".to_string(),
                prev_tab: "Ctrl+Shift+Tab".to_string(),
                search: "Ctrl+Shift+F".to_string(),
                command_palette: "Ctrl+Shift+P".to_string(),
            },
            window: WindowConfig { width: 0, height: 0, opacity: 1.0, blur: false, padding: 4 },
            scrollback: ScrollbackConfig { max_lines: 10_000, scroll_speed: 3 },
        }
    }
}

impl Config {
    /// Load configuration from the default location
    ///
    /// If the configuration file doesn't exist, creates a default one.
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            info!("Loading configuration from: {:?}", config_path);
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
            
            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;
            
            debug!("Configuration loaded successfully");
            Ok(config)
        } else {
            info!("Config file not found, creating default configuration");
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to the default location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }
        
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;
        
        info!("Configuration saved to: {:?}", config_path);
        Ok(())
    }

    /// Get the path to the configuration file
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;
        
        Ok(config_dir.join("termiemu").join("config.toml"))
    }

    /// Get the path to the themes directory
    pub fn themes_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;
        
        Ok(config_dir.join("termiemu").join("themes"))
    }

    /// Validate the configuration
    ///
    /// Returns Ok(()) if valid, or an error describing what's invalid.
    pub fn validate(&self) -> Result<()> {
        // Validate font size
        if self.font.size <= 0.0 || self.font.size > 72.0 {
            anyhow::bail!("Font size must be between 0 and 72, got {}", self.font.size);
        }

        // Validate opacity
        if self.window.opacity < 0.0 || self.window.opacity > 1.0 {
            anyhow::bail!("Window opacity must be between 0.0 and 1.0, got {}", self.window.opacity);
        }

        // Validate scrollback
        if self.scrollback.max_lines == 0 {
            warn!("Scrollback max_lines is 0, scrollback will be disabled");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.font.family, "JetBrains Mono");
        assert_eq!(config.font.size, 12.0);
        assert!(config.font.ligatures);
        assert_eq!(config.theme, "catppuccin-mocha");
        assert_eq!(config.scrollback.max_lines, 10_000);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(config.font.family, deserialized.font.family);
        assert_eq!(config.theme, deserialized.theme);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        // Test invalid font size
        config.font.size = -1.0;
        assert!(config.validate().is_err());
        
        config.font.size = 100.0;
        assert!(config.validate().is_err());

        config.font.size = 12.0;
        assert!(config.validate().is_ok());

        // Test invalid opacity
        config.window.opacity = 1.5;
        assert!(config.validate().is_err());
        
        config.window.opacity = -0.1;
        assert!(config.validate().is_err());
        
        config.window.opacity = 0.9;
        assert!(config.validate().is_ok());
    }
}
