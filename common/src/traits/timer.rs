pub trait Timer {
    fn passed_one_second(&mut self) -> bool;
}
