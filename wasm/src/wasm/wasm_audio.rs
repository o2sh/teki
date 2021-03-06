use teki_common::traits::Audio;
use teki_common::utils::consts::{BGM_VOLUME, SFX_VOLUME};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn play_se(channel: u32, filename: &str, volume: f32);
}

#[wasm_bindgen]
extern "C" {
    fn play_loop(channel: u32, filename: &str, volume: f32);
}

pub struct WasmAudio;

impl WasmAudio {
    pub fn new() -> Self {
        Self
    }
}

impl Audio for WasmAudio {
    fn load_musics(&mut self, _: &str, _: &[&str]) -> Result<(), String> {
        Ok(())
    }

    fn play_sound(&mut self, channel: u32, filename: &str) {
        play_se(channel, filename, SFX_VOLUME);
    }

    fn play_music(&mut self, channel: u32, filename: &str) {
        play_loop(channel, &format!("assets/bgm/{}", filename), BGM_VOLUME);
    }
}
