use std::error::Error;

use crate::{
    display::ui::{draw_ui, init_ui, shutdown_ui, ui_should_close},
    monitoring::{
        polling::{SystemPoller, SystemPollerTargets},
        sysinfo_poller::SiSystemPoller,
    },
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut sys_poller =
        SiSystemPoller::new().set_poll_targets(vec![SystemPollerTargets::CpuUsage]);

    let mut terminal = init_ui()?;

    while !ui_should_close()? {
        terminal.draw(|f| {
            draw_ui(f);
        });
    }

    shutdown_ui()?;

    Ok(())
}
