use crate::traits::Timer;

pub struct FpsCalc<T: Timer> {
    fps: i32,
    timer: T,
    ndraw: i32,
}

impl<T: Timer> FpsCalc<T> {
    pub fn new(timer: T) -> Self {
        Self { fps: 0, timer, ndraw: 0 }
    }

    pub fn update(&mut self) -> bool {
        self.ndraw += 1;
        if !self.timer.passed_one_second() {
            return false;
        }

        self.fps = self.ndraw;
        self.ndraw = 0;
        true
    }

    pub fn fps(&self) -> i32 {
        self.fps
    }
}
