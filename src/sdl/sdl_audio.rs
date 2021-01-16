use sdl2::mixer::{Chunk, MAX_VOLUME};
use teki_common::traits::audio::Audio;

pub struct SdlAudio {
    channels: Vec<Option<Chunk>>,
    base_volume: i32,
}

impl SdlAudio {
    pub fn new(channel_count: u32, base_volume: f32) -> Self {
        let mut channels = Vec::with_capacity(channel_count as usize);
        channels.resize_with(channel_count as usize, || None);

        Self { channels, base_volume: (MAX_VOLUME as f32 * base_volume) as i32 }
    }

    fn play(&mut self, channel: u32, filename: &str, loops: i32) {
        if channel < self.channels.len() as u32 {
            let path = format!("{}.ogg", filename);
            let mut chunk = Chunk::from_file(path).expect("No music file");
            chunk.set_volume(self.base_volume);
            sdl2::mixer::Channel::all().play(&chunk, loops).expect("Music cannot be played");
            self.channels[channel as usize] = Some(chunk);
        }
    }
}

impl Audio for SdlAudio {
    fn play_sound(&mut self, channel: u32, filename: &str) {
        self.play(channel, filename, 0)
    }

    fn play_loop(&mut self, channel: u32, filename: &str) {
        self.play(channel, filename, i32::MAX)
    }

    fn stop(&mut self, _channel: u32) {
        sdl2::mixer::Channel::all().fade_out(100);
    }
}
