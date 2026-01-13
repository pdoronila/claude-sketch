---
name: sketch
description: Create interactive terminal visualizations using Python and Textual. Use when the user asks to create a TUI, terminal UI, or interactive sketch.
---

# Claude Sketch Skill

Create interactive terminal visualizations using Python and Textual.

## When to Use

Use this skill when the user asks you to:
- Create an interactive terminal UI or visualization
- Build a TUI (terminal user interface) application
- Make a "sketch" or visual demo in the terminal
- Create something like a Claude artifact but in the terminal

## How It Works

1. **Write the sketch** to `.claude-sketch/sketches/<name>.py` using the Write tool
2. **Run the sketch** using bash to open it in a terminal pane (iTerm2 or tmux)
3. **Update sketches** by writing the file again and re-running

## Managing Sketches

Sketches are stored in `.claude-sketch/sketches/` in the current working directory.

### List all sketches
```bash
ls .claude-sketch/sketches/
```

### Delete a specific sketch
```bash
rm .claude-sketch/sketches/<name>.py
```

### Delete all sketches
```bash
rm -rf .claude-sketch/sketches/*
```

## Running Sketches

Detect the terminal and run appropriately. Use `exec` so the pane closes when the sketch exits.

### iTerm2 (check: `$TERM_PROGRAM == "iTerm.app"`)
```bash
osascript -e '
tell application "iTerm"
    tell current session of current window
        set newSession to (split vertically with default profile)
    end tell
    tell newSession
        write text "cd \"'"$(pwd)"'\" && source .venv/bin/activate && exec env PYTHONPATH=src python3 .claude-sketch/sketches/<name>.py"
        select
    end tell
end tell
'
```

### tmux (check: `$TMUX` is set)
```bash
tmux split-window -h "cd '$(pwd)' && source .venv/bin/activate && PYTHONPATH=src python3 .claude-sketch/sketches/<name>.py; exit"
```

## Sketch Template

All sketches inherit from `SketchApp`:

```python
#!/usr/bin/env python3
from claude_sketch.runtime import SketchApp
from textual.app import ComposeResult
from textual.widgets import Static, Button
from textual.containers import Center, Vertical
from textual.reactive import reactive

class MySketch(SketchApp):
    """Description of what this sketch does."""

    CSS = """
    Screen {
        align: center middle;
    }
    """

    def compose(self) -> ComposeResult:
        """Define UI widgets here."""
        with Center():
            yield Static("Hello, Sketch!")

if __name__ == "__main__":
    MySketch().run()
```

## Available from Textual

### Widgets
- `Static` - Text display
- `Button` - Clickable button
- `Input` - Text input field
- `Label` - Simple text label
- `DataTable` - Tables with rows/columns
- `ProgressBar` - Progress indicator
- `ListView` - Scrollable list
- `Tree` - Hierarchical tree view
- `Tabs` - Tabbed interface
- `Checkbox` - Checkbox input
- `Switch` - Toggle switch
- `RadioSet`, `RadioButton` - Radio buttons
- `Select` - Dropdown selection
- `TextArea` - Multi-line text input

### Containers
- `Vertical` - Stack widgets vertically
- `Horizontal` - Stack widgets horizontally
- `Center` - Center contents
- `Grid` - Grid layout
- `ScrollableContainer` - Scrollable area
- `Container` - Generic container

### Styling with CSS

Textual uses CSS-like syntax for styling:

```python
CSS = """
Screen {
    align: center middle;
}

#title {
    text-style: bold;
    color: cyan;
}

.button-row {
    height: 3;
    align: center middle;
}

Button {
    margin: 0 1;
}

Button:hover {
    background: $accent;
}
"""
```

Common CSS properties:
- `color` - Text color (red, green, blue, cyan, magenta, yellow, white, etc.)
- `background` - Background color
- `text-style` - bold, italic, underline, strike
- `text-align` - left, center, right
- `align` - Alignment within container (center middle, left top, etc.)
- `width`, `height` - Size (auto, 100%, 50, etc.)
- `margin`, `padding` - Spacing
- `border` - Border style (solid, dashed, etc.)

### Reactive State

Use `reactive` for state that should trigger UI updates:

```python
from textual.reactive import reactive

class MySketch(SketchApp):
    # Reactive properties - UI updates automatically when these change
    count: reactive[int] = reactive(0)
    name: reactive[str] = reactive("")

    def watch_count(self, count: int) -> None:
        """Called automatically when count changes."""
        # IMPORTANT: Guard against calls before widgets are mounted!
        # Watch methods can be called during __init__ before compose() runs
        try:
            self.query_one("#value", Static).update(str(count))
        except Exception:
            pass  # Widgets not mounted yet
```

### Event Handlers

```python
def on_button_pressed(self, event: Button.Pressed) -> None:
    """Handle button clicks."""
    if event.button.id == "increment":
        self.count += 1

def on_key(self, event) -> None:
    """Handle keyboard input."""
    if event.key == "up":
        self.count += 1
    elif event.key == "down":
        self.count -= 1

def on_input_changed(self, event: Input.Changed) -> None:
    """Handle text input changes."""
    self.name = event.value

def on_mount(self) -> None:
    """Called when app starts - good for initialization."""
    pass
```

## Example: Counter with Buttons

```python
#!/usr/bin/env python3
from claude_sketch.runtime import SketchApp
from textual.app import ComposeResult
from textual.widgets import Static, Button
from textual.containers import Center, Vertical, Horizontal
from textual.reactive import reactive

class CounterSketch(SketchApp):
    """A simple counter with increment and decrement buttons."""

    CSS = """
    Screen {
        align: center middle;
    }

    #counter-box {
        width: 50;
        height: auto;
        border: solid green;
        padding: 1 2;
    }

    #title {
        text-align: center;
        text-style: bold;
        margin-bottom: 1;
    }

    #value {
        text-align: center;
        color: cyan;
        text-style: bold;
        margin: 1 0;
    }

    .button-row {
        align: center middle;
        height: 3;
    }

    Button {
        margin: 0 1;
    }

    #help {
        text-align: center;
        color: $text-muted;
        margin-top: 1;
    }
    """

    count: reactive[int] = reactive(0)

    def compose(self) -> ComposeResult:
        with Center():
            with Vertical(id="counter-box"):
                yield Static("Counter Sketch", id="title")
                yield Static(str(self.count), id="value")
                with Horizontal(classes="button-row"):
                    yield Button("[-] Decrement", id="dec", variant="error")
                    yield Button("[+] Increment", id="inc", variant="success")
                yield Static("Click buttons or use +/- keys | q to quit", id="help")

    def watch_count(self, count: int) -> None:
        """Update display when count changes."""
        self.query_one("#value", Static).update(str(count))

    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Handle button clicks."""
        if event.button.id == "inc":
            self.count += 1
        elif event.button.id == "dec":
            self.count -= 1

    def on_key(self, event) -> None:
        """Handle keyboard input."""
        if event.key in ("+", "="):
            self.count += 1
        elif event.key == "-":
            self.count -= 1

if __name__ == "__main__":
    CounterSketch().run()
```

## Terminal Support

Sketches run in a new terminal pane:
- **iTerm2** - Split pane to the right
- **tmux** - Horizontal split
- Other terminals - Run directly in current terminal

## Tips

1. **Press 'q' or Escape to exit** - SketchApp includes this by default
2. **Use reactive properties** for state that affects the UI
3. **Use CSS** for styling instead of inline styles
4. **Widgets handle their own clicks** - no need for manual hit detection!
5. **Use containers** (Vertical, Horizontal, Center) for layout
6. **Guard watch methods** - Always wrap `query_one()` in try/except in watch methods, as they can be called before widgets are mounted
7. **Widget IDs must be unique** - Never reuse the same `id=` value for multiple widgets or you'll get a MountError
8. **Keep layouts compact** - Terminal panes are typically 20-30 rows. For complex UIs that might exceed this, use `ScrollableContainer`:
   ```python
   from textual.containers import ScrollableContainer

   def compose(self):
       with ScrollableContainer():
           # Content that might overflow
   ```
9. **Prefer flat layouts** - Deeply nested containers with `height: auto` can cause layout errors when content exceeds terminal size. Use fewer nesting levels when possible.
