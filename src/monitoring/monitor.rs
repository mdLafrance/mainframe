use std::time::Instant;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SystemMonitorTargets {
    CpuUsage = 0,
    CpuTemperature = 1,
    CpuFanspeed = 2,
    RamUsage = 3,
}

/// SystemPollResult struct holds the latest polled system data, and is
/// returned from a call to `SystemMonitor::poll()`.
///
/// Fields correspond to flag options of SystemMonitorTargets, and if the monitor
/// is not set up to track that metric, the corresponding field in this struct
/// will be empty.
#[derive(Debug)]
pub struct SystemPollResult<'a> {
    pub cpu_usage: &'a [Measurement],
    pub cpu_temperature: &'a [Measurement],
    pub cpu_fanspeed: &'a [Measurement],
    pub ram_usage: &'a [Measurement],

    pub(in crate::monitoring) measurements: Vec<Measurement>,
}

impl SystemPollResult<'_> {
    pub fn new() -> Self {
        SystemPollResult {
            cpu_usage: &[],
            cpu_temperature: &[],
            cpu_fanspeed: &[],
            ram_usage: &[],
            measurements: vec![Measurement::default(); 0],
        }
    }
}

/// Trait SystemMonitor defines the expected interface for an object which can
/// be used to monitor the performance of the system.
pub trait SystemMonitor {
    fn new(
        target_flags: Vec<SystemMonitorTargets>,
        poll_rate: usize,
        poll_buffer_size: usize,
    ) -> Self;

    fn poll(&mut self) -> SystemPollResult;
}

/// Struct TimePoint encodes a moment in time.
#[derive(Copy, Clone, Debug)]
pub struct TimePoint(Instant);

#[derive(Copy, Clone, Debug)]
pub struct MeasurementName([u8; 64]);

/// Measurement encodes a single measurement of some floating point metric
/// at a moment in time.
#[derive(Clone, Debug)]
pub struct Measurement {
    pub name: MeasurementName,
    pub time: TimePoint,
    pub value: f32,
}

impl Default for TimePoint {
    fn default() -> Self {
        TimePoint(Instant::now())
    }
}

impl Default for MeasurementName {
    fn default() -> Self {
        MeasurementName([0; 64])
    }
}

impl Default for Measurement {
    fn default() -> Self {
        Measurement {
            name: MeasurementName::default(),
            time: TimePoint::default(),
            value: 0 as f32,
        }
    }
}
