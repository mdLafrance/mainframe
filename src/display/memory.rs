use human_bytes::human_bytes;
use ratatui::style::Modifier;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use super::{bar_chart::generate_bar_chart, util::get_color_for_range};

pub fn draw_memory_usage_block(total_memory: f32, used_memory: f32, f: &mut Frame, area: Rect) {
    let usage_percent_text = format!("{}%", (100f32 * used_memory / total_memory) as usize);

    let text = vec![
        Line::from(vec![
            Span::styled(
                "Total Memory: ".to_string(),
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(human_bytes(total_memory), Style::new()),
        ]),
        Line::from(vec![
            Span::styled(
                "Used Memory:  ".to_string(),
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                human_bytes(used_memory),
                Style::new().fg(get_color_for_range(used_memory, (0f32, total_memory))),
            ),
        ]),
        *generate_bar_chart(
            &usage_percent_text,
            used_memory,
            (0f32, total_memory),
            5,
            area.width as usize - 6,
        ),
    ];

    let p = Paragraph::new(text).block(
        Block::new()
            .title(" Memory ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 2, 0, 0)),
    );

    f.render_widget(p, area)
}
