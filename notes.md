Claude-sketch is now a complete Claude Code plugin. Here's the final structure:

  claude-sketch/
  ├── .claude-plugin/
  │   └── plugin.json          # Plugin manifest
  ├── .mcp.json                # MCP server configuration
  ├── skills/
  │   └── sketch/
  │       └── SKILL.md         # Teaches Claude how to create sketches
  ├── commands/
  │   └── sketch.md            # /claude-sketch:sketch command
  ├── marketplace.json         # For marketplace distribution
  ├── setup.sh                 # Build & install script
  ├── README.md                # Documentation
  ├── crates/
  │   ├── claude-sketch-mcp/   # MCP server
  │   └── claude-sketch-runtime/ # Runtime library
  └── examples/counter/        # Example sketch

  To Install the Plugin

  1. Build first (required - the MCP server must be compiled):
  cd /Users/doronila/git/claude-sketch
  ./setup.sh
  2. Install the plugin:
  claude /plugin install /Users/doronila/git/claude-sketch

  How to Use

  After installation, you can:

  1. Ask Claude directly:
  Create a counter with increment and decrement buttons
  2. Use the slash command:
  /claude-sketch:sketch a todo list with checkboxes
  3. The MCP tools are available:
    - create_sketch - Create/update a sketch
    - run_sketch - Compile and display
    - stop_sketch - Stop a running sketch
    - list_sketches - See all sketches
    - delete_sketch - Remove a sketch

  Plugin Components
  ┌─────────────┬────────────────────────────────────────────┐
  │  Component  │                  Purpose                   │
  ├─────────────┼────────────────────────────────────────────┤
  │ plugin.json │ Defines the plugin identity and structure  │
  ├─────────────┼────────────────────────────────────────────┤
  │ .mcp.json   │ Configures the MCP server connection       │
  ├─────────────┼────────────────────────────────────────────┤
  │ SKILL.md    │ Teaches Claude the sketch API and patterns │
  ├─────────────┼────────────────────────────────────────────┤
  │ sketch.md   │ Provides the /sketch slash command         │
  └─────────────┴────────────────────────────────────────────┘
  The skill file contains the complete template and examples that Claude uses to generate correct sketch code.
