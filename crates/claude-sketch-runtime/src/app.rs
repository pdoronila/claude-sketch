//! Application trait and control flow for sketches

use ratatui::layout::Rect;
use ratatui::Frame;

use crate::events::SketchEvent;

/// Control flow returned from update to indicate whether to continue or quit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    /// Continue running the sketch
    Continue,
    /// Stop the sketch and exit
    Break,
}

/// The main trait that all sketches must implement
pub trait SketchApp: Sized {
    /// Create a new instance of the sketch application
    fn new() -> Self;

    /// Handle an event and update state
    ///
    /// Return `ControlFlow::Break` to exit the sketch,
    /// or `ControlFlow::Continue` to keep running.
    fn update(&mut self, event: SketchEvent) -> ControlFlow;

    /// Render the current state to the terminal frame
    fn render(&self, frame: &mut Frame);

    /// Optional: Called once before the main loop starts
    fn init(&mut self) {}

    /// Optional: Called once after the main loop ends (for cleanup)
    fn cleanup(&mut self) {}
}

/// Helper function to create a centered rectangle of given size within an area
pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;

    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(20, 10, area);

        assert_eq!(centered.x, 40);
        assert_eq!(centered.y, 20);
        assert_eq!(centered.width, 20);
        assert_eq!(centered.height, 10);
    }

    #[test]
    fn test_centered_rect_larger_than_area() {
        let area = Rect::new(0, 0, 10, 5);
        let centered = centered_rect(20, 10, area);

        // Should be clamped to area size
        assert_eq!(centered.width, 10);
        assert_eq!(centered.height, 5);
    }
}
