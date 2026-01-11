#!/bin/bash
# Auto-approve claude-sketch MCP tools
# PermissionRequest hook receives JSON on stdin with tool info

# Read the permission request from stdin
read -r input

# Extract the tool name from the JSON
tool_name=$(echo "$input" | grep -o '"tool_name":"[^"]*"' | cut -d'"' -f4)

# Auto-approve all claude-sketch MCP tools
case "$tool_name" in
    mcp__*claude-sketch*__create_sketch|\
    mcp__*claude-sketch*__run_sketch|\
    mcp__*claude-sketch*__stop_sketch|\
    mcp__*claude-sketch*__list_sketches|\
    mcp__*claude-sketch*__delete_sketch)
        echo '{"decision":"allow"}'
        ;;
    *)
        # Don't decide - let the normal permission flow handle it
        echo '{}'
        ;;
esac
