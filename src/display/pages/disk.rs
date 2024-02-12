use ratatui::{
    layout::{Constraint, Layout, Rect, Size},
    widgets::Paragraph,
    Frame,
};
use tui_scrollview::ScrollView;

use crate::{display::util::draw_disk_info, monitoring::system::SystemData};

pub fn draw_disk_page(data: &SystemData, f: &mut Frame, area: Rect) {
    // let disk_layout = Layout::default()
    //     .direction(ratatui::layout::Direction::Vertical)
    //     .constraints(vec![Constraint::Length(10); data.disks.len()])
    //     .split(area);

    // for i in 0..data.disks.len() {
    //     draw_disk_info(&data.disks[i], f, disk_layout[i]);
    // }

    // let size = Size::new(10, 100);
    // let mut scroll_view = ScrollView::new(size);
    // let some_long_string =
    //     std::iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
    //         .take(100)
    //         .collect::<String>();
    // let area = Rect::new(0, 0, 10, 100);
    // scroll_view.render_widget(Paragraph::new(some_long_string), area);
    // let mut state = ScrollViewState::default();
    // frame.render_stateful_widget(scroll_view, area, &mut state);
}
