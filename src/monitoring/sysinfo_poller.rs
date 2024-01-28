use std::time::Instant;

use sysinfo::System;

pub use super::polling::{
    Measurement, SystemPollResult, SystemPoller, SystemPollerTargets, TimePoint,
};

pub struct SiSystemPoller {
    sys: System,
    target_flags: Vec<SystemPollerTargets>,
}

impl SystemPoller for SiSystemPoller {
    /// Initialize a new SystemPoller compatible object using the sysinfo
    /// crate as a backend.
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SiSystemPoller {
            sys,
            target_flags: vec![],
        }
    }

    fn set_poll_targets(&mut self, targets: Vec<SystemPollerTargets>) -> &mut Self {
        self.target_flags = targets;

        self
    }

    fn poll(&mut self) -> SystemPollResult {
        let mut res = SystemPollResult::new();
        let time = Instant::now();

        for k in self.target_flags.as_slice() {
            match k {
                // Fetch cpu usage
                SystemPollerTargets::CpuUsage => {
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
