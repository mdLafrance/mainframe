use human_bytes::human_bytes;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::monitoring::system::{DiskInformation, SystemInformation};

pub fn draw_sys_info(s: &SystemInformation, f: &mut Frame, area: Rect) {
    let style_category = |s: String| Span::styled(s, Style::new().add_modifier(Modifier::BOLD));
    let style_value = |s: String| Span::styled(s, Style::new());

    let text = vec![
        Line::from(vec![
            style_category("Host Name:         ".into()),
            style_value(s.host_name.clone()),
        ]),
        Line::from(vec![
            style_category("Operating System:  ".into()),
            style_value(s.os.clone()),
        ]),
        Line::from(vec![
            style_category("OS Version:        ".into()),
            style_value(s.os_version.clone()),
        ]),
        Line::from(vec![
            style_category("Kernel Version:    ".into()),
            style_value(s.kernel_version.clone()),
        ]),
        Line::from(vec![
            style_category("CPU count:         ".into()),
            style_value(s.logical_processors.to_string()),
        ]),
        Line::from(vec![
            style_category("Core count:        ".into()),
            style_value(s.physical_processors.to_string()),
        ]),
    ];

    let p = Paragraph::new(text).block(
        Block::new()
            .title(" System Information ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 1, 1)),
    );

    f.render_widget(p, area)
}

pub fn draw_disk_info(d: &DiskInformation, f: &mut Frame, area: Rect) {
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

    // let usage_percent = 1.0 - (d.available_space as f64 / d.total_space as f64);

    // let gauge = Gauge::default()
    //     .gauge_style(
    //         Style::default()
    //             .fg(get_color_for_range(usage_percent, (0.0, 1.0)))
    //             .bg(Color::DarkGray),
    //     )
    //     .label(format!(
    //         "{} free ({}% used)",
    //         human_bytes(d.available_space as f64),
    //         format!("{:.2}", 100.0 * usage_percent)
    //     ))
    //     .ratio(usage_percent);

    // f.render_widget(gauge, layout[1]);
}

pub fn get_color_for_range(v: f32, r: (f32, f32)) -> Color {
    let x = (v - r.0) / r.1;

    match x {
        x if (0.0..0.6).contains(&x) => Color::Green,
        x if (0.6..0.85).contains(&x) => Color::Yellow,
        _ => Color::Red,
    }
}

pub fn default_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
}
