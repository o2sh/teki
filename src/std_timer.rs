use std::time::{Duration, SystemTime};

use teki_common::traits::Timer;

pub struct StdTimer {
    last_time: SystemTime,
}

impl StdTimer {
    pub fn new() -> Self {
        StdTimer { last_time: SystemTime::now() }
    }
}

impl Timer for StdTimer {
    fn passed_one_second(&mut self) -> bool {
        let now = SystemTime::now();
        if now.duration_since(self.last_time).expect("Time went backwards").as_secs() < 1 {
            return false;
        }

        self.last_time = self.last_time + Duration::from_secs(1);
        true
    }
}
