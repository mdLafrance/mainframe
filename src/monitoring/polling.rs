#![allow(patterns_in_fns_without_body)]

use std::time::Instant;

use get_sys_info::Platform;
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use systemstat::System;

use super::system::{DiskInformation, SystemInformation};

/// SystemPollTargets enum allows selection of specific targets when performing
/// a system poll.
///
/// The following polling targets are available:
/// - [`Self::CpuUsage`] current usage percentages of available cores.
/// - [`Self::CpuTemperature`] current average cpu temperature.
/// - [`Self::Gpu`] current usage stats about available gpus. (NOTE: Due to
/// limitations of nvidia's available monitoring packages, all gpu information
/// has to be polled at once)
/// - [`Self::Memory`] total and available RAM
#[derive(Debug, Clone, Copy)]
pub enum SystemPollerTarget {
    CpuUsage,
    CpuTemperature,
    Gpu,
    Memory,
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
    pub memory_usage: Measurement,
    pub gpu_info: Vec<GpuPollResult>,
}

#[derive(Clone, Debug)]
pub struct GpuPollResult {
    pub name: String,
    pub temp: f32,
    pub usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
}

impl Default for GpuPollResult {
    fn default() -> Self {
        GpuPollResult {
            name: "???".to_string(),
            temp: 0f32,
            usage: 0f32,
            memory_total: 0u64,
            memory_used: 0u64,
        }
    }
}

impl SystemPollResult {
    pub fn new() -> Self {
        SystemPollResult {
            cpu_usage: vec![Measurement::default(); 0],
            cpu_temperature: Measurement::default(),
            memory_usage: Measurement::default(),
            gpu_info: vec![],
        }
    }
}

impl Default for SystemPollResult {
    fn default() -> Self {
        SystemPollResult::new()
    }
}

/// SystemPoller manages the polling of system data.
///
/// System data includes metrics like cpu usage, temperature, memory
/// usage, and gpu usage, as well as time invariant data like hostname and
/// operating system.
///
/// Instantiate new [`SystemPoller`] instances with [`SystemPoller::new()`].
///
/// By default, not all system metrics will be polled. Select which metrics
/// should be recorded with [`SystemPoller::with_poll_targets()`].
pub struct SystemPoller {
    sysinfo_system: sysinfo::System,
    gsi_system: get_sys_info::System,
    nvml: Option<Nvml>,
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
            gsi_system: get_sys_info::System::new(),
            nvml: match Nvml::init() {
                Ok(nvml) => Some(nvml),
                Err(_) => None,
            },
            target_flags: vec![],
        }
    }

    /// Select which poll targets this poller object should fetch.
    ///
    /// As polling can be an expensive process, only select the metrics you
    /// want to read.
    ///
    /// This function accepts a vector of [`SystemPollerTarget`] enum
    /// variants, which will determine which system data will be fetched on
    /// each call to [`SystemPoller::poll()`].
    ///
    /// See [`SystemPollerTarget`] for specific details about each enum variant.
    ///
    /// # Example
    /// ```
    /// // Initialize poller to read cpu usage, and temperature only.
    /// let s = SystemPoller::new()
    ///     .with_poll_targets(vec![
    ///         SystemPollerTarget::CpuUsage,
    ///         SystemPollerTarget::CpuTemperature
    ///     ]);
    ///
    /// ...
    ///
    /// let result = s.poll(); // Cpu usage, and cpu temperature will be fetched.
    ///
    /// ```
    pub fn with_poll_targets(mut self, targets: Vec<SystemPollerTarget>) -> Self {
        self.target_flags = targets;

        self
    }

    /// Poll the system for each of the previously defined poll targets.
    ///
    /// See [`Self::with_poll_targets()`] for more details about selecting poll targets.
    pub fn poll(&mut self) -> SystemPollResult {
        let mut res = SystemPollResult::new();
        let time = TimePoint(Instant::now());

        for k in self.target_flags.as_slice() {
            match k {
                // Fetch cpu usage
                SystemPollerTarget::CpuUsage => {
                    self.sysinfo_system.refresh_cpu();

                    for cpu in self.sysinfo_system.cpus() {
                        res.cpu_usage.push(Measurement {
                            name: cpu.name().to_owned(),
                            value: cpu.cpu_usage(),
                            time,
                        });
                    }
                }
                SystemPollerTarget::CpuTemperature => {
                    res.cpu_temperature = Measurement {
                        time,
                        name: "".into(),
                        value: match self.gsi_system.cpu_temp() {
                            Ok(v) => v,
                            Err(_) => 10f32,
                        },
                    };
                    // println!("Polled temp: {:?}", res.cpu_temperature);
                }
                SystemPollerTarget::Gpu => res.gpu_info = self.poll_gpus(),
                SystemPollerTarget::Memory => {
                    self.sysinfo_system.refresh_memory();

                    res.memory_usage = Measurement {
                        time,
                        name: "memory".to_string(),
                        value: self.sysinfo_system.used_memory() as f32,
                    }
                }
            }
        }

        res
    }

    /// Get a [`SystemInformation`] object representing the current system.
    ///
    /// Calls [`Self::poll()`] once in order to obtain accurate readings.
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

    fn poll_gpus(&self) -> Vec<GpuPollResult> {
        match &self.nvml {
            None => vec![],
            Some(nvml) => {
                let mut gpus = Vec::<GpuPollResult>::new();
                gpus.reserve(nvml.device_count().unwrap() as usize);

                for i in 0..nvml.device_count().unwrap() {
                    let device = nvml.device_by_index(i).unwrap();

                    let memory_info = device.memory_info();

                    gpus.push(GpuPollResult {
                        name: match device.name() {
                            Ok(n) => n,
                            Err(e) => e.to_string(),
                        },
                        temp: match device.temperature(TemperatureSensor::Gpu) {
                            Ok(t) => t as f32,
                            Err(_) => 0f32,
                        },
                        usage: match device.utilization_rates() {
                            Ok(r) => r.gpu as f32,
                            Err(_) => 0f32,
                        },
                        memory_total: match &memory_info {
                            Ok(m) => m.total,
                            Err(_) => 0,
                        },
                        memory_used: match memory_info {
                            Ok(m) => m.used,
                            Err(_) => 0,
                        },
                    })
                }

                gpus
            }
        }
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
