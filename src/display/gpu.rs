use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::monitoring::polling::GpuPollResult;

use super::bar_chart::generate_bar_chart;

pub fn draw_gpu_info_block(gpu: &GpuPollResult, f: &mut Frame, area: Rect) {
    let b = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(" GPU ");

    let inner_area = b.inner(area);

    let width = inner_area.width;

    // let temp_text = format!("{}C", cpu_temp.value);

    let p = Paragraph::new("asdf".to_string());
    // let p = Paragraph::new(*generate_bar_chart(
    //     &temp_text,
    //     cpu_temp.value,
    //     (0f32, 100f32),
    //     8,
    //     width as usize,
    // ));

    f.render_widget(b, area);
    f.render_widget(p, inner_area);
}
