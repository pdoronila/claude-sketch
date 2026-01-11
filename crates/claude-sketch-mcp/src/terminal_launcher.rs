//! Terminal detection and pane creation
//!
//! Supports iTerm2, tmux, and Ghostty terminals.

use std::path::Path;
use std::process::{Child, Command, Stdio};

use anyhow::{anyhow, Context, Result};

/// Supported terminal types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType {
    /// iTerm2 on macOS
    ITerm2,
    /// tmux terminal multiplexer
    Tmux,
    /// Ghostty terminal
    Ghostty,
    /// Unknown/unsupported terminal - will open new window
    Unknown,
}

/// Detect the current terminal environment
pub fn detect_terminal() -> TerminalType {
    // Check for tmux first (it can run inside other terminals)
    if std::env::var("TMUX").is_ok() {
        return TerminalType::Tmux;
    }

    // Check for Ghostty
    if std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        return TerminalType::Ghostty;
    }

    // Check for iTerm2
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        if term_program == "iTerm.app" {
            return TerminalType::ITerm2;
        }
    }

    // Check for iTerm2 via LC_TERMINAL (alternative detection)
    if let Ok(lc_terminal) = std::env::var("LC_TERMINAL") {
        if lc_terminal == "iTerm2" {
            return TerminalType::ITerm2;
        }
    }

    TerminalType::Unknown
}

/// Launch a binary in a new terminal pane
pub fn launch_in_terminal(terminal: &TerminalType, binary_path: &Path) -> Result<Child> {
    let binary_str = binary_path
        .to_str()
        .ok_or_else(|| anyhow!("Invalid binary path"))?;

    match terminal {
        TerminalType::ITerm2 => launch_iterm2_pane(binary_str),
        TerminalType::Tmux => launch_tmux_pane(binary_str),
        TerminalType::Ghostty => launch_ghostty_pane(binary_str),
        TerminalType::Unknown => launch_new_terminal(binary_str),
    }
}

/// Launch a pane in iTerm2 using AppleScript
fn launch_iterm2_pane(binary_path: &str) -> Result<Child> {
    // Split vertically, run command in the NEW session, keep focus there,
    // and close pane when the sketch exits (using exec to replace the shell)
    let script = format!(
        r#"
tell application "iTerm"
    tell current session of current window
        set newSession to (split vertically with default profile)
    end tell
    tell newSession
        write text "exec \"{}\""
        select
    end tell
end tell
"#,
        binary_path.replace("\"", "\\\"")
    );

    // Execute the AppleScript
    let status = Command::new("osascript")
        .args(["-e", &script])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to execute AppleScript for iTerm2")?;

    if !status.success() {
        return Err(anyhow!("AppleScript execution failed"));
    }

    // Return a dummy child process (the actual process is managed by iTerm2)
    // We'll use a sleep process as a placeholder
    Command::new("sleep")
        .arg("infinity")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to create placeholder process")
}

/// Launch a pane in tmux
fn launch_tmux_pane(binary_path: &str) -> Result<Child> {
    // Create a new pane to the right
    let status = Command::new("tmux")
        .args(["split-window", "-h", binary_path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to create tmux pane")?;

    if !status.success() {
        return Err(anyhow!("tmux split-window failed"));
    }

    // Return a placeholder process
    Command::new("sleep")
        .arg("infinity")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to create placeholder process")
}

/// Launch a pane in Ghostty
fn launch_ghostty_pane(binary_path: &str) -> Result<Child> {
    // Ghostty supports splits via keybindings, but for programmatic control
    // we need to use the Ghostty CLI or a new window
    // For now, we'll try the ghostty CLI if available, otherwise new window

    // Try to use ghostty CLI for new tab/split
    let ghostty_result = Command::new("ghostty")
        .args(["--", binary_path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match ghostty_result {
        Ok(child) => Ok(child),
        Err(_) => {
            // Fall back to opening in a new terminal window
            launch_new_terminal(binary_path)
        }
    }
}

/// Launch in a new terminal window (fallback)
fn launch_new_terminal(binary_path: &str) -> Result<Child> {
    // On macOS, use open -a Terminal
    #[cfg(target_os = "macos")]
    {
        // Create a temporary script to run the binary
        let script = format!(
            "#!/bin/bash\n{}\nread -p 'Press enter to close...'",
            binary_path
        );

        let temp_script = std::env::temp_dir().join("claude_sketch_run.sh");
        std::fs::write(&temp_script, &script).context("Failed to write temp script")?;

        // Make executable
        Command::new("chmod")
            .args(["+x", temp_script.to_str().unwrap()])
            .status()
            .context("Failed to make script executable")?;

        Command::new("open")
            .args(["-a", "Terminal", temp_script.to_str().unwrap()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to open Terminal.app")
    }

    #[cfg(target_os = "linux")]
    {
        // Try common terminal emulators
        let terminals = ["gnome-terminal", "konsole", "xterm"];

        for term in &terminals {
            let result = Command::new(term)
                .args(["-e", binary_path])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();

            if result.is_ok() {
                return result.context("Failed to spawn terminal");
            }
        }

        Err(anyhow!("No supported terminal emulator found"))
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err(anyhow!("Unsupported operating system"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_terminal() {
        // This test will vary based on environment
        let terminal = detect_terminal();
        // Just ensure it returns something
        assert!(matches!(
            terminal,
            TerminalType::ITerm2
                | TerminalType::Tmux
                | TerminalType::Ghostty
                | TerminalType::Unknown
        ));
    }
}
