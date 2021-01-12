use teki_common::traits::Timer;

const SEC: u64 = 1000;

pub struct WasmTimer<F: Fn() -> f64> {
    last_time: u64,
    get_now: F,
}

impl<F: Fn() -> f64> WasmTimer<F> {
    pub fn new(get_now: F) -> Self {
        let last_time = get_now() as u64;
        WasmTimer { last_time, get_now }
    }
}

impl<F: Fn() -> f64> Timer for WasmTimer<F> {
    fn passed_one_second(&mut self) -> bool {
        let now = (self.get_now)() as u64;
        if now.wrapping_sub(self.last_time) < SEC {
            return false;
        }

        self.last_time += SEC;
        true
    }
}
