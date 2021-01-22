pub trait Audio {
    fn load_musics(&mut self, base_path: &str, filenames: &[&str]) -> Result<(), String>;
    fn play_sound(&mut self, channel: u32, filename: &str);
    fn play_music(&mut self, channel: u32, filename: &str);
}
