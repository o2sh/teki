use sdl2::mixer::{Chunk, MAX_VOLUME};

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

    pub fn play_sound(&mut self, channel: u32, filename: &str) {
        self.play(channel, filename, 0)
    }

    pub fn play_loop(&mut self, channel: u32, filename: &str) {
        self.play(channel, filename, i32::MAX)
    }

    pub fn halt(&mut self) {
        sdl2::mixer::Channel::all().fade_out(100);
    }

    fn play(&mut self, channel: u32, filename: &str, loops: i32) {
        if channel < self.channels.len() as u32 {
            let path = format!("{}", filename);
            let mut chunk = Chunk::from_file(path).expect("play: No music flile");
            chunk.set_volume(self.base_volume);
            sdl2::mixer::Channel::all().play(&chunk, loops).expect("Play music failed");
            self.channels[channel as usize] = Some(chunk);
        }
    }
}
