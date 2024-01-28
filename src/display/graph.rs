use ratatui::{
    layout::Rect,
    style::Color,
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Line, Map, MapResolution, Points, Rectangle},
        Block, Borders,
    },
    Frame,
};

use crate::monitoring::polling::Measurement;

pub struct GraphOpts {
    pub visible_time_points: usize,
}

impl Default for GraphOpts {
    fn default() -> Self {
        GraphOpts {
            visible_time_points: 20,
        }
    }
}

/// Draw a graph of the given `data`.
pub fn draw_graph(data: &[Measurement], opts: &GraphOpts, f: &mut Frame, area: Rect) {
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            // Background
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
                color: Color::DarkGray,
            });
            ctx.layer();
            ctx.draw(&Line {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
                color: Color::LightBlue,
            });
            ctx.draw(&Points {
                coords: (0..10)
                    .map(|i| (i as f64 * 0.01, 0.0))
                    .collect::<Vec<(f64, f64)>>()
                    .as_slice(),
                color: Color::LightRed,
            });
            ctx.draw(&Rectangle {
                x: 10.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
                color: Color::Red,
            });
        });

    f.render_widget(canvas, area);
}
