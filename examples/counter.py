#!/usr/bin/env python3
"""Counter Sketch - demonstrates reactive state and button clicks.

This example shows:
- Inheriting from SketchApp
- Using reactive properties for state
- Handling button clicks
- Handling keyboard input
- CSS styling
"""
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
