use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, List, Padding},
    Frame,
};

use crate::{
    display::util::draw_sys_info,
    monitoring::system::{DiskInformation, SystemData},
};

pub fn draw_home_page(data: &SystemData, f: &mut Frame, area: Rect) {
    // Setup layout
    let vertical_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let (top_section, bottom_section) = (vertical_layout[0], vertical_layout[1]);

    let top_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_section);

    let top_left = top_layout[0];
    let top_right = top_layout[1];

    // Draw elements
    draw_sys_info(&data.info, f, top_left);
    draw_cpu_usage_graph(f, bottom_section);
}

fn draw_cpu_usage_graph(f: &mut Frame, area: Rect) {
    // Create the datasets to fill the chart with
    let datasets = vec![
        // Scatter chart
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().cyan())
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        // Line chart
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().magenta())
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["".into()]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Average CPU usage".red())
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["0.0".into(), "50.0".into(), "100.0".into()]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Usage ")
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 1)),
        );

    f.render_widget(chart, area);
}
