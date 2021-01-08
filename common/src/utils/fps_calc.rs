use std::time::{Duration, SystemTime};

pub struct FpsCalc {
    fps: i32,
    last_time: SystemTime,
    ndraw: i32,
}

impl FpsCalc {
    pub fn new() -> Self {
        Self { fps: 0, last_time: SystemTime::now(), ndraw: 0 }
    }

    pub fn update(&mut self) -> bool {
        self.ndraw += 1;
        if !self.passed_one_second() {
            return false;
        }

        self.fps = self.ndraw;
        self.ndraw = 0;
        true
    }

    pub fn fps(&self) -> i32 {
        self.fps
    }

    fn passed_one_second(&mut self) -> bool {
        let now = SystemTime::now();
        if now.duration_since(self.last_time).expect("Time went backwards").as_secs() < 1 {
            return false;
        }

        self.last_time = self.last_time + Duration::from_secs(1);
        true
    }
}
