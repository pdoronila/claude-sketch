//! Sketch lifecycle management
//!
//! Handles creating, compiling, running, and stopping sketches.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::terminal_launcher::{detect_terminal, launch_in_terminal, TerminalType};

/// Status of a sketch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SketchStatus {
    /// Sketch exists but hasn't been compiled
    Created,
    /// Sketch is being compiled
    Compiling,
    /// Sketch has been compiled and is ready to run
    Ready,
    /// Sketch is currently running
    Running,
    /// Sketch compilation failed
    Failed,
    /// Sketch was stopped
    Stopped,
}

/// Information about a sketch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SketchInfo {
    pub name: String,
    pub description: Option<String>,
    pub status: SketchStatus,
    pub pid: Option<u32>,
    pub path: PathBuf,
}

/// Manages the lifecycle of sketches
pub struct SketchManager {
    /// Base directory for sketches (<cwd>/.claude-sketch/sketches)
    sketches_dir: PathBuf,
    /// Path to the claude-sketch-runtime crate (for Cargo.toml references)
    runtime_path: PathBuf,
    /// Currently running sketch processes
    running: Arc<Mutex<HashMap<String, Child>>>,
    /// Detected terminal type
    terminal: TerminalType,
}

impl SketchManager {
    /// Create a new sketch manager
    pub fn new() -> Result<Self> {
        // Use the current working directory (inherited from Claude Code)
        // so sketches are stored in the project directory
        let cwd = std::env::current_dir().context("Failed to get current working directory")?;
        let base_dir = cwd.join(".claude-sketch");
        let sketches_dir = base_dir.join("sketches");

        // Create directories if they don't exist
        std::fs::create_dir_all(&sketches_dir)
            .context("Failed to create sketches directory")?;

        // Get the runtime path from CLAUDE_PLUGIN_ROOT (set by Claude Code for plugins)
        // or fall back to relative path for development
        let runtime_path = std::env::var("CLAUDE_PLUGIN_ROOT")
            .map(|root| PathBuf::from(root).join("crates/claude-sketch-runtime"))
            .unwrap_or_else(|_| {
                // Fallback: try to find runtime relative to the MCP binary
                std::env::current_exe()
                    .ok()
                    .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
                    .map(|p| p.join("../../crates/claude-sketch-runtime"))
                    .unwrap_or_else(|| PathBuf::from("claude-sketch-runtime"))
            });

        let terminal = detect_terminal();

        Ok(Self {
            sketches_dir,
            runtime_path,
            running: Arc::new(Mutex::new(HashMap::new())),
            terminal,
        })
    }

    /// Get the path to a sketch directory
    fn sketch_path(&self, name: &str) -> PathBuf {
        self.sketches_dir.join(name)
    }

    /// Validate a sketch name
    fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Sketch name cannot be empty"));
        }
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(anyhow!(
                "Sketch name can only contain alphanumeric characters, underscores, and hyphens"
            ));
        }
        if name.len() > 64 {
            return Err(anyhow!("Sketch name cannot exceed 64 characters"));
        }
        Ok(())
    }

    /// Create a new sketch from source code
    pub fn create_sketch(
        &self,
        name: &str,
        description: Option<&str>,
        source_code: &str,
    ) -> Result<SketchInfo> {
        Self::validate_name(name)?;

        let sketch_dir = self.sketch_path(name);
        let src_dir = sketch_dir.join("src");

        // Create directories
        std::fs::create_dir_all(&src_dir).context("Failed to create sketch source directory")?;

        // Generate Cargo.toml with empty [workspace] to exclude from parent workspaces
        let cargo_toml = format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
claude-sketch-runtime = {{ version = "0.1", path = "{runtime_path}" }}
ratatui = "0.29"
crossterm = "0.28"
anyhow = "1"

# Exclude from parent workspaces
[workspace]
"#,
            name = name,
            runtime_path = self.runtime_path.display()
        );

        std::fs::write(sketch_dir.join("Cargo.toml"), cargo_toml)
            .context("Failed to write Cargo.toml")?;

        // Write main.rs
        std::fs::write(src_dir.join("main.rs"), source_code)
            .context("Failed to write main.rs")?;

        Ok(SketchInfo {
            name: name.to_string(),
            description: description.map(String::from),
            status: SketchStatus::Created,
            pid: None,
            path: sketch_dir,
        })
    }

    /// Compile a sketch
    pub fn compile_sketch(&self, name: &str) -> Result<CompileResult> {
        Self::validate_name(name)?;

        let sketch_dir = self.sketch_path(name);
        if !sketch_dir.exists() {
            return Err(anyhow!("Sketch '{}' does not exist", name));
        }

        // Run cargo build
        let output = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&sketch_dir)
            .output()
            .context("Failed to execute cargo build")?;

        if output.status.success() {
            Ok(CompileResult {
                success: true,
                binary_path: Some(sketch_dir.join("target/release").join(name)),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        } else {
            Ok(CompileResult {
                success: false,
                binary_path: None,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    /// Run a sketch (compile if needed)
    pub fn run_sketch(&self, name: &str) -> Result<RunResult> {
        Self::validate_name(name)?;

        let sketch_dir = self.sketch_path(name);
        if !sketch_dir.exists() {
            return Err(anyhow!("Sketch '{}' does not exist", name));
        }

        // Stop if already running
        self.stop_sketch(name).ok();

        // Compile
        let compile_result = self.compile_sketch(name)?;
        if !compile_result.success {
            return Ok(RunResult {
                success: false,
                message: format!("Compilation failed:\n{}", compile_result.stderr),
                pid: None,
            });
        }

        let binary_path = compile_result
            .binary_path
            .ok_or_else(|| anyhow!("No binary path after successful compilation"))?;

        // Launch in terminal
        match launch_in_terminal(&self.terminal, &binary_path) {
            Ok(child) => {
                let pid = child.id();
                let mut running = self.running.lock().unwrap();
                running.insert(name.to_string(), child);

                Ok(RunResult {
                    success: true,
                    message: format!("Sketch '{}' is now running", name),
                    pid: Some(pid),
                })
            }
            Err(e) => Ok(RunResult {
                success: false,
                message: format!("Failed to launch sketch: {}", e),
                pid: None,
            }),
        }
    }

    /// Stop a running sketch
    pub fn stop_sketch(&self, name: &str) -> Result<()> {
        Self::validate_name(name)?;

        let mut running = self.running.lock().unwrap();
        if let Some(mut child) = running.remove(name) {
            // Try to kill gracefully first
            let _ = child.kill();
            let _ = child.wait();
        }
        Ok(())
    }

    /// List all sketches
    pub fn list_sketches(&self) -> Result<Vec<SketchInfo>> {
        let mut sketches = Vec::new();

        if !self.sketches_dir.exists() {
            return Ok(sketches);
        }

        let running = self.running.lock().unwrap();

        for entry in std::fs::read_dir(&self.sketches_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
                    .to_string();

                let (status, pid) = if let Some(child) = running.get(&name) {
                    (SketchStatus::Running, Some(child.id()))
                } else if path.join("target/release").join(&name).exists() {
                    (SketchStatus::Ready, None)
                } else if path.join("src/main.rs").exists() {
                    (SketchStatus::Created, None)
                } else {
                    continue; // Invalid sketch directory
                };

                sketches.push(SketchInfo {
                    name,
                    description: None,
                    status,
                    pid,
                    path,
                });
            }
        }

        Ok(sketches)
    }

    /// Delete a sketch
    pub fn delete_sketch(&self, name: &str) -> Result<()> {
        Self::validate_name(name)?;

        // Stop if running
        self.stop_sketch(name).ok();

        let sketch_dir = self.sketch_path(name);
        if sketch_dir.exists() {
            std::fs::remove_dir_all(&sketch_dir).context("Failed to delete sketch directory")?;
        }

        Ok(())
    }
}

/// Result of compiling a sketch
#[derive(Debug)]
pub struct CompileResult {
    pub success: bool,
    pub binary_path: Option<PathBuf>,
    pub stdout: String,
    pub stderr: String,
}

/// Result of running a sketch
#[derive(Debug, Serialize, Deserialize)]
pub struct RunResult {
    pub success: bool,
    pub message: String,
    pub pid: Option<u32>,
}
