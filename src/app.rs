use crate::components::*;
use crate::consts::*;
use crate::pad::{Pad, PadBit};
use crate::system_player::*;
use crate::SdlRenderer;
use legion::*;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

enum AppState {
    Title(Title),
    Game(Game),
}

pub struct EcsApp {
    pressed_key: Option<Keycode>,
    state: AppState,
    pad: Pad,
}

impl EcsApp {
    pub fn new() -> Self {
        Self { pressed_key: None, state: AppState::Title(Title), pad: Pad::default() }
    }

    pub fn on_key(&mut self, key: Keycode, down: bool) {
        self.pad.on_key(key, down);
        if down {
            self.pressed_key = Some(key);
        }
    }

    pub fn update(&mut self) -> bool {
        self.pad.update();
        if self.pressed_key == Some(Keycode::Escape) {
            match &self.state {
                AppState::Title(_title) => {
                    self.pressed_key = None;
                    return false;
                }
                _ => self.back_to_title(),
            }
        }

        match &mut self.state {
            AppState::Title(title) => {
                if let Some(value) = title.update(&self.pad) {
                    if value {
                        self.start_game();
                    } else {
                        return false;
                    }
                }
            }
            AppState::Game(game) => {
                if !game.update(&self.pad) {
                    self.back_to_title();
                }
            }
        };
        self.pressed_key = None;
        true
    }

    pub fn draw(&mut self, renderer: &mut SdlRenderer) {
        match &self.state {
            AppState::Title(title) => title.draw(renderer),
            AppState::Game(game) => game.draw(renderer),
        }
    }

    fn start_game(&mut self) {
        self.state = AppState::Game(Game::new());
    }

    fn back_to_title(&mut self) {
        self.state = AppState::Title(Title);
    }
}

struct Title;

impl Title {
    fn update(&mut self, pad: &Pad) -> Option<bool> {
        if pad.is_pressed(PadBit::A) {
            return Some(true);
        }
        None
    }

    fn draw(&self, renderer: &mut SdlRenderer) {
        renderer.set_draw_color(0, 0, 0);
        renderer.clear();

        renderer.draw_str("assets/font.png", 10 * 9, 8 * 8, "TEKI");

        let msg = "PRESS SPACE KEY TO START";
        renderer.draw_str("assets/font.png", (28 - msg.len() as i32) / 2 * 8, 25 * 8, msg);
    }
}

struct Game {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl Game {
    fn new() -> Self {
        let schedule = Schedule::builder().add_system(move_player_system()).build();
        let mut world = World::default();
        world.push((new_player(), Position(Point::new(CENTER_X, PLAYER_Y)), player_sprite()));
        let mut resources = Resources::default();

        Self { world, resources, schedule }
    }

    fn update(&mut self, pad: &Pad) -> bool {
        self.resources.insert(pad.clone());
        self.schedule.execute(&mut self.world, &mut self.resources);
        true
    }

    fn draw(&self, renderer: &mut SdlRenderer) {
        renderer.draw_bg("assets/water.png");

        for (position, drawable) in <(&Position, &SpriteDrawable)>::query().iter(&self.world) {
            renderer.draw_sprite(drawable.sprite_name, position.0);
        }
    }
}
