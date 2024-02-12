use core::panic;

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

use super::util::get_color_for_range;

static BAR_CHARACTER: char = '|';

pub fn generate_bar_chart<'a>(
    name: &'a str,
    value: f32,
    bounds: (f32, f32),
    name_width: usize,
    width: usize,
) -> Box<Line<'a>> {
    let start = Span::styled("[", Style::new().gray());
    let end = Span::styled("]", Style::new().gray());

    if bounds.1 <= bounds.0 {
        panic!("Illegal bar chart bounds: {:?}", bounds);
    }

    if width <= 2 {
        return Box::new(Line::from(vec![start, end]));
    }

    let bar_characters = width as i32 - 2 - name_width as i32;

    // yes this will panic if bounds[1] is 0, but as this is an internal api,
    // we will not be doing that please
    let blocks_f = ((value - bounds.0) / bounds.1) * bar_characters as f32;

    let blocks = blocks_f.round() as i32;

    let empty_blocks = bar_characters - blocks;

    let mut spans = Vec::<Span>::new();

    spans.push(name.into());

    if name.len() < name_width {
        spans.push(" ".repeat(name_width - name.len()).into());
    }

    spans.push(start);

    for i in 0..blocks {
        let color = get_color_for_range(100.0 * i as f32 / bar_characters as f32, bounds);

        spans.push(Span::styled(
            format!("{}", BAR_CHARACTER),
            Style::default().fg(color),
        ));
    }

    for _ in 0..empty_blocks {
        spans.push(Span::styled(
            format!("{}", BAR_CHARACTER),
            Style::default().fg(Color::DarkGray),
        ));
    }

    spans.push(end);

    Box::new(Line::from(spans))
}
