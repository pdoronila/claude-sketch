//! Claude Sketch Runtime
//!
//! A library for building interactive terminal sketches with ratatui.
//! This library provides the core traits and utilities that Claude-generated
//! sketches use to create interactive terminal UIs.

pub mod app;
pub mod events;
pub mod terminal;
pub mod widgets;

pub use app::{ControlFlow, SketchApp};
pub use events::SketchEvent;
pub use terminal::run_sketch;

/// Prelude module for convenient imports in generated sketches
pub mod prelude {
    pub use crate::app::{centered_rect, ControlFlow, SketchApp};
    pub use crate::events::SketchEvent;
    pub use crate::terminal::run_sketch;
    pub use crate::widgets::{Button, Counter, TextInput};

    // Re-export commonly used crossterm types
    pub use crossterm::event::{
        KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    };

    // Re-export commonly used ratatui types
    pub use ratatui::layout::{Alignment, Constraint, Layout, Rect};
    pub use ratatui::style::{Color, Modifier, Style, Stylize};
    pub use ratatui::widgets::{Block, Borders, Paragraph};
    pub use ratatui::Frame;

    // Re-export anyhow for error handling
    pub use anyhow::Result;
}
