mod teki;

use clap::{crate_description, crate_name, crate_version, App, Arg};

use crate::teki::sdl::sdl_audio::SdlAudio;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::Sdl;
use std::time::Duration;
use teki::ecs::app::EcsApp;
use teki::sdl::sdl_renderer::SdlRenderer;

use teki::utils::consts::*;

#[derive(Clone, Copy, PartialEq)]

pub enum VKey {
    Space,
    Escape,
    Left,
    Right,
    Up,
    Down,
}

fn main() -> Result<(), String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("full").help("Use fullscreen").short("f").long("fullscreen"))
        .arg(
            Arg::with_name("scale")
                .help("Specify window scale (default: 3)")
                .short("s")
                .long("scale")
                .takes_value(true),
        )
        .get_matches();
    let fullscreen = matches.is_present("full");
    let scale = if let Some(scale) = matches.value_of("scale") {
        String::from(scale).parse().unwrap()
    } else {
        3
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let mut window_builder =
        video_subsystem.window(APP_NAME, WINDOW_WIDTH as u32 * scale, WINDOW_HEIGHT as u32 * scale);

    if fullscreen {
        window_builder.fullscreen();
    } else {
        window_builder.position_centered().resizable();
    }

    let window = window_builder.opengl().build().map_err(|e| e.to_string())?;

    if fullscreen {
        sdl_context.mouse().show_cursor(false);
    }

    let canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut renderer = SdlRenderer::new(canvas, (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));

    let audio = SdlAudio::new(CHANNEL_COUNT, BASE_VOLUME);

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;

    let mut app = EcsApp::new(audio);

    let skip_count = 0;
    'running: loop {
        if !pump_events(&sdl_context, &mut app)? {
            break 'running;
        }
        let step = 1 + skip_count;

        for _ in 0..step {
            if !app.update() {
                break 'running;
            }
        }
        app.draw(&mut renderer);
        renderer.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
    Ok(())
}

fn pump_events(sdl_context: &Sdl, app: &mut EcsApp) -> Result<bool, String> {
    let mut event_pump = sdl_context.event_pump()?;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                return Ok(false);
            }
            Event::KeyDown { keycode: Some(key), .. } => {
                app.on_key(key, true);
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                app.on_key(key, false);
            }
            _ => {}
        }
    }
    Ok(true)
}
