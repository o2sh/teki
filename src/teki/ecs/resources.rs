use crate::teki::sdl::sdl_audio::SdlAudio;

pub struct SoundQueue {
    queue: Vec<(u32, &'static str)>,
}

impl SoundQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn flush(&mut self, audio: &mut SdlAudio) {
        for (channel, filename) in self.queue.iter() {
            audio.play(*channel, filename);
        }
        self.queue.clear();
    }

    pub fn push_play(&mut self, channel: u32, filename: &'static str) {
        self.queue.push((channel, filename));
    }
}
