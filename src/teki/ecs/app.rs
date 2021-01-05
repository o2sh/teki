use crate::{
    teki::{
        ecs::{components::*, resources::*, system_enemy::*, system_player::*},
        sdl::sdl_audio::SdlAudio,
        utils::{
            consts::*,
            fps_calc::FpsCalc,
            pad::{Pad, PadBit},
        },
    },
    SdlRenderer,
};

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
    fps_calc: FpsCalc,
    audio: SdlAudio,
}

impl EcsApp {
    pub fn new(audio: SdlAudio) -> Self {
        Self {
            pressed_key: None,
            state: AppState::Title(Title),
            pad: Pad::default(),
            fps_calc: FpsCalc::new(),
            audio,
        }
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
                if !game.update(&self.pad, &mut self.audio) {
                    self.back_to_title();
                }
            }
        };
        self.pressed_key = None;
        true
    }

    pub fn draw(&mut self, renderer: &mut SdlRenderer) {
        renderer.clear();
        renderer.set_draw_gradient();
        match &self.state {
            AppState::Title(title) => title.draw(renderer),
            AppState::Game(game) => game.draw(renderer),
        }

        self.fps_calc.update();

        renderer.draw_str(
            FONTS,
            WINDOW_WIDTH - 6 * 8,
            1 * 8,
            &format!("FPS{:2}", self.fps_calc.fps()),
        );
    }

    fn start_game(&mut self) {
        self.state = AppState::Game(Game::new());
        self.audio.play_loop(1, BG_LOOP);
    }

    fn back_to_title(&mut self) {
        self.audio.halt();
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
        let title = "TEKI";
        renderer.draw_str(FONTS, (WINDOW_WIDTH / 2) - (title.len() as i32 / 2) * 8, 8 * 8, title);

        let msg = "PRESS SPACE KEY TO START";
        renderer.draw_str(FONTS, (WINDOW_WIDTH / 2) - (msg.len() as i32 / 2) * 8, 25 * 8, msg);
    }
}

struct Game {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl Game {
    fn new() -> Self {
        let schedule = Schedule::builder()
            .add_system(move_player_system())
            .add_system(fire_myshot_system())
            .add_system(move_myshot_system())
            .add_system(spawn_enemy_system())
            .add_system(update_enemy_formation_system())
            .flush()
            .add_system(move_enemy_formation_system())
            .build();
        let mut world = World::default();
        world.push((new_player(), Position(Point::new(CENTER_X, PLAYER_Y)), player_sprite()));
        let mut resources = Resources::default();
        resources.insert(SoundQueue::new());
        resources.insert(EnemyFormation::default());
        Self { world, resources, schedule }
    }

    fn update(&mut self, pad: &Pad, audio: &mut SdlAudio) -> bool {
        self.resources.insert(pad.clone());
        self.schedule.execute(&mut self.world, &mut self.resources);
        let mut sound_queue = self.resources.get_mut::<SoundQueue>().unwrap();
        sound_queue.flush(audio);
        true
    }

    fn draw(&self, renderer: &mut SdlRenderer) {
        renderer.draw_bg(WATER_TEXTURE, false);

        for (position, drawable) in <(&Position, &SpriteDrawable)>::query().iter(&self.world) {
            renderer.draw_sprite(drawable, position.0);
        }
    }
}
