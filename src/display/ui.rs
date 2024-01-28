use std::{
    error::Error,
    io::{self, stdout, Stdout},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
};

/// Setup the necessary components to make terminal ui calls.
pub fn init_ui() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

/// Teardown ui components, and release the terminal back to the user.
pub fn shutdown_ui() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_ui(f: &mut Frame) {
    // Render block header
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" MAINFRAME v{} ", env!("CARGO_PKG_VERSION")))
            .title_style(Style::new().add_modifier(Modifier::BOLD)),
        f.size(),
    );
}

/// Poll ui events, and check if the user signalled application close.
pub fn ui_should_close() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
