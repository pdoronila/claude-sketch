//! Text input widget

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// A text input widget
#[derive(Debug, Clone)]
pub struct TextInput {
    /// Current text value
    value: String,
    /// Cursor position (byte index)
    cursor: usize,
    /// Whether the input is focused
    focused: bool,
    /// Placeholder text when empty
    placeholder: Option<String>,
    /// Maximum length (if any)
    max_length: Option<usize>,
    /// Style when focused
    focused_style: Style,
    /// Style when unfocused
    unfocused_style: Style,
    /// Style for placeholder text
    placeholder_style: Style,
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl TextInput {
    /// Create a new empty text input
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            focused: false,
            placeholder: None,
            max_length: None,
            focused_style: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            unfocused_style: Style::default().fg(Color::Gray),
            placeholder_style: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        }
    }

    /// Create a text input with initial value
    pub fn with_value(value: impl Into<String>) -> Self {
        let value = value.into();
        let cursor = value.len();
        Self {
            value,
            cursor,
            ..Self::new()
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// Set focused style
    pub fn focused_style(mut self, style: Style) -> Self {
        self.focused_style = style;
        self
    }

    /// Set unfocused style
    pub fn unfocused_style(mut self, style: Style) -> Self {
        self.unfocused_style = style;
        self
    }

    /// Get the current value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the value
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        if let Some(max) = self.max_length {
            self.value.truncate(max);
        }
        self.cursor = self.cursor.min(self.value.len());
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Focus the input
    pub fn focus(&mut self) {
        self.focused = true;
    }

    /// Unfocus the input
    pub fn blur(&mut self) {
        self.focused = false;
    }

    /// Handle a key event (returns true if the event was consumed)
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if !self.focused {
            return false;
        }

        match key.code {
            KeyCode::Char(c) => {
                // Check max length
                if let Some(max) = self.max_length {
                    if self.value.len() >= max {
                        return true;
                    }
                }

                // Handle Ctrl+A to select all (just move cursor to end for now)
                if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'a' {
                    self.cursor = self.value.len();
                    return true;
                }

                // Insert character at cursor
                self.value.insert(self.cursor, c);
                self.cursor += c.len_utf8();
                true
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    // Find the previous character boundary
                    let prev = self.value[..self.cursor]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    self.value.remove(prev);
                    self.cursor = prev;
                }
                true
            }
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                }
                true
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    // Find the previous character boundary
                    self.cursor = self.value[..self.cursor]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                }
                true
            }
            KeyCode::Right => {
                if self.cursor < self.value.len() {
                    // Find the next character boundary
                    self.cursor = self.value[self.cursor..]
                        .char_indices()
                        .nth(1)
                        .map(|(i, _)| self.cursor + i)
                        .unwrap_or(self.value.len());
                }
                true
            }
            KeyCode::Home => {
                self.cursor = 0;
                true
            }
            KeyCode::End => {
                self.cursor = self.value.len();
                true
            }
            _ => false,
        }
    }

    /// Render the text input to the frame at the given area
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let style = if self.focused {
            self.focused_style
        } else {
            self.unfocused_style
        };

        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style);

        // Display value or placeholder
        let display_text = if self.value.is_empty() {
            self.placeholder.as_deref().unwrap_or("")
        } else {
            &self.value
        };

        let text_style = if self.value.is_empty() && self.placeholder.is_some() {
            self.placeholder_style
        } else {
            style
        };

        // Add cursor indicator when focused
        let text = if self.focused && !self.value.is_empty() {
            // Insert cursor character at cursor position
            let (before, after) = self.value.split_at(self.cursor);
            format!("{}|{}", before, after)
        } else if self.focused && self.value.is_empty() {
            "|".to_string()
        } else {
            display_text.to_string()
        };

        let paragraph = Paragraph::new(text)
            .style(text_style)
            .alignment(Alignment::Left)
            .block(block);

        frame.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_typing() {
        let mut input = TextInput::new();
        input.set_focused(true);

        input.handle_key(KeyEvent::from(KeyCode::Char('h')));
        input.handle_key(KeyEvent::from(KeyCode::Char('i')));

        assert_eq!(input.value(), "hi");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_input_backspace() {
        let mut input = TextInput::with_value("hello");
        input.set_focused(true);

        input.handle_key(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(input.value(), "hell");
    }

    #[test]
    fn test_input_cursor_movement() {
        let mut input = TextInput::with_value("hello");
        input.set_focused(true);

        input.handle_key(KeyEvent::from(KeyCode::Home));
        assert_eq!(input.cursor, 0);

        input.handle_key(KeyEvent::from(KeyCode::Right));
        assert_eq!(input.cursor, 1);

        input.handle_key(KeyEvent::from(KeyCode::End));
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_input_max_length() {
        let mut input = TextInput::new().max_length(5);
        input.set_focused(true);

        for c in "hello world".chars() {
            input.handle_key(KeyEvent::from(KeyCode::Char(c)));
        }

        assert_eq!(input.value(), "hello");
    }
}
