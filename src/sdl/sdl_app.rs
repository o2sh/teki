use crate::sdl::SdlRenderer;
use counted_array::counted_array;
use lazy_static::lazy_static;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::Sdl;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, SystemTime};
use teki_common::traits::App;
use teki_common::utils::consts::*;
use teki_common::utils::pad::Key;

pub struct SdlApp<A: App<SdlRenderer>> {
    sdl_context: Sdl,
    last_update_time: SystemTime,
    app: A,
}

impl<A: App<SdlRenderer>> SdlApp<A> {
    pub fn new(app: A) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        Ok(Self { sdl_context, last_update_time: SystemTime::now(), app })
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

        let _audio = self.sdl_context.audio()?;

        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
        let _mixer_context = sdl2::mixer::init(
            sdl2::mixer::InitFlag::MP3
                | sdl2::mixer::InitFlag::FLAC
                | sdl2::mixer::InitFlag::MOD
                | sdl2::mixer::InitFlag::OGG,
        )?;

        // Number of mixing channels available for sound effect `Chunk`s to play
        // simultaneously.
        sdl2::mixer::allocate_channels(4);

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let mut renderer =
            SdlRenderer::new(canvas, ttf_context, (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));

        self.app.init(&mut renderer);

        self.last_update_time = SystemTime::now();
        let mut skip_count = 0;
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

            skip_count = self.wait_frame(Duration::from_micros(1_000_000 / FPS as u64));
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
    pub fn wait_frame(&mut self, duration: Duration) -> u32 {
        let next_update_time = self.last_update_time + duration;
        let now = SystemTime::now();
        if now < next_update_time {
            let wait = next_update_time.duration_since(now).expect("");
            thread::sleep(wait);
            self.last_update_time = next_update_time;
            0
        } else {
            let late = now.duration_since(next_update_time).expect("");
            let skip_count = (late.as_millis() as f32 / duration.as_millis() as f32).floor() as u32;
            if skip_count <= FPS / MIN_FPS {
                self.last_update_time = next_update_time + duration * skip_count;
                skip_count
            } else {
                self.last_update_time = now;
                FPS / MIN_FPS
            }
        }
    }
}

counted_array!(const KEY_MAP_TABLE: [(Keycode, Key); _] = [
    (Keycode::Space,  Key::Space),
    (Keycode::Escape, Key::Escape),
    (Keycode::Left,   Key::Left),
    (Keycode::Right,  Key::Right),
    (Keycode::Up,     Key::Up),
    (Keycode::Down,   Key::Down),
    (Keycode::Z,      Key::Z),

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
