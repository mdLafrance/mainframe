// Contains functionality for drawing ui elements related to gpu reporting.
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::monitoring::polling::GpuPollResult;

use super::{bar_chart::generate_bar_chart, util::default_block};

/// Draws a blocked element reporting gpu name, average usage, and temperature.
pub fn draw_gpu_info_block(gpu_data: &Vec<GpuPollResult>, f: &mut Frame, area: Rect) {
    let gpu_block_height = 8;

    let mut gpu_constraints = vec![Constraint::Length(gpu_block_height); gpu_data.len()];
    gpu_constraints.push(Constraint::Percentage(99));

    let gpu_layout = Layout::default().constraints(gpu_constraints).split(area);

    for i in 0..gpu_data.len() {
        let gpu = &gpu_data[i];

        // Compute block element
        let b = Block::new()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::new().dark_gray())
            .title(format!(" {} ", gpu.name))
            .title_style(Style::new().white().bold())
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            });

        let current_gpu_layout = b.inner(gpu_layout[i]);

        f.render_widget(b, gpu_layout[i]);

        // Compute interior layout for gpu
        let l = Layout::default()
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(current_gpu_layout);

        // Gpu temp
        let temp_str = format!(" {}C", gpu.temp);
        f.render_widget(
            Paragraph::new(*generate_bar_chart(
                &temp_str,
                gpu.temp,
                (0f32, 100f32),
                8,
                current_gpu_layout.width as usize - 2,
            ))
            .block(default_block(" GPU Temp (C) ")),
            l[0],
        );

        // Gpu usage
        let usage_str = format!(" {}%", gpu.usage);
        f.render_widget(
            Paragraph::new(*generate_bar_chart(
                &usage_str,
                gpu.usage,
                (0f32, 100f32),
                8,
                current_gpu_layout.width as usize - 2,
            ))
            .block(default_block(" GPU Usage (avg) ")),
            l[1],
        )
    }
}
