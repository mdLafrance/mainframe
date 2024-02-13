// Contains functionality for drawing ui elements related to cpu reporting.
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType::Rounded, Borders, Padding, Paragraph},
    Frame,
};

use crate::monitoring::polling::Measurement;

use super::bar_chart::generate_bar_chart;

/// Draw the cpu usage block to the given frame.
///
/// The CPU usage block is a scrollable block element that contains usage stats
/// for current cpus.
pub fn draw_cpu_usage_block(readings: &Vec<Measurement>, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" CPU Usage ")
        .borders(Borders::ALL)
        .padding(Padding::new(1, 1, 0, 0))
        .border_type(Rounded);

    let inner_area = block.inner(area);

    let left_right_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(inner_area);

    let (left_area, right_area) = (left_right_layout[0], left_right_layout[2]);

    f.render_widget(block, area);

    let mut paragraphs = vec![Vec::<Line>::new(); 2];

    match readings.len() {
        0 => (),
        _ => readings
            .iter()
            .enumerate()
            .for_each(|(i, measurement): (usize, &Measurement)| {
                let bar_chart = *generate_bar_chart(
                    &measurement.name,
                    measurement.value,
                    (0f32, 100f32),
                    6,
                    left_area.width as usize - 2,
                );

                paragraphs[i % 2].push(bar_chart);
            }),
    };

    f.render_widget(Paragraph::new(paragraphs[0].clone()), left_area);
    f.render_widget(Paragraph::new(paragraphs[1].clone()), right_area);
}

/// Draws a blocked bar chart reporting cpu average usage.
///
/// Average usage is calculated as the average over all currently polled cpus.
pub fn draw_cpu_average_block(readings: &Vec<Measurement>, f: &mut Frame, area: Rect) {
    let b = Block::default()
        .borders(Borders::ALL)
        .border_type(Rounded)
        .title(" CPU Load (avg) ");

    let inner_area = b.inner(area);

    let width = inner_area.width;

    let mut load_avg = 0f32;

    if readings.len() > 0 {
        load_avg = readings.iter().map(|m| m.value).sum::<f32>() / readings.len() as f32;
    }

    let usage_text = format!("{}%", load_avg as i32);

    let p = Paragraph::new(*generate_bar_chart(
        &usage_text,
        load_avg,
        (0f32, 100f32),
        8,
        width as usize,
    ));

    f.render_widget(b, area);
    f.render_widget(p, inner_area);
}

/// Draws a blocked bar chart reporting cpu temperature.
pub fn draw_cpu_temp_block(cpu_temp: &Measurement, f: &mut Frame, area: Rect) {
    let b = Block::default()
        .borders(Borders::ALL)
        .border_type(Rounded)
        .title(" CPU Temp (C) ");

    let inner_area = b.inner(area);

    let width = inner_area.width;

    let temp_text = format!("{}C", cpu_temp.value);

    let p = Paragraph::new(*generate_bar_chart(
        &temp_text,
        cpu_temp.value,
        (0f32, 100f32),
        8,
        width as usize,
    ));

    f.render_widget(b, area);
    f.render_widget(p, inner_area);
}
