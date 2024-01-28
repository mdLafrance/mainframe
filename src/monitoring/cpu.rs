use super::metric::{Measurement, Metric};

struct CPU {
    name: String,
}

impl Metric for CPU {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn poll(&mut self) -> Measurement {
        todo!();
    }
}
