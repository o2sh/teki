mod sdl;
mod std_timer;

use crate::sdl::SdlApp;
use clap::{crate_description, crate_name, crate_version, App, Arg};

use crate::sdl::SdlAudio;
use crate::std_timer::StdTimer;
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
        .get_matches();
    let fullscreen = matches.is_present("full");

    let audio = SdlAudio::new(CHANNEL_COUNT, BASE_VOLUME);

    let timer = StdTimer::new();
    let mut app = SdlApp::new(EcsApp::new(audio, timer))?;

    app.run(1, fullscreen)?;

    Ok(())
}
