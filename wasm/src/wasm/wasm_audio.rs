use wasm_bindgen::prelude::*;

use teki_common::traits::Audio;

#[wasm_bindgen]
extern "C" {
    fn play_se(channel: u32, filename: &str);
}

#[wasm_bindgen]
extern "C" {
    fn play_loop(channel: u32, filename: &str);
}

#[wasm_bindgen]
extern "C" {
    fn stop(channel: u32);
}

pub struct WasmAudio;

impl WasmAudio {
    pub fn new() -> Self {
        Self
    }
}

impl Audio for WasmAudio {
    fn play_sound(&mut self, channel: u32, filename: &str) {
        play_se(channel, filename);
    }

    fn play_loop(&mut self, channel: u32, filename: &str) {
        play_loop(channel, filename);
    }
    fn stop(&mut self, channel: u32) {
        stop(channel);
    }
}
