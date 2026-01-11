//! Counter Sketch Example
//!
//! A simple counter with increment and decrement buttons.
//! Demonstrates both keyboard and mouse interaction.

use claude_sketch_runtime::prelude::*;

struct CounterApp {
    count: i64,
    inc_button: Button,
    dec_button: Button,
}

impl SketchApp for CounterApp {
    fn new() -> Self {
        Self {
            count: 0,
            inc_button: Button::new("[+] Increment"),
            dec_button: Button::new("[-] Decrement"),
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        match event {
            // Keyboard events
            SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => ControlFlow::Break,

            SketchEvent::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('+'),
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('='),
                ..
            }) => {
                self.count += 1;
                ControlFlow::Continue
            }

            SketchEvent::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('-'),
                ..
            }) => {
                self.count -= 1;
                ControlFlow::Continue
            }

            // Mouse click events
            SketchEvent::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) => {
                if self.inc_button.contains(column, row) {
                    self.count += 1;
                } else if self.dec_button.contains(column, row) {
                    self.count -= 1;
                }
                ControlFlow::Continue
            }

            _ => ControlFlow::Continue,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let center = centered_rect(50, 14, area);

        let layout = Layout::vertical([
            Constraint::Length(2), // Title
            Constraint::Length(3), // Counter display
            Constraint::Length(3), // Buttons row
            Constraint::Length(2), // Help text
            Constraint::Min(0),
        ])
        .split(center);

        // Title
        let title = Paragraph::new("Counter Sketch")
            .style(Style::default().bold())
            .alignment(Alignment::Center);
        frame.render_widget(title, layout[0]);

        // Counter display
        let counter = Paragraph::new(format!("{}", self.count))
            .style(Style::default().fg(Color::Cyan).bold())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Value"));
        frame.render_widget(counter, layout[1]);

        // Buttons row
        let button_layout = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(layout[2]);

        // We need mutable access to render buttons (to update their bounds)
        // This is a slight awkwardness - in real code you'd use interior mutability
        // For now, we'll create temporary buttons for rendering
        let mut dec_button = self.dec_button.clone();
        let mut inc_button = self.inc_button.clone();

        dec_button.render(frame, button_layout[0]);
        inc_button.render(frame, button_layout[1]);

        // Instructions
        let help = Paragraph::new("Click buttons or use [+/-] keys | [q/Esc] to quit")
            .style(Style::default().dim())
            .alignment(Alignment::Center);
        frame.render_widget(help, layout[3]);
    }
}

// We need to handle button bounds properly - using a RefCell pattern
// For the MVP, let's use a simpler approach with tracked areas
use std::cell::RefCell;

struct CounterAppWithTracking {
    count: i64,
    inc_bounds: RefCell<Option<Rect>>,
    dec_bounds: RefCell<Option<Rect>>,
}

impl SketchApp for CounterAppWithTracking {
    fn new() -> Self {
        Self {
            count: 0,
            inc_bounds: RefCell::new(None),
            dec_bounds: RefCell::new(None),
        }
    }

    fn update(&mut self, event: SketchEvent) -> ControlFlow {
        match event {
            // Keyboard events
            SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => ControlFlow::Break,

            SketchEvent::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('+'),
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('='),
                ..
            }) => {
                self.count += 1;
                ControlFlow::Continue
            }

            SketchEvent::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | SketchEvent::Key(KeyEvent {
                code: KeyCode::Char('-'),
                ..
            }) => {
                self.count -= 1;
                ControlFlow::Continue
            }

            // Mouse click events
            SketchEvent::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) => {
                let contains = |bounds: &Option<Rect>, x: u16, y: u16| -> bool {
                    if let Some(b) = bounds {
                        x >= b.x && x < b.x + b.width && y >= b.y && y < b.y + b.height
                    } else {
                        false
                    }
                };

                if contains(&*self.inc_bounds.borrow(), column, row) {
                    self.count += 1;
                } else if contains(&*self.dec_bounds.borrow(), column, row) {
                    self.count -= 1;
                }
                ControlFlow::Continue
            }

            _ => ControlFlow::Continue,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let center = centered_rect(50, 14, area);

        let layout = Layout::vertical([
            Constraint::Length(2), // Title
            Constraint::Length(3), // Counter display
            Constraint::Length(3), // Buttons row
            Constraint::Length(2), // Help text
            Constraint::Min(0),
        ])
        .split(center);

        // Title
        let title = Paragraph::new("Counter Sketch")
            .style(Style::default().bold())
            .alignment(Alignment::Center);
        frame.render_widget(title, layout[0]);

        // Counter display
        let counter = Paragraph::new(format!("{}", self.count))
            .style(Style::default().fg(Color::Cyan).bold())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Value"));
        frame.render_widget(counter, layout[1]);

        // Buttons row
        let button_layout = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(layout[2]);

        // Store bounds for click detection
        *self.dec_bounds.borrow_mut() = Some(button_layout[0]);
        *self.inc_bounds.borrow_mut() = Some(button_layout[1]);

        // Render decrement button
        let dec_btn = Paragraph::new("[-] Decrement")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(dec_btn, button_layout[0]);

        // Render increment button
        let inc_btn = Paragraph::new("[+] Increment")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(inc_btn, button_layout[1]);

        // Instructions
        let help = Paragraph::new("Click buttons or use [+/-] keys | [q/Esc] to quit")
            .style(Style::default().dim())
            .alignment(Alignment::Center);
        frame.render_widget(help, layout[3]);
    }
}

fn main() -> Result<()> {
    run_sketch::<CounterAppWithTracking>()
}
