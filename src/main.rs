mod consts;
mod ecs;
mod renderer;

use clap::{crate_description, crate_name, crate_version, App, Arg};

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use std::time::Duration;

use consts::*;
use ecs::components::*;
use ecs::systems::*;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

pub enum ShootCommand {
    Fire,
    Rest,
}

fn initialize_player(world: &mut World, spritesheet: usize) {
    let player_top_left_frame = Rect::new(0, 0, 45, 45);
    let (frame_width, frame_height) = player_top_left_frame.size();

    let sprite = Sprite {
        spritesheet,
        region: Rect::new(
            player_top_left_frame.x(),
            player_top_left_frame.y(),
            frame_width,
            frame_height,
        ),
    };

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Gun { fire: false })
        .with(Position(Point::new(122, 260)))
        .with(Velocity { speed: 0, direction: Direction::Right })
        .with(sprite)
        .build();
}

fn initialize_enemy(world: &mut World, spritesheet: usize, position: Point) {
    let enemy_top_left_frame = Rect::new(0, 0, 45, 45);
    let (frame_width, frame_height) = enemy_top_left_frame.size();

    let sprite = Sprite {
        spritesheet,
        region: Rect::new(
            enemy_top_left_frame.x(),
            enemy_top_left_frame.y(),
            frame_width,
            frame_height,
        ),
    };

    world
        .create_entity()
        .with(Enemy)
        .with(Position(position))
        .with(Velocity { speed: 0, direction: Direction::Right })
        .with(sprite)
        .build();
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
        video_subsystem.window(APP_NAME, WIDTH as u32 * scale, HEIGHT as u32 * scale);

    if fullscreen {
        window_builder.fullscreen();
    } else {
        window_builder.position_centered().resizable();
    }

    let window = window_builder.opengl().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(movement::Movement, "Movement", &["Keyboard"])
        .with(shooting::Shooting, "Shooting", &["Keyboard"])
        .with(ai::AI, "Ai", &[])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    let shoot_command: Option<ShootCommand> = None;
    world.insert(movement_command);
    world.insert(shoot_command);
    let (width, height) = canvas.output_size()?;
    world.insert((width, height));

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/neko.png")?,
        texture_creator.load_texture("assets/corgi.png")?,
    ];

    let player_spritesheet = 0;
    let enemy_spritesheet = 1;

    initialize_player(&mut world, player_spritesheet);
    initialize_enemy(&mut world, enemy_spritesheet, Point::new(10, 10));
    initialize_enemy(&mut world, enemy_spritesheet, Point::new(50, 50));
    initialize_enemy(&mut world, enemy_spritesheet, Point::new(100, 100));

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let mut movement_command = None;
        let mut shoot_command = None;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    shoot_command = Some(ShootCommand::Fire);
                }
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                    shoot_command = Some(ShootCommand::Rest);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;
        *world.write_resource() = shoot_command;
        let (width, height) = canvas.output_size()?;
        *world.write_resource() = (width, height);

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
