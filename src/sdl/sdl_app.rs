use crate::sdl::SdlRenderer;
use counted_array::counted_array;
use lazy_static::lazy_static;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use std::collections::HashMap;
use std::time::Duration;
use teki_common::traits::App;
use teki_common::utils::pad::Key;

use teki_common::utils::consts::*;

pub struct SdlApp<A: App<SdlRenderer>> {
    sdl_context: Sdl,
    app: A,
}

impl<A: App<SdlRenderer>> SdlApp<A> {
    pub fn new(app: A) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        Ok(Self { sdl_context, app })
    }

    pub fn run(&mut self, scale: u32, fullscreen: bool) -> Result<(), String> {
        let video_subsystem = self.sdl_context.video()?;
        let _image_context = image::init(InitFlag::PNG)?;

        let mut window_builder = video_subsystem.window(
            APP_NAME,
            WINDOW_WIDTH as u32 * scale,
            WINDOW_HEIGHT as u32 * scale,
        );

        if fullscreen {
            window_builder.fullscreen();
        } else {
            window_builder.position_centered().resizable();
        }

        let window = window_builder.opengl().build().map_err(|e| e.to_string())?;

        if fullscreen {
            self.sdl_context.mouse().show_cursor(false);
        }

        let canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

        let mut renderer = SdlRenderer::new(canvas, (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));

        self.app.init(&mut renderer);

        let skip_count = 0;
        'running: loop {
            if !self.pump_events()? {
                break 'running;
            }
            let step = 1 + skip_count;

            for _ in 0..step {
                if !self.app.update() {
                    break 'running;
                }
            }
            self.app.draw(&mut renderer);
            renderer.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }

        Ok(())
    }

    pub fn pump_events(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return Ok(false);
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(vkey) = map_key(key) {
                        self.app.on_key(vkey, true);
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(vkey) = map_key(key) {
                        self.app.on_key(vkey, false);
                    }
                }
                _ => {}
            }
        }
        Ok(true)
    }
}

counted_array!(const KEY_MAP_TABLE: [(Keycode, Key); _] = [
    (Keycode::Space,  Key::Space),
    (Keycode::Escape, Key::Escape),
    (Keycode::Left,   Key::Left),
    (Keycode::Right,  Key::Right),
    (Keycode::Up,     Key::Up),
    (Keycode::Down,   Key::Down),

]);

lazy_static! {
    static ref KEY_MAP: HashMap<Keycode, Key> = {
        let mut m = HashMap::new();
        for &(keycode, vkey) in KEY_MAP_TABLE.iter() {
            m.insert(keycode, vkey);
        }
        m
    };
}

fn map_key(keycode: Keycode) -> Option<Key> {
    KEY_MAP.get(&keycode).copied()
}
