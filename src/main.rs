mod sdl;
mod std_timer;

use crate::sdl::{SdlApp, SdlAudio};
use crate::std_timer::StdTimer;
use clap::{crate_description, crate_name, crate_version, App};

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
    App::new(crate_name!()).version(crate_version!()).about(crate_description!());

    let audio = SdlAudio::new(CHANNEL_COUNT);

    let timer = StdTimer::new();
    let mut app = SdlApp::new(EcsApp::new(audio, timer))?;

    app.run()?;

    Ok(())
}
