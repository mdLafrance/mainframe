use std::{error::Error, time::Instant};

use ratatui::{
    layout::{
        Constraint,
        Direction::{Horizontal, Vertical},
        Layout,
    },
    widgets::Block,
};

use crate::{
    display::{
        graph,
        ui::{self, init_ui, shutdown_ui, ui_should_close},
    },
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

            let layout = Layout::default()
                .direction(Horizontal)
                .constraints(vec![Constraint::Max(100), Constraint::Min(10)])
                .split(frame);

            ui::draw_sys_info(&sys_info, f, layout[0]);

            ui::display_disk_info(&sys_poller.get_disk_info()[0], f, layout[1]);

            // graph::draw_graph(&[], &graph::GraphOpts::default(), f, layout[1]);
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
