#![allow(patterns_in_fns_without_body)]

use std::time::Instant;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SystemPollerTargets {
    CpuUsage = 0,
    CpuTemperature = 1,
    CpuFanspeed = 2,
    RamUsage = 3,
}

/// SystemPollResult struct holds the latest polled system data, and is
/// returned from a call to `SystemPoller::poll()`.
///
/// Fields correspond to flag options of SystemPollerTargets, and if the monitor
/// is not set up to track that metric, the corresponding field in this struct
/// will be empty.
#[derive(Debug, Clone)]
pub struct SystemPollResult {
    pub cpu_usage: Vec<Measurement>,
    pub cpu_temperature: Vec<Measurement>,
    pub cpu_fanspeed: Vec<Measurement>,
    pub ram_usage: Vec<Measurement>,
}

impl SystemPollResult {
    pub fn new() -> Self {
        SystemPollResult {
            cpu_usage: vec![Measurement::default(); 0],
            cpu_temperature: vec![Measurement::default(); 0],
            cpu_fanspeed: vec![Measurement::default(); 0],
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
pub trait SystemPoller {
    fn new() -> Self;

    /// Specify that this system monitor should monitor the given targets.
    fn with_poll_targets(mut self, targets: Vec<SystemPollerTargets>) -> Self;

    fn poll(&mut self) -> SystemPollResult;
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
