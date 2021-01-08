pub trait Audio {
    fn play_sound(&mut self, channel: u32, filename: &str);
    fn play_loop(&mut self, channel: u32, filename: &str);
    fn halt(&mut self);
}
