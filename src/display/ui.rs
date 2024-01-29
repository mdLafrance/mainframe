use human_bytes::human_bytes;
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
    layout::{
        Constraint,
        Direction::{self, Horizontal},
        Layout, Rect,
    },
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Padding, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::monitoring::system::{DiskInformation, SystemData, SystemInformation};

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

pub fn draw(state: &UIState, data: &SystemData, f: &mut Frame) {
    let l = Layout::default()
        .constraints(vec![Constraint::Length(2), Constraint::Percentage(99)])
        .split(f.size());

    let (header_area, area) = (l[0], l[1]);

    draw_header(state, f, header_area);

    // Draw page according to tab
    match state.current_tab {
        1 => draw_usage_page(&data, f, area),
        2 => draw_disk_page(&data, f, area),
        _ => draw_home_page(&data, f, area),
    }
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

fn draw_home_page(data: &SystemData, f: &mut Frame, area: Rect) {
    draw_sys_info(&data.info, f, area);
}

fn draw_usage_page(data: &SystemData, f: &mut Frame, area: Rect) {}

fn draw_disk_page(data: &SystemData, f: &mut Frame, area: Rect) {}

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

pub fn display_disk_info(d: &DiskInformation, f: &mut Frame, area: Rect) {
    // Surround display information in block
    let block = Block::default()
        .borders(Borders::TOP)
        .padding(Padding::new(2, 2, 1, 1))
        .title(format!(" Disk {} ", d.name));

    let inner_area = block.inner(area);

    f.render_widget(block, area);

    // Setup layout for display elements
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Percentage(99),
        ])
        .split(inner_area);

    // Draw name and title information
    let formatted_message = vec![
        format!("{} {}", d.kind, d.name),
        format!(
            "Free space: {} ({}b)",
            human_bytes(d.available_space as f64),
            d.available_space
        ),
        format!(
            "Total space: {} ({}b)",
            human_bytes(d.total_space as f64),
            d.total_space
        ),
    ];

    f.render_widget(Paragraph::new(formatted_message.join("\n")), layout[0]);

    let usage_percent = 1.0 - (d.available_space as f64 / d.total_space as f64);

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(get_color_for_range(usage_percent, (0.0, 1.0)))
                .bg(Color::DarkGray),
        )
        .label(format!(
            "{} free ({}% used)",
            human_bytes(d.available_space as f64),
            format!("{:.2}", 100.0 * usage_percent)
        ))
        .ratio(usage_percent);

    f.render_widget(gauge, layout[1]);
}

fn get_color_for_range(v: f64, r: (f64, f64)) -> Color {
    let x = (v - r.0) / r.1;

    match x {
        0.0..=0.6 => Color::Green,
        0.6..=0.85 => Color::Yellow,
        _ => Color::Red,
    }
}
