#!/bin/bash
# Claude Sketch Plugin - Development Setup Script
#
# NOTE: This script is for DEVELOPMENT/CONTRIBUTORS only.
# Plugin users don't need to run this - pre-built binaries are included.

set -e

echo "=== Development Build ==="
echo ""
echo "Building claude-sketch from source..."
cargo build --release

echo ""
echo "Build complete!"
echo ""
echo "=== For Plugin Users ==="
echo "Pre-built binaries are included. Just install the plugin:"
echo "  claude /plugin install $(pwd)"
echo ""
echo "=== For Developers ==="
echo "After building, install the plugin for testing:"
echo "  claude /plugin install $(pwd)"
echo ""
echo "After installation, you can:"
echo "  1. Ask Claude to 'create a counter sketch'"
echo "  2. Use /sketch <description>"
echo ""
