/// Module sysinfo_shim provides wrappings around the sysinfo crate.
///
use std::time::Instant;

use sysinfo::System;

pub use super::polling::{
    Measurement, SystemPollResult, SystemPoller, SystemPollerTargets, TimePoint,
};
use super::system::SystemInformation;

pub struct SiSystemPoller {
    sys: System,
    target_flags: Vec<SystemPollerTargets>,
}

impl SiSystemPoller {
    /// Get data and construct a SystemInformation object.
    ///
    /// Calls `poll()` once in order to obtain accurate readings.
    pub fn get_system_info(&mut self) -> SystemInformation {
        self.poll();

        SystemInformation {
            os: System::name().unwrap_or_else(|| "".to_owned()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "".to_owned()),
            os_version: System::os_version().unwrap_or_else(|| "".to_owned()),
            host_name: System::host_name().unwrap_or_else(|| "".to_owned()),
            logical_processors: self.sys.cpus().len(),
            physical_processors: self.sys.physical_core_count().unwrap_or_else(|| 0),
            total_memory: self.sys.total_memory(),
        }
    }
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

    fn with_poll_targets(mut self, targets: Vec<SystemPollerTargets>) -> Self {
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
