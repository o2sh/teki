extern crate js_sys;

use wasm_bindgen::prelude::*;

use teki_common::traits::App;
use teki_common::utils::pad::Key;

use teki_ecs::app::ecs_app::EcsApp;

use crate::wasm::wasm_audio::WasmAudio;
use crate::wasm::wasm_renderer::WasmRenderer;
use crate::wasm::wasm_timer::WasmTimer;

#[wasm_bindgen]
pub struct WasmApp {
    app: Box<dyn App<WasmRenderer>>,
    renderer: WasmRenderer,
}

#[wasm_bindgen]
impl WasmApp {
    pub fn new(mut renderer: WasmRenderer, get_now_fn: js_sys::Function) -> Self {
        crate::utils::set_panic_hook();
        let audio = WasmAudio::new();
        let timer = WasmTimer::new(move || {
            let this = JsValue::NULL;
            if let Ok(v) = get_now_fn.call0(&this) {
                if let Some(t) = v.as_f64() {
                    return t;
                }
            }
            0.0
        });
        let mut app = EcsApp::new(audio, timer);

        app.init(&mut renderer);

        Self { app: Box::new(app), renderer }
    }

    pub fn start_title_song(&mut self) {
        self.app.start_title_song()
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
        "KeyZ" => Some(Key::Z),
        "KeyX" => Some(Key::X),
        "Space" => Some(Key::Space),
        "Escape" => Some(Key::Escape),
        "ArrowLeft" => Some(Key::Left),
        "ArrowRight" => Some(Key::Right),
        "ArrowUp" => Some(Key::Up),
        "ArrowDown" => Some(Key::Down),
        _ => None,
    }
}
