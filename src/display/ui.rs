use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction::Horizontal, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use super::state::UIState;

///
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

pub fn draw(state: &UIState, f: &mut Frame) {
    let l = Layout::default()
        .constraints(vec![Constraint::Length(2), Constraint::Percentage(99)])
        .split(f.size());

    let (header_area, area) = (l[0], l[1]);

    draw_header(state, f, header_area);

    f.render_widget(Block::default().title("asdf").borders(Borders::ALL), area);
}

fn draw_header(state: &UIState, f: &mut Frame, area: Rect) {
    // Draw header bg and outer styling elements
    let header_block = Block::default().borders(Borders::BOTTOM);

    let header_area = header_block.inner(area);

    f.render_widget(header_block, area);

    // Split layout
    let l = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Constraint::Percentage(99), Constraint::Min(20)])
        .split(header_area);

    let (title_area, tab_area) = (l[0], l[1]);

    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled("MAINFRAME", Style::new().bold()),
        Span::styled("  ", Style::new()),
        Span::styled(
            format!("v{}", env!("CARGO_PKG_VERSION")),
            Style::new().dim(),
        ),
    ])])
    .alignment(ratatui::layout::Alignment::Left);

    let tabs = Tabs::new(vec!["Home", "Usage", "Disks", "Help"])
        .select(state.current_tab)
        .padding(" ", " ");

    f.render_widget(title, title_area);
    f.render_widget(tabs, tab_area);
}
