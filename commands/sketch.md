---
description: Create an interactive terminal sketch
---

# Sketch Command

The user wants to create an interactive terminal sketch: "$ARGUMENTS"

If the user didn't specify what to create, ask them what kind of interactive terminal visualization they'd like.

## Instructions

1. Generate Python code following the template and API below
2. Write the sketch file to `.claude-sketch/sketches/<name>.py` using the Write tool
3. **ALWAYS run the sketch immediately after writing it** using the bash command below - never skip this step!

## Running Sketches

After writing the sketch file, run it with this bash command:

```bash
# For iTerm2 (opens in a split pane, closes on exit)
osascript -e '
tell application "iTerm"
    tell current session of current window
        set newSession to (split vertically with default profile)
    end tell
    tell newSession
        write text "cd \"'$(pwd)'\" && source .venv/bin/activate && exec env PYTHONPATH=src python3 .claude-sketch/sketches/<name>.py"
        select
    end tell
end tell
'

# For tmux (opens in a split pane, closes on exit)
tmux split-window -h "cd '$(pwd)' && source .venv/bin/activate && PYTHONPATH=src python3 .claude-sketch/sketches/<name>.py; exit"
```

Note: `exec` replaces the shell with Python, so when the sketch exits, the pane closes automatically.

Detect the terminal by checking environment variables:
- iTerm2: `$TERM_PROGRAM == "iTerm.app"` or `$LC_TERMINAL == "iTerm2"`
- tmux: `$TMUX` is set

## Required Template

All sketches MUST follow this exact structure:

```python
#!/usr/bin/env python3
from claude_sketch.runtime import SketchApp
from textual.app import ComposeResult
from textual.widgets import Static, Button
from textual.containers import Center, Vertical

class MySketch(SketchApp):
    """Description of this sketch."""

    CSS = """
    Screen {
        align: center middle;
    }
    """

    def compose(self) -> ComposeResult:
        """Define UI widgets here."""
        with Center():
            yield Static("Hello!")

if __name__ == "__main__":
    MySketch().run()
```

## API Reference

### SketchApp (inherit from this)
- Provides default 'q' and Escape key bindings to quit
- Enables dark mode by default

### Key Methods
- `compose(self) -> ComposeResult` - Define your UI widgets (REQUIRED)
- `on_mount(self)` - Called when app starts
- `on_button_pressed(self, event)` - Handle button clicks
- `on_key(self, event)` - Handle keyboard input
- `watch_<property>(self, value)` - Called when a reactive property changes

### Reactive State
```python
from textual.reactive import reactive

class MySketch(SketchApp):
    count: reactive[int] = reactive(0)

    def watch_count(self, count: int) -> None:
        # IMPORTANT: Guard against calls before widgets are mounted!
        try:
            self.query_one("#value", Static).update(str(count))
        except Exception:
            pass  # Widgets not mounted yet
```

### Available Widgets
From `textual.widgets`:
- `Static` - Text display
- `Button` - Clickable button (use `variant="primary"`, `"success"`, `"error"`)
- `Input` - Text input field
- `Checkbox` - Checkbox
- `Switch` - Toggle switch
- `DataTable` - Data table
- `ProgressBar` - Progress indicator
- `ListView` - Scrollable list
- `Tree` - Tree view
- `Tabs` - Tabbed interface

### Available Containers
From `textual.containers`:
- `Vertical` - Stack vertically
- `Horizontal` - Stack horizontally
- `Center` - Center contents
- `Grid` - Grid layout
- `ScrollableContainer` - Scrollable area

### CSS Styling
```python
CSS = """
Screen {
    align: center middle;
}

#my-id {
    color: cyan;
    text-style: bold;
    border: solid green;
    padding: 1 2;
}

.my-class {
    margin: 1;
}

Button {
    margin: 0 1;
}
"""
```

Common properties: `color`, `background`, `text-style`, `text-align`, `align`, `width`, `height`, `margin`, `padding`, `border`

### Event Handling
```python
def on_button_pressed(self, event: Button.Pressed) -> None:
    if event.button.id == "my-button":
        # handle click

def on_key(self, event) -> None:
    if event.key == "up":
        # handle key

def on_input_submitted(self, event: Input.Submitted) -> None:
    # handle enter in input
```

## Managing Sketches

Sketches are stored in `.claude-sketch/sketches/`.

- **List**: `ls .claude-sketch/sketches/`
- **Delete one**: `rm .claude-sketch/sketches/<name>.py`
- **Delete all**: `rm -rf .claude-sketch/sketches/*`

## Tips

1. **'q' and Escape exit by default** - SketchApp includes these bindings
2. **Use reactive properties** for state that updates the UI
3. **Widgets handle their own events** - no manual hit detection needed
4. **Use CSS for styling** - cleaner than inline styles
5. **Containers for layout** - Vertical, Horizontal, Center
6. **Guard watch methods** - Always wrap `query_one()` in try/except in watch methods, as they can be called before widgets are mounted
7. **Widget IDs must be unique** - Never reuse the same `id=` value for multiple widgets
8. **Keep layouts compact (~20 rows)** - For complex UIs, use `ScrollableContainer` to handle overflow
9. **Prefer flat layouts** - Deeply nested containers with `height: auto` can cause layout errors
