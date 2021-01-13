pub trait Audio {
    fn play_sound(&mut self, channel: u32, filename: &str);
    fn play_loop(&mut self, channel: u32, filename: &str);
    fn stop(&mut self, channel: u32);
}
