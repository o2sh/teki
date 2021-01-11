use wasm_bindgen::prelude::*;

use teki_common::traits::App;
use teki_common::utils::pad::Key;

use teki_ecs::app::ecs_app::EcsApp;

use crate::wasm_audio::WasmAudio;
use crate::wasm_renderer::WasmRenderer;

#[wasm_bindgen]
pub struct WasmApp {
    app: Box<dyn App<WasmRenderer>>,
    renderer: WasmRenderer,
}

#[wasm_bindgen]
impl WasmApp {
    pub fn new(mut renderer: WasmRenderer) -> Self {
        let audio = WasmAudio::new();

        let mut app = EcsApp::new(audio);

        app.init(&mut renderer);

        Self { app: Box::new(app), renderer }
    }

    pub fn on_key(&mut self, key_code: &str, down: bool) {
        if let Some(key) = to_key(key_code) {
            self.app.on_key(key, down);
        }
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn draw(&mut self) {
        self.app.draw(&mut self.renderer);
    }
}

fn to_key(key_code: &str) -> Option<Key> {
    match key_code {
        "Space" => Some(Key::Space),
        "Escape" => Some(Key::Escape),
        "ArrowLeft" => Some(Key::Left),
        "ArrowRight" => Some(Key::Right),
        "ArrowUp" => Some(Key::Up),
        "ArrowDown" => Some(Key::Down),
        _ => None,
    }
}
