#![allow(patterns_in_fns_without_body)]

use std::time::Instant;

use systemstat::Platform;

use super::system::{DiskInformation, SystemInformation};

/// SystemPollTargets enum allows selection of specific targets when performing
/// a system poll.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SystemPollerTarget {
    CpuUsage = 0,
    CpuTemperature = 1,
    CpuFanspeed = 2,
    RamUsage = 3,
}

/// SystemPollResult struct holds the latest polled system data, and is
/// returned from a call to `SystemPoller::poll()`.
///
/// Fields correspond to flag options of SystemPollerTarget, and if the monitor
/// is not set up to track that metric, the corresponding field in this struct
/// will be empty.
#[derive(Debug, Clone)]
pub struct SystemPollResult {
    pub cpu_usage: Vec<Measurement>,
    pub cpu_temperature: Measurement,
    pub ram_usage: Vec<Measurement>,
}

impl SystemPollResult {
    pub fn new() -> Self {
        SystemPollResult {
            cpu_usage: vec![Measurement::default(); 0],
            cpu_temperature: Measurement::default(),
            ram_usage: vec![Measurement::default(); 0],
        }
    }
}

impl Default for SystemPollResult {
    fn default() -> Self {
        SystemPollResult::new()
    }
}

/// Trait SystemPoller defines the expected interface for an object which can
/// be used to monitor the performance of the system.
pub struct SystemPoller {
    sysinfo_system: sysinfo::System,
    systemstat_system: systemstat::System,
    target_flags: Vec<SystemPollerTarget>,
}

impl SystemPoller {
    /// Initialize a new SystemPoller compatible object using the sysinfo
    /// crate as a backend.
    pub fn new() -> Self {
        let mut sysinfo_system = sysinfo::System::new_all();
        sysinfo_system.refresh_all();

        SystemPoller {
            sysinfo_system,
            systemstat_system: systemstat::System::new(),
            target_flags: vec![],
        }
    }

    /// Select which poll targets this poller object should fetch.
    ///
    /// As polling can be an expensive process, only select the metrics you
    /// want to read.
    ///
    /// # Example
    /// ```
    /// // Initialize poller to read cpu usage, and temperature only.
    /// let s = SystemPoller::new()
    ///     .with_poll_targets(vec![
    ///         SystemPollerTarget::CpuUsage,
    ///         SystemPollerTarget::CpuTemperature
    ///     ]);
    /// ```
    pub fn with_poll_targets(mut self, targets: Vec<SystemPollerTarget>) -> Self {
        self.target_flags = targets;

        self
    }

    /// Poll the system for each of the previously defined poll targets.
    ///
    /// See [`with_poll_targets`] for more details.
    pub fn poll(&mut self) -> SystemPollResult {
        let mut res = SystemPollResult::new();
        let time = Instant::now();

        for k in self.target_flags.as_slice() {
            match k {
                // Fetch cpu usage
                SystemPollerTarget::CpuUsage => {
                    self.sysinfo_system.refresh_cpu();

                    for cpu in self.sysinfo_system.cpus() {
                        res.cpu_usage.push(Measurement {
                            name: cpu.name().to_owned(),
                            time: TimePoint(time),
                            value: cpu.cpu_usage(),
                        });
                    }
                }
                SystemPollerTarget::CpuTemperature => {
                    res.cpu_temperature = Measurement {
                        name: "".into(),
                        time: TimePoint(time),
                        value: match self.systemstat_system.cpu_temp() {
                            Ok(v) => v,
                            Err(_) => 10f32,
                        },
                    };
                    // println!("Polled temp: {:?}", res.cpu_temperature);
                }
                _ => (),
            }
        }

        res
    }

    /// Get data and construct a SystemInformation object.
    ///
    /// Calls `poll()` once in order to obtain accurate readings.
    pub fn get_system_info(&mut self) -> SystemInformation {
        self.poll();

        SystemInformation {
            os: sysinfo::System::name().unwrap_or_else(|| "".to_owned()),
            kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "".to_owned()),
            os_version: sysinfo::System::os_version().unwrap_or_else(|| "".to_owned()),
            host_name: sysinfo::System::host_name().unwrap_or_else(|| "".to_owned()),
            logical_processors: self.sysinfo_system.cpus().len(),
            physical_processors: self
                .sysinfo_system
                .physical_core_count()
                .unwrap_or_else(|| 0),
            total_memory: self.sysinfo_system.total_memory(),
        }
    }

    /// Construct a disk data object for each disk.
    pub fn get_disk_info(&mut self) -> Vec<DiskInformation> {
        let mut disks = Vec::<DiskInformation>::new();

        for disk in &sysinfo::Disks::new_with_refreshed_list() {
            disks.push(DiskInformation {
                name: disk.name().to_string_lossy().to_string(),
                kind: match disk.kind() {
                    sysinfo::DiskKind::SSD => "SSD".to_string(),
                    sysinfo::DiskKind::HDD => "HDD".to_string(),
                    sysinfo::DiskKind::Unknown(s) => format!("??? ({})", s),
                },
                available_space: disk.total_space() - disk.available_space(),
                total_space: disk.total_space(),
            });
        }

        disks
    }
}

/// Struct TimePoint encodes a moment in time.
#[derive(Copy, Clone, Debug)]
pub struct TimePoint(pub Instant);

/// Measurement encodes a single measurement of some floating point metric
/// at a moment in time.
#[derive(Clone, Debug)]
pub struct Measurement {
    pub name: String,
    pub time: TimePoint,
    pub value: f32,
}

impl Default for TimePoint {
    fn default() -> Self {
        TimePoint(Instant::now())
    }
}

impl Default for Measurement {
    fn default() -> Self {
        Measurement {
            name: String::default(),
            time: TimePoint::default(),
            value: 0 as f32,
        }
    }
}
