//! PTY (Pseudo-Terminal) integration for TermiEmu
//!
//! This module provides cross-platform PTY abstraction using `portable-pty`.
//! It handles:
//! - PTY creation and configuration
//! - Process spawning (shells like bash, zsh, fish, powershell)
//! - PTY resizing on window resize
//! - Signal forwarding
//! - Process termination handling
//! - Environment variable passing
//!
//! # Example
//!
//! ```no_run
//! use termiemu::pty::{Pty, PtyConfig};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = PtyConfig::default();
//! let mut pty = Pty::new(config)?;
//! pty.spawn()?;
//! # Ok(())
//! # }
//! ```

use crate::error::TermError;
use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize, PtySystem};
use std::io::{Read, Write};
use tracing::{debug, info, trace};

/// Configuration for PTY creation
#[derive(Debug, Clone)]
pub struct PtyConfig {
    /// Number of rows (lines) in the terminal
    pub rows: u16,
    /// Number of columns (characters per line) in the terminal
    pub cols: u16,
    /// Shell command to execute (None = auto-detect)
    pub shell: Option<String>,
    /// Working directory for the shell
    pub working_directory: Option<std::path::PathBuf>,
    /// Environment variables to pass to the shell
    pub env: Vec<(String, String)>,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self { rows: 24, cols: 80, shell: None, working_directory: None, env: Vec::new() }
    }
}

/// PTY wrapper providing high-level interface
pub struct Pty {
    config: PtyConfig,
    pty_system: Box<dyn PtySystem>,
    child: Option<Box<dyn Child + Send>>,
    master: Option<Box<dyn MasterPty + Send>>,
    writer: Option<Box<dyn Write + Send>>,
}

impl Pty {
    /// Create a new PTY instance with the given configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY system cannot be initialized
    pub fn new(config: PtyConfig) -> Result<Self> {
        debug!("Creating PTY with config: rows={}, cols={}", config.rows, config.cols);

        let pty_system = native_pty_system();

        Ok(Self { config, pty_system, child: None, master: None, writer: None })
    }

    /// Spawn a shell process in the PTY
    ///
    /// This creates a PTY pair and spawns the configured shell (or auto-detected
    /// shell if none specified).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - PTY pair creation fails
    /// - Shell detection fails
    /// - Process spawning fails
    pub fn spawn(&mut self) -> Result<()> {
        info!("Spawning shell in PTY");

        // Create PTY pair
        let pty_size = PtySize {
            rows: self.config.rows,
            cols: self.config.cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = self
            .pty_system
            .openpty(pty_size)
            .map_err(|e| TermError::pty(format!("Failed to create PTY: {}", e)))?;

        debug!("PTY pair created successfully");

        // Detect or use configured shell
        let shell = self.detect_shell()?;
        info!("Using shell: {}", shell);

        // Build command
        let mut cmd = CommandBuilder::new(&shell);

        // Set working directory if specified
        if let Some(ref cwd) = self.config.working_directory {
            cmd.cwd(cwd);
            debug!("Set working directory: {:?}", cwd);
        }

        // Add environment variables
        for (key, value) in &self.config.env {
            cmd.env(key, value);
            trace!("Set environment variable: {}={}", key, value);
        }

        // Spawn the child process
        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| TermError::pty(format!("Failed to spawn shell: {}", e)))?;

        info!("Shell process spawned successfully");

        // Get writer before storing master
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| TermError::pty(format!("Failed to get writer: {}", e)))?;

        // Store the master PTY, writer and child for I/O operations
        self.master = Some(pair.master);
        self.writer = Some(writer);
        self.child = Some(child);

        Ok(())
    }

    /// Resize the PTY to the given dimensions
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY has not been spawned or resize fails
    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        debug!("Resizing PTY to {}x{}", rows, cols);

        let master =
            self.master.as_mut().ok_or_else(|| TermError::invalid_state("PTY not spawned"))?;

        let new_size = PtySize { rows, cols, pixel_width: 0, pixel_height: 0 };

        master
            .resize(new_size)
            .map_err(|e| TermError::pty(format!("Failed to resize PTY: {}", e)))?;

        self.config.rows = rows;
        self.config.cols = cols;

        debug!("PTY resized successfully");
        Ok(())
    }

    /// Read data from the PTY
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY has not been spawned or read fails
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut reader = self
            .master
            .as_mut()
            .ok_or_else(|| TermError::invalid_state("PTY not spawned"))?
            .try_clone_reader()
            .map_err(|e| TermError::pty(format!("Failed to clone reader: {}", e)))?;

        reader.read(buf).context("Failed to read from PTY")
    }

    /// Write data to the PTY
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY has not been spawned or write fails
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        let writer =
            self.writer.as_mut().ok_or_else(|| TermError::invalid_state("PTY not spawned"))?;

        writer.write_all(data).context("Failed to write to PTY")?;

        Ok(data.len())
    }

    /// Check if the child process is still alive
    ///
    /// Returns `None` if the PTY has not been spawned
    pub fn is_alive(&mut self) -> Option<bool> {
        self.child
            .as_mut()
            .map(|child| child.try_wait().map(|status| status.is_none()).unwrap_or(false))
    }

    /// Wait for the child process to exit
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY has not been spawned or wait fails
    pub fn wait(&mut self) -> Result<()> {
        let child =
            self.child.as_mut().ok_or_else(|| TermError::invalid_state("PTY not spawned"))?;

        child.wait().map_err(|e| TermError::pty(format!("Failed to wait for child: {}", e)))?;

        info!("Child process exited");
        Ok(())
    }

    /// Kill the child process
    ///
    /// # Errors
    ///
    /// Returns an error if the PTY has not been spawned or kill fails
    pub fn kill(&mut self) -> Result<()> {
        let child =
            self.child.as_mut().ok_or_else(|| TermError::invalid_state("PTY not spawned"))?;

        child.kill().map_err(|e| TermError::pty(format!("Failed to kill child: {}", e)))?;

        info!("Child process killed");
        Ok(())
    }

    /// Get the current PTY configuration
    pub fn config(&self) -> &PtyConfig {
        &self.config
    }

    /// Detect the default shell for the current platform
    ///
    /// On Unix-like systems, this checks:
    /// 1. SHELL environment variable
    /// 2. /bin/bash
    /// 3. /bin/sh
    ///
    /// On Windows, this uses:
    /// 1. COMSPEC environment variable (typically cmd.exe)
    /// 2. powershell.exe
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable shell can be found
    fn detect_shell(&self) -> Result<String> {
        // If shell is explicitly configured, use it
        if let Some(ref shell) = self.config.shell {
            return Ok(shell.clone());
        }

        // Platform-specific shell detection
        #[cfg(unix)]
        {
            // Try SHELL environment variable first
            if let Ok(shell) = std::env::var("SHELL") {
                debug!("Detected shell from SHELL env var: {}", shell);
                return Ok(shell);
            }

            // Try common shells
            let common_shells = ["/bin/bash", "/bin/zsh", "/bin/fish", "/bin/sh"];
            for shell in &common_shells {
                if std::path::Path::new(shell).exists() {
                    debug!("Using shell: {}", shell);
                    return Ok(shell.to_string());
                }
            }

            Err(TermError::pty("No suitable shell found").into())
        }

        #[cfg(windows)]
        {
            // Try COMSPEC environment variable first (typically cmd.exe)
            if let Ok(comspec) = std::env::var("COMSPEC") {
                debug!("Detected shell from COMSPEC env var: {}", comspec);
                return Ok(comspec);
            }

            // Try PowerShell
            let powershell = "powershell.exe";
            debug!("Using PowerShell: {}", powershell);
            Ok(powershell.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_config_default() {
        let config = PtyConfig::default();
        assert_eq!(config.rows, 24);
        assert_eq!(config.cols, 80);
        assert!(config.shell.is_none());
        assert!(config.working_directory.is_none());
        assert!(config.env.is_empty());
    }

    #[test]
    fn test_pty_creation() {
        let config = PtyConfig::default();
        let result = Pty::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pty_spawn_and_cleanup() {
        let config = PtyConfig::default();
        let mut pty = Pty::new(config).expect("Failed to create PTY");

        // Spawn should succeed
        let result = pty.spawn();
        assert!(result.is_ok(), "Failed to spawn PTY: {:?}", result);

        // Child should be alive
        assert!(pty.is_alive().unwrap_or(false));

        // Cleanup
        let _ = pty.kill();
    }

    #[test]
    fn test_pty_resize() {
        let config = PtyConfig::default();
        let mut pty = Pty::new(config).expect("Failed to create PTY");
        pty.spawn().expect("Failed to spawn PTY");

        // Test resize
        let result = pty.resize(40, 120);
        assert!(result.is_ok(), "Failed to resize PTY: {:?}", result);
        assert_eq!(pty.config().rows, 40);
        assert_eq!(pty.config().cols, 120);

        // Cleanup
        let _ = pty.kill();
    }

    #[test]
    fn test_shell_detection() {
        let config = PtyConfig::default();
        let pty = Pty::new(config).expect("Failed to create PTY");

        let shell = pty.detect_shell();
        assert!(shell.is_ok(), "Failed to detect shell: {:?}", shell);

        let shell_path = shell.unwrap();

        #[cfg(unix)]
        assert!(
            shell_path.contains("bash")
                || shell_path.contains("zsh")
                || shell_path.contains("fish")
                || shell_path.contains("sh")
        );

        #[cfg(windows)]
        assert!(
            shell_path.to_lowercase().contains("cmd.exe")
                || shell_path.to_lowercase().contains("powershell")
        );
    }

    #[test]
    fn test_custom_shell() {
        let mut config = PtyConfig::default();
        config.shell = Some("/bin/sh".to_string());

        let pty = Pty::new(config).expect("Failed to create PTY");
        let shell = pty.detect_shell().expect("Failed to detect shell");

        assert_eq!(shell, "/bin/sh");
    }

    #[test]
    fn test_write_to_pty() {
        let config = PtyConfig::default();
        let mut pty = Pty::new(config).expect("Failed to create PTY");
        pty.spawn().expect("Failed to spawn PTY");

        // Write a simple command
        let result = pty.write(b"echo test\n");
        assert!(result.is_ok(), "Failed to write to PTY: {:?}", result);

        // Cleanup
        let _ = pty.kill();
    }
}
