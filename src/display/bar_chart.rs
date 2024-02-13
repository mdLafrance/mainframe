use core::panic;

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

use super::util::get_color_for_range;

static BAR_CHARACTER: char = '|';

/// Generates a text-only bar chart to display one-dimensional data.
///
/// This is a workaround for [`ratatui`], which does include native bar chart
/// components, but has no way to embed them in a scrollable layout.
///
/// The bar chart will render data according to [`name`], [`value`], and
/// [`bounds`], and will choose rendering dimensions according to
/// [`name_width`], and [`width`].
///
/// Since we are using an immediate mode ui, expected width values are already
/// known at draw time.
///
/// - name: A label which will sit to the left of the chart.
/// - value: The current value of the bar chart. The percentage fill of the
/// chart will be calculated from this value, and the bounds.
/// - bounds: The expected upper and lower bounds for the data (the bottom and
/// top of the bar chart)
/// - name_width: How much to pad the name to.
/// - width: How many characters the full bar chart (including padded name)
/// should be calculated to take up.
///
/// # Example
/// ```
/// use mainframe::display::bar_chart::*;
///
/// let current_temperature = 75f32;
/// let temperature_bounds = (0f32, 110f32);
///
/// // Generate a bar chart to show the above data.
/// // We want the name to be padded to 6 characters wide,
/// // and we've precalculated that we want this bar chart to total 20
/// // characters in width.
/// let b = generate_bar_chart(
///     "Temp(C)",
///     current_temperature,
///     temperature_bounds,
///     6,
///     20
/// );
/// ```
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

    // Bar charts have two bracket padding characters, so if there isn't enough
    // room to draw any bar content, we can exit here.
    if width <= 2 {
        return Box::new(Line::from(vec![start, end]));
    }

    // Number of text characters the actual bar characters will occupy.
    let bar_characters = width as i32 - 2 - name_width as i32;

    // yes this will panic if bounds[1] is 0, but as this is an internal api,
    // not gonna worry about it.
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
