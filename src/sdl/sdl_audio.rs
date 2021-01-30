use crate::sdl::SdlResourceManager;
use sdl2::mixer::{Chunk, Music, MAX_VOLUME};
use teki_common::traits::audio::Audio;
use teki_common::utils::consts::{BGM_VOLUME, SFX_VOLUME};

pub struct SdlAudio<'a> {
    channels: Vec<Option<Chunk>>,
    resource_manager: SdlResourceManager<Music<'a>>,
}

impl SdlAudio<'static> {
    pub fn new(channel_count: u32) -> Self {
        let mut channels = Vec::with_capacity(channel_count as usize);
        channels.resize_with(channel_count as usize, || None);

        Self { channels, resource_manager: SdlResourceManager::new() }
    }

    fn play_sound(&mut self, channel: u32, filename: &str) {
        if channel < self.channels.len() as u32 {
            let path = format!("{}.ogg", filename);
            let mut chunk = Chunk::from_file(path).expect("No music file");
            chunk.set_volume((MAX_VOLUME as f32 * SFX_VOLUME) as i32);
            sdl2::mixer::Channel::all().play(&chunk, 0).expect("Music cannot be played");
            self.channels[channel as usize] = Some(chunk);
        }
    }

    fn play_music(&mut self, filename: &str, loops: i32) {
        let music = self.resource_manager.get_mut(filename).unwrap();
        Music::set_volume((MAX_VOLUME as f32 * BGM_VOLUME) as i32);
        music.play(loops).expect("");
    }
}

impl Audio for SdlAudio<'static> {
    fn load_musics(&mut self, base_path: &str, filenames: &[&str]) -> Result<(), String> {
        self.resource_manager.load(base_path, filenames, |path: &str| Music::from_file(path))
    }

    fn play_sound(&mut self, channel: u32, filename: &str) {
        self.play_sound(channel, filename)
    }

    fn play_music(&mut self, _: u32, filename: &str) {
        self.play_music(filename, i32::MAX)
    }
}
