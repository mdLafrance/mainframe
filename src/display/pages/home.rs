use ratatui::{layout::Rect, Frame};

use crate::{display::util::draw_sys_info, monitoring::system::SystemData};

pub fn draw_home_page(data: &SystemData, f: &mut Frame, area: Rect) {
    draw_sys_info(&data.info, f, area);
}
