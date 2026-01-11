# Claude Sketch

Create interactive terminal visualizations using Rust and ratatui - like Claude artifacts but in the terminal.

## What is Claude Sketch?

Claude Sketch gives Claude Code the power to create dynamic, interactive terminal UIs on-the-fly. Instead of predefined canvases, Claude generates Rust code using ratatui to create any visualization you need.

## Installation

### As a Claude Code Plugin (Recommended)

Pre-built binaries are included for macOS (arm64/x64), Linux (x64/arm64), and Windows (x64).

```bash
git clone https://github.com/yourusername/claude-sketch
cd claude-sketch
claude /plugin install ./
```

That's it! No compilation required for the plugin itself.

**Note:** You'll still need Rust/cargo installed to create sketches, since your sketches are custom Rust code that gets compiled.

### From Marketplace

```bash
claude /plugin marketplace add yourusername/claude-sketch
claude /plugin install claude-sketch@yourusername/claude-sketch
```

### Manual MCP Configuration

If you prefer manual setup, add to `~/.claude/mcp_settings.json`:

```json
{
  "mcpServers": {
    "claude-sketch": {
      "command": "/path/to/claude-sketch/bin/run-mcp.sh",
      "args": []
    }
  }
}
```

## Requirements

- **Claude Code** - The plugin integrates with Claude Code
- **Rust/cargo** - Required for compiling your sketches (not for plugin installation)
- **Supported terminal** - iTerm2, tmux, or Ghostty for split-pane display

## Usage

### With the Plugin

Simply ask Claude to create a sketch:

```
Create a counter with increment and decrement buttons
```

Or use the slash command:

```
/claude-sketch:sketch a todo list with checkboxes
```

### Available MCP Tools

- `create_sketch` - Create a sketch from Rust source code
- `run_sketch` - Compile and run in a terminal pane
- `stop_sketch` - Stop a running sketch
- `list_sketches` - List all sketches
- `delete_sketch` - Delete a sketch

## Terminal Support

Sketches open in a new terminal pane:
- **iTerm2** - Split pane to the right
- **tmux** - Horizontal split pane
- **Ghostty** - New pane/window

## Example Sketch

When you ask Claude to "create a counter with buttons", it generates:

```rust
use claude_sketch_runtime::prelude::*;
use std::cell::RefCell;

struct CounterSketch {
    count: i64,
    inc_bounds: RefCell<Option<Rect>>,
    dec_bounds: RefCell<Option<Rect>>,
}

impl SketchApp for CounterSketch {
    fn new() -> Self {
        Self {
            count: 0,
            inc_bounds: RefCell::new(None),
            dec_bounds: RefCell::new(None),
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        match event {
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                ControlFlow::Break
            }
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('+'), .. }) => {
                self.count += 1;
                ControlFlow::Continue
            }
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('-'), .. }) => {
                self.count -= 1;
                ControlFlow::Continue
            }
            SketchEvent::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column, row, ..
            }) => {
                // Handle button clicks
                ControlFlow::Continue
            }
            _ => ControlFlow::Continue
        }
    }

    fn render(&self, frame: &mut Frame) {
        // Render counter UI with ratatui
    }
}

fn main() -> Result<()> {
    run_sketch::<CounterSketch>()
}
```

## Project Structure

```
claude-sketch/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest
├── .mcp.json                # MCP server config
├── bin/
│   ├── run-mcp.sh           # MCP launcher (auto-selects platform)
│   ├── get-binary.sh        # Platform detection helper
│   ├── darwin-arm64/        # macOS Apple Silicon binary
│   ├── darwin-x64/          # macOS Intel binary
│   ├── linux-x64/           # Linux x64 binary
│   ├── linux-arm64/         # Linux ARM binary
│   └── windows-x64/         # Windows binary
├── skills/
│   └── sketch/
│       └── SKILL.md         # Skill for creating sketches
├── commands/
│   └── sketch.md            # /sketch command
├── crates/
│   ├── claude-sketch-mcp/   # MCP server source
│   └── claude-sketch-runtime/ # Runtime library for sketches
└── examples/
    └── counter/             # Example sketch
```

## Development

For contributors who want to modify the plugin:

```bash
# Build from source (creates target/release/claude-sketch-mcp)
./setup.sh

# Or build manually
cargo build --release

# Test the MCP server
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./target/release/claude-sketch-mcp

# Run the counter example
cargo run -p counter-sketch --release
```

### Building for Release

Pre-built binaries are created via GitHub Actions when you push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This builds binaries for all supported platforms and creates a release.

## How It Works

1. You ask Claude to create a visualization
2. Claude generates Rust code using `claude-sketch-runtime`
3. The MCP server compiles the code with `cargo build`
4. The compiled binary runs in a new terminal pane
5. You interact with the sketch using keyboard/mouse
6. Press 'q' to exit

## Sketch Persistence

Sketches are saved to `~/.claude-sketch/sketches/` and persist across sessions. You can:
- List them with `list_sketches`
- Re-run them with `run_sketch`
- Delete them with `delete_sketch`

## License

MIT
