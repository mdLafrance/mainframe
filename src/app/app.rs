use std::{error::Error, time::Instant};

use ratatui::widgets::Block;

use crate::{
    display::ui::{self, init_ui, shutdown_ui, ui_should_close},
    monitoring::{
        polling::{SystemPollResult, SystemPoller, SystemPollerTargets},
        sysinfo_shim::SiSystemPoller,
    },
    ringbuffer::RingBuffer,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut sys_poller =
        SiSystemPoller::new().with_poll_targets(vec![SystemPollerTargets::CpuUsage]);

    let sys_info = sys_poller.get_system_info();

    let mut recorded_metrics = RingBuffer::<SystemPollResult>::new(2);

    let mut terminal = init_ui()?;

    let mut t0 = Instant::now();

    while !ui_should_close()? {
        terminal.draw(|f| {
            let frame = ui::draw_app_outline(f);

            ui::draw_sys_info(&sys_info, f, frame)
        })?;

        // Poll new data if time elapsed
        if t0.elapsed().as_millis() > 1000 {
            t0 = Instant::now();

            recorded_metrics.add(sys_poller.poll());
        }
    }

    shutdown_ui()?;

    Ok(())
}
