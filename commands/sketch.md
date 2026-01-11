---
description: Create an interactive terminal sketch
---

# Sketch Command

The user wants to create an interactive terminal sketch: "$ARGUMENTS"

If the user didn't specify what to create, ask them what kind of interactive terminal visualization they'd like.

## Instructions

1. Generate Rust code following the template and API below
2. Use the `create_sketch` MCP tool to create the sketch
3. Use the `run_sketch` MCP tool to compile and display it

## Required Template

All sketches MUST follow this exact structure:

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
            SketchEvent::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return ControlFlow::Break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return ControlFlow::Break
                    }
                    // Handle other keys...
                    _ => {}
                }
            }
            SketchEvent::Mouse(mouse) => {
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    // Handle click at (mouse.column, mouse.row)
                }
            }
            SketchEvent::Resize(width, height) => {
                // Handle terminal resize
            }
            SketchEvent::Tick => {
                // Handle tick for animations
            }
        }
        ControlFlow::Continue
    }

    fn render(&self, frame: &mut Frame) {
        // Render UI using ratatui widgets
        let area = frame.area();
        let paragraph = Paragraph::new("Hello!")
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

fn main() -> Result<()> {
    run_sketch::<MySketch>()
}
```

## API Reference

### SketchApp Trait (REQUIRED methods)
- `fn new() -> Self` - Create initial state
- `fn update(&mut self, event: SketchEvent) -> ControlFlow` - Handle events
- `fn render(&self, frame: &mut Frame)` - Render the UI

### SketchEvent Variants
- `SketchEvent::Key(KeyEvent)` - Keyboard input
- `SketchEvent::Mouse(MouseEvent)` - Mouse input
- `SketchEvent::Resize(u16, u16)` - Terminal resized
- `SketchEvent::Tick` - Animation tick

### ControlFlow
- `ControlFlow::Continue` - Keep running
- `ControlFlow::Break` - Exit the sketch

### Available from prelude

**Keyboard:**
- `KeyCode` - `Char('x')`, `Up`, `Down`, `Left`, `Right`, `Enter`, `Esc`, `Backspace`, `Tab`, `Delete`
- `KeyEvent` - Has `.code` and `.modifiers` fields
- `KeyModifiers` - `CONTROL`, `SHIFT`, `ALT` (use with `.contains()`)

**Mouse:**
- `MouseEvent` - Has `.kind`, `.column`, `.row`, `.modifiers`
- `MouseEventKind` - `Down(MouseButton)`, `Up(MouseButton)`, `Drag(MouseButton)`, `ScrollDown`, `ScrollUp`
- `MouseButton` - `Left`, `Right`, `Middle`

**Layout:**
- `Layout` - Use `Layout::vertical([...])` or `Layout::horizontal([...])`
- `Constraint` - `Length(n)`, `Min(n)`, `Max(n)`, `Percentage(n)`, `Ratio(a, b)`
- `Rect` - Rectangle area
- `centered_rect(width, height, area)` - Create centered rectangle

**Widgets:**
- `Paragraph::new(text)` - Text display
- `Block::default().borders(Borders::ALL).title("Title")` - Box with border
- `Borders` - `NONE`, `ALL`, `TOP`, `BOTTOM`, `LEFT`, `RIGHT`

**Styling:**
- `Style::default()` - Base style
- `.fg(Color::Red)` - Foreground color
- `.bg(Color::Blue)` - Background color
- `.add_modifier(Modifier::BOLD)` - Add modifier
- `Color` - `Red`, `Green`, `Blue`, `Yellow`, `Cyan`, `Magenta`, `White`, `Black`, `DarkGray`, `Gray`
- `Modifier` - `BOLD`, `DIM`, `ITALIC`, `UNDERLINED`, `CROSSED_OUT`
- `Alignment` - `Left`, `Center`, `Right`

**Other ratatui widgets** (import separately):
```rust
use ratatui::widgets::{List, ListItem, Table, Row, Cell, Gauge, Tabs};
use ratatui::text::{Line, Span, Text};
```

### Helpers
- `run_sketch::<T>()` - Run the sketch (call in main)
- `Result<()>` - Return type for main (from `anyhow`)

## Interior Mutability for Clickable Areas

Since `render(&self, ...)` takes an immutable reference but button bounds are calculated during render, use `Cell<Rect>` to store clickable areas:

```rust
use claude_sketch_runtime::prelude::*;
use std::cell::Cell;

struct MySketch {
    count: i32,
    button_area: Cell<Rect>,  // Use Cell for areas updated in render()
}

impl SketchApp for MySketch {
    fn new() -> Self {
        Self {
            count: 0,
            button_area: Cell::new(Rect::default()),
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        if let SketchEvent::Mouse(mouse) = event {
            if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                let area = self.button_area.get();
                if mouse.column >= area.x && mouse.column < area.x + area.width
                    && mouse.row >= area.y && mouse.row < area.y + area.height
                {
                    self.count += 1;
                }
            }
        }
        ControlFlow::Continue
    }

    fn render(&self, frame: &mut Frame) {
        let button_rect = centered_rect(20, 3, frame.area());
        self.button_area.set(button_rect);  // Cell allows mutation in &self
        // ... render button at button_rect
    }
}
```

## Tips

1. **Always handle 'q' or Ctrl+C** to allow users to exit
2. **Use Layout** to split areas into rows/columns
3. **Use centered_rect()** to center your UI
4. **Use `Cell<Rect>`** to store button bounds for click detection (required because render takes `&self`)
