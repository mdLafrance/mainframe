use std::time::Instant;

/// Trait Metric describes a measurable hardware value that changes over time.
///
/// An example of a measureable metric is 'cpu usage'.
pub trait Metric {
    /// Get the name of this metric.
    fn name(&self) -> String;

    /// Record a new measurement for this metric.
    ///
    /// The new measurement is instantly returned, and is also recorded in the
    /// Metric's measurements buffer.
    fn poll(&mut self) -> Measurement;
}

/// Measurement encodes a single measurement of some metric at a moment in time.
pub struct Measurement(pub Instant, pub f32);
