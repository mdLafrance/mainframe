use std::{
    error::Error,
    io::{self, stdout, Stdout},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use human_bytes::human_bytes;
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Row, Table, Wrap,
    },
};

use crate::monitoring::system::SystemInformation;

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

/// Draw a block outline of the terminal, and display a title of the app.
///
/// Returns the drawable rect within the bounds of the outline.
pub fn draw_app_outline(f: &mut Frame) -> Rect {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(4, 4, 2, 2))
        .title(format!(" MAINFRAME v{} ", env!("CARGO_PKG_VERSION")))
        .title_style(Style::new().add_modifier(Modifier::BOLD));

    let mut block_area = block.inner(f.size());

    // Render block header
    f.render_widget(block, f.size());

    return block_area;
}

pub fn draw_sys_info(s: &SystemInformation, f: &mut Frame, area: Rect) {
    let style_category = |s: String| Span::styled(s, Style::new().add_modifier(Modifier::BOLD));
    let style_value = |s: String| Span::styled(s, Style::new().add_modifier(Modifier::ITALIC));

    let text = vec![
        Line::from(vec![
            style_category("Host Name:           ".into()),
            style_value(s.host_name.clone()),
        ]),
        Line::from(vec![
            style_category("Operating System:    ".into()),
            style_value(s.os.clone()),
        ]),
        Line::from(vec![
            style_category("OS Version:          ".into()),
            style_value(s.os_version.clone()),
        ]),
        Line::from(vec![
            style_category("Kernel Version:      ".into()),
            style_value(s.kernel_version.clone()),
        ]),
        Line::from(vec![
            style_category("CPU count:           ".into()),
            style_value(s.logical_processors.to_string()),
        ]),
        Line::from(vec![
            style_category("Core count:          ".into()),
            style_value(s.physical_processors.to_string()),
        ]),
        Line::from(vec![
            style_category("Total Memory:        ".into()),
            style_value(human_bytes(s.total_memory as f64)),
        ]),
    ];

    let p = Paragraph::new(text)
        .block(
            Block::new()
                .title(" System Information ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 1)),
        )
        .style(Style::new().white().on_black());

    f.render_widget(p, area)
}
