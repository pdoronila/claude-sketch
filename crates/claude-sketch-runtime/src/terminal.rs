//! Terminal setup, cleanup, and main event loop

use std::io::{self, stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{ControlFlow, SketchApp};
use crate::events::SketchEvent;

/// Poll timeout for events (in milliseconds)
const POLL_TIMEOUT_MS: u64 = 100;

/// Run a sketch application
///
/// This function:
/// 1. Sets up the terminal (raw mode, alternate screen, mouse capture)
/// 2. Runs the main event loop
/// 3. Cleans up the terminal on exit (even on panic)
///
/// # Example
///
/// ```ignore
/// use claude_sketch_runtime::prelude::*;
///
/// struct MyApp { /* ... */ }
///
/// impl SketchApp for MyApp {
///     fn new() -> Self { /* ... */ }
///     fn update(&mut self, event: SketchEvent) -> ControlFlow { /* ... */ }
///     fn render(&self, frame: &mut Frame) { /* ... */ }
/// }
///
/// fn main() -> Result<()> {
///     run_sketch::<MyApp>()
/// }
/// ```
pub fn run_sketch<A: SketchApp>() -> Result<()> {
    // Set up panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));

    // Initialize terminal
    let mut terminal = setup_terminal()?;

    // Create and initialize the app
    let mut app = A::new();
    app.init();

    // Main event loop
    let result = run_event_loop(&mut terminal, &mut app);

    // Cleanup
    app.cleanup();
    restore_terminal()?;

    result
}

/// Set up the terminal for TUI rendering
fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

/// Run the main event loop
fn run_event_loop<A: SketchApp>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut A,
) -> Result<()> {
    loop {
        // Render the current state
        terminal.draw(|frame| app.render(frame))?;

        // Poll for events with timeout
        if event::poll(Duration::from_millis(POLL_TIMEOUT_MS))? {
            let event = event::read()?;

            // Convert to SketchEvent and let app handle it
            let sketch_event = SketchEvent::from(event);

            match app.update(sketch_event) {
                ControlFlow::Continue => {}
                ControlFlow::Break => break,
            }
        }
    }

    Ok(())
}
