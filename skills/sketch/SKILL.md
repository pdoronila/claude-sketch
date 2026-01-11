# Claude Sketch Skill

Create interactive terminal visualizations using Rust and ratatui.

## When to Use

Use this skill when the user asks you to:
- Create an interactive terminal UI or visualization
- Build a TUI (terminal user interface) application
- Make a "sketch" or visual demo in the terminal
- Create something like a Claude artifact but in the terminal

## How It Works

1. **Create a sketch** using the `create_sketch` MCP tool with Rust source code
2. **Run the sketch** using the `run_sketch` MCP tool - it compiles and opens in a terminal pane
3. **Update sketches** by calling `create_sketch` again with the same name, then `run_sketch`

## Sketch Template

All sketches must implement the `SketchApp` trait from `claude-sketch-runtime`:

```rust
use claude_sketch_runtime::prelude::*;

struct MySketch {
    // Your state here
}

impl SketchApp for MySketch {
    fn new() -> Self {
        Self {
            // Initialize state
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        match event {
            // Handle keyboard events
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                ControlFlow::Break  // Exit on 'q'
            }
            // Handle mouse clicks
            SketchEvent::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column, row, ..
            }) => {
                // Handle click at (column, row)
                ControlFlow::Continue
            }
            _ => ControlFlow::Continue
        }
    }

    fn render(&self, frame: &mut Frame) {
        // Render your UI using ratatui widgets
        let area = frame.area();

        let paragraph = Paragraph::new("Hello, Sketch!")
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

fn main() -> Result<()> {
    run_sketch::<MySketch>()
}
```

## Available from Prelude

The `claude_sketch_runtime::prelude::*` provides:

### Types
- `SketchApp` - The trait to implement
- `SketchEvent` - Events (Key, Mouse, Resize, Tick)
- `ControlFlow` - Continue or Break

### Keyboard
- `KeyCode` - Char, Up, Down, Left, Right, Enter, Esc, etc.
- `KeyEvent` - code, modifiers
- `KeyModifiers` - CONTROL, SHIFT, ALT

### Mouse
- `MouseEvent` - kind, column, row, modifiers
- `MouseEventKind` - Down, Up, Drag, ScrollDown, ScrollUp
- `MouseButton` - Left, Right, Middle

### Widgets (from claude-sketch-runtime)
- `Button` - Clickable button with `.contains(x, y)` for hit detection
- `Counter` - Numeric counter with min/max/step
- `TextInput` - Text input field with cursor

### Ratatui Re-exports
- `Frame` - The render target
- `Rect` - Rectangle for areas
- `Layout`, `Constraint` - For layouts
- `Paragraph`, `Block`, `Borders` - Basic widgets
- `Style`, `Color`, `Modifier` - Styling
- `Alignment` - Text alignment

### Helpers
- `centered_rect(width, height, area)` - Create a centered rectangle
- `run_sketch::<T>()` - Run your sketch app

## Example: Counter with Buttons

```rust
use claude_sketch_runtime::prelude::*;
use std::cell::Cell;

struct CounterSketch {
    count: i64,
    inc_bounds: Cell<Rect>,
    dec_bounds: Cell<Rect>,
}

impl SketchApp for CounterSketch {
    fn new() -> Self {
        Self {
            count: 0,
            inc_bounds: Cell::new(Rect::default()),
            dec_bounds: Cell::new(Rect::default()),
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        match event {
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('q'), .. }) |
            SketchEvent::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                ControlFlow::Break
            }
            SketchEvent::Key(KeyEvent { code: KeyCode::Up, .. }) |
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('+'), .. }) => {
                self.count += 1;
                ControlFlow::Continue
            }
            SketchEvent::Key(KeyEvent { code: KeyCode::Down, .. }) |
            SketchEvent::Key(KeyEvent { code: KeyCode::Char('-'), .. }) => {
                self.count -= 1;
                ControlFlow::Continue
            }
            SketchEvent::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column, row, ..
            }) => {
                let contains = |bounds: Rect, x: u16, y: u16| -> bool {
                    x >= bounds.x && x < bounds.x + bounds.width
                        && y >= bounds.y && y < bounds.y + bounds.height
                };

                if contains(self.inc_bounds.get(), column, row) {
                    self.count += 1;
                } else if contains(self.dec_bounds.get(), column, row) {
                    self.count -= 1;
                }
                ControlFlow::Continue
            }
            _ => ControlFlow::Continue
        }
    }

    fn render(&self, frame: &mut Frame) {
        let center = centered_rect(50, 12, frame.area());

        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ]).split(center);

        // Title
        frame.render_widget(
            Paragraph::new("Counter Sketch")
                .style(Style::default().bold())
                .alignment(Alignment::Center),
            layout[0]
        );

        // Counter value
        frame.render_widget(
            Paragraph::new(format!("{}", self.count))
                .style(Style::default().fg(Color::Cyan).bold())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Value")),
            layout[1]
        );

        // Buttons
        let buttons = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(layout[2]);

        self.dec_bounds.set(buttons[0]);
        self.inc_bounds.set(buttons[1]);

        frame.render_widget(
            Paragraph::new("[-] Decrement")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            buttons[0]
        );
        frame.render_widget(
            Paragraph::new("[+] Increment")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            buttons[1]
        );

        // Help
        frame.render_widget(
            Paragraph::new("Click buttons or use +/- keys | q to quit")
                .style(Style::default().dim())
                .alignment(Alignment::Center),
            layout[3]
        );
    }
}

fn main() -> Result<()> {
    run_sketch::<CounterSketch>()
}
```

## MCP Tools Available

- `create_sketch` - Create or update a sketch with Rust source code
- `run_sketch` - Compile and run a sketch in a new terminal pane
- `stop_sketch` - Stop a running sketch
- `list_sketches` - List all sketches and their status
- `delete_sketch` - Delete a sketch

## Terminal Support

Sketches run in a new terminal pane. Supported terminals:
- **iTerm2** - Split pane to the right
- **tmux** - Horizontal split
- **Ghostty** - New window/pane

## Interior Mutability for Clickable Areas

Since `render(&self, ...)` takes an immutable reference but button bounds are calculated during render, use `Cell<Rect>` or `RefCell` to store clickable areas:

```rust
use std::cell::Cell;

struct MySketch {
    button_area: Cell<Rect>,  // Use Cell for simple Copy types like Rect
}

fn render(&self, frame: &mut Frame) {
    let button_rect = centered_rect(20, 3, frame.area());
    self.button_area.set(button_rect);  // Cell allows mutation in &self
}

fn update(&mut self, event: SketchEvent) -> ControlFlow {
    if let SketchEvent::Mouse(mouse) = event {
        let area = self.button_area.get();
        // Check if click is inside area...
    }
}
```

## Tips

1. **Always handle 'q' or Esc** to allow users to exit
2. **Use `Cell<Rect>` for button bounds** to track click areas during render (required because render takes `&self`)
3. **Use centered_rect()** to center your UI
4. **Test with keyboard first**, then add mouse support
5. **Keep sketches focused** - they're meant to be quick visualizations
