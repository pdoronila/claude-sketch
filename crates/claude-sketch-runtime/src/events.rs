//! Event types for sketch interaction

use crossterm::event::{Event, KeyEvent, MouseEvent};

/// Events that sketches can handle
#[derive(Debug, Clone)]
pub enum SketchEvent {
    /// Keyboard input event
    Key(KeyEvent),
    /// Mouse input event (click, scroll, move)
    Mouse(MouseEvent),
    /// Terminal was resized
    Resize(u16, u16),
    /// A tick event for animations (if enabled)
    Tick,
}

impl From<Event> for SketchEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(key) => SketchEvent::Key(key),
            Event::Mouse(mouse) => SketchEvent::Mouse(mouse),
            Event::Resize(width, height) => SketchEvent::Resize(width, height),
            // Map other events to Tick for simplicity
            _ => SketchEvent::Tick,
        }
    }
}
