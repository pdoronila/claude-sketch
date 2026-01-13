#!/bin/bash
# Build a Python virtual environment with textual for the current platform.
#
# Usage:
#   ./scripts/build-venv.sh              # Build for current platform
#   ./scripts/build-venv.sh darwin-arm64 # Build for specific platform
#
# This script is typically run via GitHub Actions to build venvs for each platform.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Detect or use provided platform
if [ -n "$1" ]; then
    PLATFORM="$1"
else
    # Auto-detect platform
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"

    case "$OS" in
        darwin)
            if [ "$ARCH" = "arm64" ]; then
                PLATFORM="darwin-arm64"
            else
                PLATFORM="darwin-x64"
            fi
            ;;
        linux)
            if [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
                PLATFORM="linux-arm64"
            else
                PLATFORM="linux-x64"
            fi
            ;;
        msys*|mingw*|cygwin*)
            PLATFORM="windows-x64"
            ;;
        *)
            echo "Unknown platform: $OS $ARCH" >&2
            exit 1
            ;;
    esac
fi

echo "Building venv for platform: $PLATFORM"

VENV_DIR="$PROJECT_ROOT/venv/$PLATFORM"

# Remove existing venv if it exists
if [ -d "$VENV_DIR" ]; then
    echo "Removing existing venv..."
    rm -rf "$VENV_DIR"
fi

# Create the virtual environment
echo "Creating virtual environment..."
python3 -m venv "$VENV_DIR"

# Determine pip path based on platform
if [[ "$PLATFORM" == windows-* ]]; then
    PIP="$VENV_DIR/Scripts/pip"
    PYTHON="$VENV_DIR/Scripts/python"
else
    PIP="$VENV_DIR/bin/pip"
    PYTHON="$VENV_DIR/bin/python3"
fi

# Upgrade pip
echo "Upgrading pip..."
"$PIP" install --upgrade pip

# Install textual and dependencies
echo "Installing textual..."
"$PIP" install textual

# Copy claude_sketch module into site-packages
echo "Installing claude_sketch module..."
SITE_PACKAGES=$("$PYTHON" -c "import site; print(site.getsitepackages()[0])")
cp -r "$PROJECT_ROOT/src/claude_sketch" "$SITE_PACKAGES/"

echo "Done! Venv created at: $VENV_DIR"
echo ""
echo "To test:"
echo "  $PYTHON -c 'from claude_sketch.runtime import SketchApp; print(\"OK\")'"
