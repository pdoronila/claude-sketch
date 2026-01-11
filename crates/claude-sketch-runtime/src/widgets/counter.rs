//! Counter widget for numeric values

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// A counter widget that displays and manages a numeric value
#[derive(Debug, Clone)]
pub struct Counter {
    /// Current value
    value: i64,
    /// Minimum allowed value (if any)
    min: Option<i64>,
    /// Maximum allowed value (if any)
    max: Option<i64>,
    /// Step size for increment/decrement
    step: i64,
    /// Label to display above the value
    label: Option<String>,
    /// Style for the value display
    value_style: Style,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    /// Create a new counter starting at 0
    pub fn new() -> Self {
        Self {
            value: 0,
            min: None,
            max: None,
            step: 1,
            label: None,
            value_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }

    /// Create a new counter with an initial value
    pub fn with_value(value: i64) -> Self {
        Self {
            value,
            ..Self::new()
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min: i64) -> Self {
        self.min = Some(min);
        self.value = self.value.max(min);
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: i64) -> Self {
        self.max = Some(max);
        self.value = self.value.min(max);
        self
    }

    /// Set the step size
    pub fn step(mut self, step: i64) -> Self {
        self.step = step;
        self
    }

    /// Set the label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the value style
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = style;
        self
    }

    /// Get the current value
    pub fn value(&self) -> i64 {
        self.value
    }

    /// Set the value directly (respecting min/max)
    pub fn set_value(&mut self, value: i64) {
        self.value = self.clamp(value);
    }

    /// Increment the counter by step
    pub fn increment(&mut self) {
        self.value = self.clamp(self.value.saturating_add(self.step));
    }

    /// Decrement the counter by step
    pub fn decrement(&mut self) {
        self.value = self.clamp(self.value.saturating_sub(self.step));
    }

    /// Clamp value to min/max bounds
    fn clamp(&self, value: i64) -> i64 {
        let mut v = value;
        if let Some(min) = self.min {
            v = v.max(min);
        }
        if let Some(max) = self.max {
            v = v.min(max);
        }
        v
    }

    /// Render the counter to the frame at the given area
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut block = Block::default().borders(Borders::ALL);

        if let Some(ref label) = self.label {
            block = block.title(label.as_str());
        }

        let paragraph = Paragraph::new(format!("{}", self.value))
            .style(self.value_style)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment_decrement() {
        let mut counter = Counter::new();
        assert_eq!(counter.value(), 0);

        counter.increment();
        assert_eq!(counter.value(), 1);

        counter.decrement();
        assert_eq!(counter.value(), 0);

        counter.decrement();
        assert_eq!(counter.value(), -1);
    }

    #[test]
    fn test_counter_with_bounds() {
        let mut counter = Counter::new().min(0).max(10);

        counter.set_value(5);
        assert_eq!(counter.value(), 5);

        counter.set_value(-5);
        assert_eq!(counter.value(), 0); // Clamped to min

        counter.set_value(15);
        assert_eq!(counter.value(), 10); // Clamped to max
    }

    #[test]
    fn test_counter_step() {
        let mut counter = Counter::new().step(5);

        counter.increment();
        assert_eq!(counter.value(), 5);

        counter.increment();
        assert_eq!(counter.value(), 10);
    }
}
