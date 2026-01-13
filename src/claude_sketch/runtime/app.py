"""SketchApp base class for Claude Sketch.

All sketches inherit from SketchApp, which provides sensible defaults
and common functionality for interactive terminal visualizations.
"""

from textual.app import App
from textual.binding import Binding


class SketchApp(App):
    """Base class for Claude sketches using Textual.

    Inherits from textual.App and provides:
    - Default 'q' and Escape key bindings to quit
    - Dark mode enabled by default
    - CSS support for styling

    Usage:
        from claude_sketch.runtime import SketchApp
        from textual.app import ComposeResult
        from textual.widgets import Static

        class MySketch(SketchApp):
            def compose(self) -> ComposeResult:
                yield Static("Hello, Sketch!")

        if __name__ == "__main__":
            MySketch().run()
    """

    # Default key bindings - sketches can extend this
    BINDINGS = [
        Binding("q", "quit", "Quit", show=True),
        Binding("escape", "quit", "Quit", show=False),
    ]

    # Enable dark mode by default
    ENABLE_COMMAND_PALETTE = False

    # Default CSS - sketches can override with their own CSS class variable
    CSS = """
    Screen {
        align: center middle;
    }
    """

    def __init__(self, *args, **kwargs):
        """Initialize the sketch app."""
        super().__init__(*args, **kwargs)
        # Force dark mode for consistent appearance
        self.dark = True
