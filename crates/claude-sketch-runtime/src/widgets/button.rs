//! Button widget with mouse click support

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// State of a button (for visual feedback)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Normal,
    Hovered,
    Pressed,
}

/// An interactive button widget with mouse support
#[derive(Debug, Clone)]
pub struct Button {
    /// The button label text
    label: String,
    /// Current button state
    state: ButtonState,
    /// The button's bounding rectangle (set after rendering)
    bounds: Option<Rect>,
    /// Style for normal state
    normal_style: Style,
    /// Style for hovered state
    hover_style: Style,
    /// Style for pressed state
    pressed_style: Style,
}

impl Button {
    /// Create a new button with the given label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            state: ButtonState::Normal,
            bounds: None,
            normal_style: Style::default().fg(Color::White),
            hover_style: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            pressed_style: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        }
    }

    /// Set the normal state style
    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }

    /// Set the hover state style
    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }

    /// Set the pressed state style
    pub fn pressed_style(mut self, style: Style) -> Self {
        self.pressed_style = style;
        self
    }

    /// Get the button's label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Set the button's label
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Get the current button state
    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// Set the button state
    pub fn set_state(&mut self, state: ButtonState) {
        self.state = state;
    }

    /// Get the button's bounds (if it has been rendered)
    pub fn bounds(&self) -> Option<Rect> {
        self.bounds
    }

    /// Check if the given coordinates are within the button's bounds
    pub fn contains(&self, x: u16, y: u16) -> bool {
        if let Some(bounds) = self.bounds {
            x >= bounds.x
                && x < bounds.x + bounds.width
                && y >= bounds.y
                && y < bounds.y + bounds.height
        } else {
            false
        }
    }

    /// Render the button to the frame at the given area
    ///
    /// This also updates the button's bounds for hit detection.
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Store bounds for click detection
        self.bounds = Some(area);

        // Select style based on state
        let style = match self.state {
            ButtonState::Normal => self.normal_style,
            ButtonState::Hovered => self.hover_style,
            ButtonState::Pressed => self.pressed_style,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(style);

        let paragraph = Paragraph::new(self.label.as_str())
            .style(style)
            .alignment(ratatui::layout::Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_contains() {
        let mut button = Button::new("Test");
        button.bounds = Some(Rect::new(10, 5, 20, 3));

        assert!(button.contains(10, 5)); // Top-left corner
        assert!(button.contains(29, 7)); // Bottom-right corner (exclusive, so 29 is valid)
        assert!(!button.contains(9, 5)); // Just outside left
        assert!(!button.contains(30, 5)); // Just outside right
        assert!(!button.contains(10, 4)); // Just outside top
        assert!(!button.contains(10, 8)); // Just outside bottom
    }

    #[test]
    fn test_button_no_bounds() {
        let button = Button::new("Test");
        assert!(!button.contains(0, 0)); // Should return false when no bounds set
    }
}
