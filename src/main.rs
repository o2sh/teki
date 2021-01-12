mod sdl;
mod std_timer;

use crate::sdl::SdlApp;
use clap::{crate_description, crate_name, crate_version, App, Arg};

use crate::sdl::SdlAudio;
use crate::std_timer::StdTimer;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use teki_common::utils::consts::*;
use teki_ecs::app::EcsApp;

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
    let audio = SdlAudio::new(CHANNEL_COUNT, BASE_VOLUME);

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;

    sdl2::mixer::allocate_channels(4);

    let timer = StdTimer::new();
    let mut app = SdlApp::new(EcsApp::new(audio, timer))?;

    app.run(scale, fullscreen)?;

    Ok(())
}
