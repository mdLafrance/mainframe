use std::time::Instant;

use sysinfo::System;

use super::monitor::{
    Measurement, SystemMonitor, SystemMonitorTargets, SystemPollResult, TimePoint,
};

pub struct SiSystemMonitor {
    sys: System,
    target_flags: Vec<SystemMonitorTargets>,
    poll_rate: usize,
    poll_buffer_size: usize,
}

impl SystemMonitor for SiSystemMonitor {
    /// Initialize a new SystemMonitor compatible object using the sysinfo
    /// crate as a backend.
    fn new(
        target_flags: Vec<SystemMonitorTargets>,
        poll_rate: usize,
        poll_buffer_size: usize,
    ) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SiSystemMonitor {
            sys,
            target_flags,
            poll_rate,
            poll_buffer_size,
        }
    }

    fn poll(&mut self) -> SystemPollResult {
        let mut res = SystemPollResult::new();
        let time = Instant::now();

        for k in self.target_flags.as_slice() {
            match k {
                // Fetch cpu usage
                SystemMonitorTargets::CpuUsage => {
                    self.sys.refresh_cpu();

                    for cpu in self.sys.cpus() {
                        res.cpu_usage.push(Measurement {
                            name: cpu.name().to_owned(),
                            time: TimePoint(time),
                            value: cpu.cpu_usage(),
                        });
                    }
                }
                _ => (),
            }
        }

        res
    }
}
