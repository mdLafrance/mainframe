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
        .title("CPU Usage")
        .borders(Borders::ALL)
        .border_type(Rounded);

    let inner_area = block.inner(area);

    f.render_widget(block, area);

    let lines = match readings.len() {
        0 => vec![],
        _ => readings
            .iter()
            .map(|measurement: &Measurement| {
                *generate_bar_chart(measurement.value, (0f32, 100f32), 50)
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
