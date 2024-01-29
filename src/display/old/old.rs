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
    layout::Layout,
    prelude::*,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Map, MapResolution, Rectangle},
        Block, BorderType, Borders, Gauge, LineGauge, Padding, Paragraph, RenderDirection, Row,
        Sparkline, Table, Wrap,
    },
};

use crate::monitoring::system::{DiskInformation, SystemInformation};

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

pub fn draw_graph(f: &mut Frame, area: Rect) {
    let graph = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();

            ctx.draw(&Rectangle {
                x: 10.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
                color: Color::Red,
            });
        });
    f.render_widget(graph, area);
}

fn get_color_for_range(v: f64, r: (f64, f64)) -> Color {
    let x = (v - r.0) / r.1;

    match x {
        0.0..=0.6 => Color::Green,
        0.6..=0.85 => Color::Yellow,
        _ => Color::Red,
    }
}
