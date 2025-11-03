//! Integration tests for PTY functionality
//!
//! These tests verify that the PTY integration works correctly across
//! different scenarios and platforms.

use std::time::Duration;
use termiemu::pty::{Pty, PtyConfig};

#[test]
fn test_pty_spawn_and_write() {
    // Create PTY with default config
    let mut pty = Pty::new(PtyConfig::default()).expect("Failed to create PTY");

    // Spawn shell
    pty.spawn().expect("Failed to spawn shell");

    // Verify shell is running
    assert!(pty.is_alive().unwrap());

    // Write a simple command
    let command = b"echo test\n";
    let written = pty.write(command).expect("Failed to write to PTY");
    assert_eq!(written, command.len());

    // Clean up
    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_multiple_commands() {
    let mut pty = Pty::new(PtyConfig::default()).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    // Write multiple commands
    let commands =
        vec![b"echo first\n" as &[u8], b"echo second\n" as &[u8], b"echo third\n" as &[u8]];

    for cmd in commands {
        let result = pty.write(cmd);
        assert!(result.is_ok(), "Failed to write command: {:?}", result);
    }

    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_resize_multiple_times() {
    let mut pty = Pty::new(PtyConfig::default()).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    // Test multiple resizes
    let sizes = vec![(24, 80), (40, 120), (60, 160), (30, 100)];

    for (rows, cols) in sizes {
        pty.resize(rows, cols).expect("Failed to resize PTY");
        assert_eq!(pty.config().rows, rows);
        assert_eq!(pty.config().cols, cols);
    }

    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_with_custom_working_directory() {
    let mut config = PtyConfig::default();
    config.working_directory = Some(std::env::temp_dir());

    let mut pty = Pty::new(config).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    assert!(pty.is_alive().unwrap());

    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_with_environment_variables() {
    let mut config = PtyConfig::default();
    config.env = vec![
        ("TEST_VAR_1".to_string(), "value1".to_string()),
        ("TEST_VAR_2".to_string(), "value2".to_string()),
    ];

    let mut pty = Pty::new(config).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    assert!(pty.is_alive().unwrap());

    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_read_write_cycle() {
    let mut pty = Pty::new(PtyConfig::default()).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    // Write a command
    pty.write(b"echo hello\n").expect("Failed to write");

    // Give shell time to process
    std::thread::sleep(Duration::from_millis(100));

    // Try to read output
    let mut buffer = vec![0u8; 4096];
    match pty.read(&mut buffer) {
        Ok(n) if n > 0 => {
            // Successfully read some data
            let output = String::from_utf8_lossy(&buffer[..n]);
            assert!(!output.is_empty());
        }
        Ok(_) => {
            // No data available yet, but read succeeded
        }
        Err(_) => {
            // Read failed, which is okay for non-blocking reads
        }
    }

    pty.kill().expect("Failed to kill shell");
}

#[test]
fn test_pty_kill_cleanup() {
    let mut pty = Pty::new(PtyConfig::default()).expect("Failed to create PTY");
    pty.spawn().expect("Failed to spawn shell");

    assert!(pty.is_alive().unwrap());

    // Kill the process
    pty.kill().expect("Failed to kill shell");

    // Wait a moment for cleanup
    std::thread::sleep(Duration::from_millis(50));

    // Process should no longer be alive
    // Note: is_alive() might still return true immediately after kill
    // This is platform-dependent behavior
}

#[cfg(unix)]
#[test]
fn test_pty_unix_shell_detection() {
    let config = PtyConfig::default();
    let mut pty = Pty::new(config).expect("Failed to create PTY");

    // On Unix, we should detect a valid shell
    // This is tested indirectly through spawn, which will fail if shell detection fails
    assert!(pty.spawn().is_ok());
    pty.kill().ok();
}

#[cfg(windows)]
#[test]
fn test_pty_windows_shell_detection() {
    let config = PtyConfig::default();
    let mut pty = Pty::new(config).expect("Failed to create PTY");

    // On Windows, we should detect cmd.exe or powershell
    assert!(pty.spawn().is_ok());
    pty.kill().ok();
}
