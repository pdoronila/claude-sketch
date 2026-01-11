//! MCP tool definitions for claude-sketch

use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::mcp_protocol::{
    CallToolResult, Content, InitializeResult, ListToolsResult, ServerCapabilities, ServerInfo,
    Tool, ToolInputSchema, ToolsCapability,
};
use crate::sketch_manager::{SketchInfo, SketchManager, SketchStatus};

/// The MCP server for claude-sketch
pub struct SketchServer {
    manager: Arc<Mutex<SketchManager>>,
}

impl SketchServer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            manager: Arc::new(Mutex::new(SketchManager::new()?)),
        })
    }

    pub fn initialize(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
            },
            server_info: ServerInfo {
                name: "claude-sketch".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "Claude Sketch allows you to create interactive terminal visualizations using Rust and ratatui. \
                 Use the create_sketch tool to write a sketch, then run_sketch to compile and display it in a terminal pane. \
                 Sketches should implement the SketchApp trait from claude-sketch-runtime.".to_string()
            ),
        }
    }

    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: vec![
                Tool {
                    name: "create_sketch".to_string(),
                    description: Some("Create a new interactive terminal sketch from Rust/ratatui source code. The source code should use claude-sketch-runtime and implement the SketchApp trait.".to_string()),
                    input_schema: ToolInputSchema {
                        r#type: "object".to_string(),
                        properties: Some(json!({
                            "name": {
                                "type": "string",
                                "description": "Name of the sketch (alphanumeric, underscores, hyphens only)"
                            },
                            "description": {
                                "type": "string",
                                "description": "Optional description of what the sketch does"
                            },
                            "source_code": {
                                "type": "string",
                                "description": "The Rust source code for the sketch"
                            }
                        })),
                        required: Some(vec!["name".to_string(), "source_code".to_string()]),
                    },
                },
                Tool {
                    name: "run_sketch".to_string(),
                    description: Some("Compile and run a sketch in a new terminal pane (supports iTerm2, tmux, Ghostty)".to_string()),
                    input_schema: ToolInputSchema {
                        r#type: "object".to_string(),
                        properties: Some(json!({
                            "name": {
                                "type": "string",
                                "description": "Name of the sketch to run"
                            }
                        })),
                        required: Some(vec!["name".to_string()]),
                    },
                },
                Tool {
                    name: "stop_sketch".to_string(),
                    description: Some("Stop a running sketch".to_string()),
                    input_schema: ToolInputSchema {
                        r#type: "object".to_string(),
                        properties: Some(json!({
                            "name": {
                                "type": "string",
                                "description": "Name of the sketch to stop"
                            }
                        })),
                        required: Some(vec!["name".to_string()]),
                    },
                },
                Tool {
                    name: "list_sketches".to_string(),
                    description: Some("List all sketches and their current status".to_string()),
                    input_schema: ToolInputSchema {
                        r#type: "object".to_string(),
                        properties: None,
                        required: None,
                    },
                },
                Tool {
                    name: "delete_sketch".to_string(),
                    description: Some("Delete a sketch and all its files".to_string()),
                    input_schema: ToolInputSchema {
                        r#type: "object".to_string(),
                        properties: Some(json!({
                            "name": {
                                "type": "string",
                                "description": "Name of the sketch to delete"
                            }
                        })),
                        required: Some(vec!["name".to_string()]),
                    },
                },
            ],
        }
    }

    pub async fn call_tool(&self, tool_name: &str, arguments: Option<Value>) -> CallToolResult {
        let args = arguments.unwrap_or(json!({}));
        let manager = self.manager.lock().await;

        match tool_name {
            "create_sketch" => {
                let name = args["name"].as_str().unwrap_or("");
                let description = args["description"].as_str();
                let source_code = args["source_code"].as_str().unwrap_or("");

                match manager.create_sketch(name, description, source_code) {
                    Ok(info) => {
                        let output = SketchInfoOutput::from(info);
                        let text = serde_json::to_string_pretty(&output)
                            .unwrap_or_else(|_| format!("Sketch '{}' created successfully", name));
                        CallToolResult {
                            content: vec![Content::text(text)],
                            is_error: None,
                        }
                    }
                    Err(e) => CallToolResult {
                        content: vec![Content::text(format!("Failed to create sketch: {}", e))],
                        is_error: Some(true),
                    },
                }
            }
            "run_sketch" => {
                let name = args["name"].as_str().unwrap_or("");

                match manager.run_sketch(name) {
                    Ok(result) => {
                        let text = if result.success {
                            format!("Sketch '{}' is now running (pid: {:?})", name, result.pid)
                        } else {
                            result.message
                        };
                        CallToolResult {
                            content: vec![Content::text(text)],
                            is_error: if result.success { None } else { Some(true) },
                        }
                    }
                    Err(e) => CallToolResult {
                        content: vec![Content::text(format!("Failed to run sketch: {}", e))],
                        is_error: Some(true),
                    },
                }
            }
            "stop_sketch" => {
                let name = args["name"].as_str().unwrap_or("");

                match manager.stop_sketch(name) {
                    Ok(()) => CallToolResult {
                        content: vec![Content::text(format!("Sketch '{}' stopped", name))],
                        is_error: None,
                    },
                    Err(e) => CallToolResult {
                        content: vec![Content::text(format!("Failed to stop sketch: {}", e))],
                        is_error: Some(true),
                    },
                }
            }
            "list_sketches" => match manager.list_sketches() {
                Ok(sketches) => {
                    let output = ListSketchesOutput {
                        sketches: sketches.into_iter().map(SketchInfoOutput::from).collect(),
                    };
                    let text =
                        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "[]".to_string());
                    CallToolResult {
                        content: vec![Content::text(text)],
                        is_error: None,
                    }
                }
                Err(e) => CallToolResult {
                    content: vec![Content::text(format!("Failed to list sketches: {}", e))],
                    is_error: Some(true),
                },
            },
            "delete_sketch" => {
                let name = args["name"].as_str().unwrap_or("");

                match manager.delete_sketch(name) {
                    Ok(()) => CallToolResult {
                        content: vec![Content::text(format!("Sketch '{}' deleted", name))],
                        is_error: None,
                    },
                    Err(e) => CallToolResult {
                        content: vec![Content::text(format!("Failed to delete sketch: {}", e))],
                        is_error: Some(true),
                    },
                }
            }
            _ => CallToolResult {
                content: vec![Content::text(format!("Unknown tool: {}", tool_name))],
                is_error: Some(true),
            },
        }
    }
}

/// Output for list_sketches tool
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSketchesOutput {
    pub sketches: Vec<SketchInfoOutput>,
}

/// Sketch info for output
#[derive(Debug, Serialize, Deserialize)]
pub struct SketchInfoOutput {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub pid: Option<u32>,
}

impl From<SketchInfo> for SketchInfoOutput {
    fn from(info: SketchInfo) -> Self {
        Self {
            name: info.name,
            description: info.description,
            status: match info.status {
                SketchStatus::Created => "created".to_string(),
                SketchStatus::Compiling => "compiling".to_string(),
                SketchStatus::Ready => "ready".to_string(),
                SketchStatus::Running => "running".to_string(),
                SketchStatus::Failed => "failed".to_string(),
                SketchStatus::Stopped => "stopped".to_string(),
            },
            pid: info.pid,
        }
    }
}
