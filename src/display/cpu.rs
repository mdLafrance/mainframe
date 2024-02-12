use ratatui::{
    layout::Rect,
    widgets::{
        Block, BorderType::Rounded, Borders, Paragraph, Scrollbar,
        ScrollbarOrientation::VerticalRight, ScrollbarState,
    },
    Frame,
};

use crate::monitoring::polling::Measurement;

use super::{bar_chart::generate_bar_chart, state::UIState};

/// Draw the cpu usage block to the given frame.
///
/// The CPU usage block is a scrollable block element that contains usage stats
/// for current cpus.
pub fn draw_cpu_usage_block(
    state: &mut UIState,
    readings: &Vec<Measurement>,
    f: &mut Frame,
    area: Rect,
) {
    let block = Block::default()
        .title(" CPU Usage ")
        .borders(Borders::ALL)
        .border_type(Rounded);

    let inner_area = block.inner(area);

    f.render_widget(block, area);

    let lines = match readings.len() {
        0 => vec![],
        _ => readings
            .iter()
            .map(|measurement: &Measurement| {
                *generate_bar_chart(
                    &measurement.name,
                    measurement.value,
                    (0f32, 100f32),
                    8,
                    inner_area.width as usize,
                )
            })
            .collect(),
    };

    let do_render_scrollbar = lines.len() > inner_area.height as usize;

    if !do_render_scrollbar {
        state.cpu_scroll = 0;
    }

    let p = Paragraph::new(lines.clone()).scroll((state.cpu_scroll as u16, 0));

    let scrollbar = Scrollbar::new(VerticalRight)
        .begin_symbol(Some("^"))
        .end_symbol(Some("v"));

    let mut scrollbar_state = ScrollbarState::new(lines.len()).position(state.cpu_scroll as usize);

    f.render_widget(p, inner_area);

    if do_render_scrollbar {
        f.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
    }
}

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
