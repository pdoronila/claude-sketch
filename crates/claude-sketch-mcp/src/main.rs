//! Claude Sketch MCP Server
//!
//! This MCP server exposes tools for creating, running, and managing
//! interactive terminal sketches from Claude Code.

mod mcp_protocol;
mod sketch_manager;
mod terminal_launcher;
mod tools;

use std::io::{BufRead, Write};

use anyhow::Result;
use serde_json::json;

use mcp_protocol::{JsonRpcRequest, JsonRpcResponse};
use tools::SketchServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the sketch server
    let server = SketchServer::new()?;

    // Read from stdin, write to stdout
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.is_empty() {
            continue;
        }

        // Parse the JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let response = JsonRpcResponse::error(None, -32700, format!("Parse error: {}", e));
                writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
                stdout.flush()?;
                continue;
            }
        };

        // Handle the request
        let response = handle_request(&server, &request).await;

        // Write the response
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }

    Ok(())
}

async fn handle_request(server: &SketchServer, request: &JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => {
            let result = server.initialize();
            JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
        }
        "initialized" => {
            // This is a notification, no response needed
            // But we'll send an empty response anyway since we're using request/response pattern
            JsonRpcResponse::success(request.id.clone(), json!({}))
        }
        "tools/list" => {
            let result = server.list_tools();
            JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
        }
        "tools/call" => {
            let params = request.params.as_ref();
            let tool_name = params
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            let arguments = params.and_then(|p| p.get("arguments")).cloned();

            let result = server.call_tool(tool_name, arguments).await;
            JsonRpcResponse::success(request.id.clone(), serde_json::to_value(result).unwrap())
        }
        "ping" => JsonRpcResponse::success(request.id.clone(), json!({})),
        _ => JsonRpcResponse::error(
            request.id.clone(),
            -32601,
            format!("Method not found: {}", request.method),
        ),
    }
}
