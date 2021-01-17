use crate::app::{components::*, resources::*, system_enemy::*, system_game::*, system_player::*};
use legion::*;
use teki_common::traits::App;
use teki_common::traits::Audio;
use teki_common::traits::Renderer;
use teki_common::traits::Timer;
use teki_common::utils::consts::*;
use teki_common::utils::math::*;
use teki_common::utils::pad::{Key, Pad, PadBit};
use teki_common::utils::FpsCalc;
use vector2d::Vector2D;

enum AppState {
    Title(Title),
    Game(Game),
}

pub struct EcsApp<A: Audio, T: Timer> {
    pressed_key: Option<Key>,
    state: AppState,
    pad: Pad,
    fps_calc: FpsCalc<T>,
    audio: A,
}

impl<A: Audio, T: Timer> EcsApp<A, T> {
    pub fn new(audio: A, timer: T) -> Self {
        Self {
            pressed_key: None,
            state: AppState::Title(Title),
            pad: Pad::default(),
            fps_calc: FpsCalc::new(timer),
            audio,
        }
    }

    fn start_game(&mut self) {
        self.state = AppState::Game(Game::new());
        self.audio.play_loop(CH_BG_MUSIC, BG_MUSIC);
    }

    fn back_to_title(&mut self) {
        self.audio.stop(CH_BG_MUSIC);
        self.state = AppState::Title(Title);
    }
}

impl<R: Renderer, A: Audio, T: Timer> App<R> for EcsApp<A, T> {
    fn init(&mut self, renderer: &mut R) {
        renderer.load_textures(
            "assets",
            &["sprites.png", "font.png", "hero.png", "tileset.png", "enemy.png", "orbs.png"],
        );
        renderer.load_sprite_sheet("assets/sprites.json");
        renderer.load_sprite_sheet("assets/hero.json");
        renderer.load_sprite_sheet("assets/tileset.json");
        renderer.load_sprite_sheet("assets/enemy.json");
        renderer.load_sprite_sheet("assets/orbs.json");
    }

    fn on_key(&mut self, key: Key, down: bool) {
        self.pad.on_key(key, down);
        if down {
            self.pressed_key = Some(key);
        }
    }

    fn update(&mut self) -> bool {
        self.pad.update();
        if self.pressed_key == Some(Key::Escape) {
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

    fn draw(&mut self, renderer: &mut R) {
        renderer.draw_bg(BG_TEXTURE, WINDOW_WIDTH, WINDOW_HEIGHT, 0);
        match &self.state {
            AppState::Title(title) => title.draw(renderer),
            AppState::Game(game) => game.draw(renderer),
        }

        self.fps_calc.update();

        renderer.draw_str(
            FONTS,
            WINDOW_WIDTH - 7 * 16,
            WINDOW_HEIGHT - 30,
            &format!("FPS {:2}", self.fps_calc.fps()),
            255,
            255,
            255,
        );
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

    fn draw<R: Renderer>(&self, renderer: &mut R) {
        let title = "TEKI";
        renderer.draw_str(
            FONTS,
            (WINDOW_WIDTH / 2) - (title.len() as i32 / 2) * 16,
            8 * 16,
            title,
            255,
            255,
            255,
        );

        let msg = "PRESS SPACE KEY TO START";
        renderer.draw_str(
            FONTS,
            (WINDOW_WIDTH / 2) - (msg.len() as i32 / 2) * 16,
            25 * 16,
            msg,
            255,
            255,
            255,
        );
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
            .add_system(update_game_info_system())
            .add_system(move_player_system())
            .add_system(fire_myshot_system())
            .add_system(move_myshot_system())
            .add_system(spawn_enemy_system())
            .add_system(update_enemy_formation_system())
            .add_system(animate_enemy_system())
            .add_system(animate_player_system())
            .flush()
            .add_system(move_enemy_formation_system())
            .add_system(collision_check_system())
            .build();
        let mut world = World::default();
        world.push((
            new_player(),
            Position(Vector2D::new(CENTER_X, PLAYER_Y)),
            player_hit_box(),
            player_sprite(),
        ));
        let mut resources = Resources::default();
        resources.insert(SoundQueue::new());
        resources.insert(EnemyFormation::default());
        resources.insert(GameInfo::default());
        Self { world, resources, schedule }
    }

    fn update<A: Audio>(&mut self, pad: &Pad, audio: &mut A) -> bool {
        self.resources.insert(pad.clone());
        self.schedule.execute(&mut self.world, &mut self.resources);
        let mut sound_queue = self.resources.get_mut::<SoundQueue>().unwrap();
        sound_queue.flush(audio);
        true
    }

    fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_gradient(GAME_WIDTH, GAME_HEIGHT, PADDING);

        for (position, drawable) in <(&Position, &SpriteDrawable)>::query().iter(&self.world) {
            let pos = round_vec(&position.0) + drawable.offset;
            renderer.draw_sprite(drawable.sprite_name, &pos);
        }

        if let Some(game_info) = self.get_game_info() {
            game_info.draw(renderer);
        }
    }

    fn get_game_info(&self) -> Option<legion::systems::Fetch<'_, GameInfo>> {
        self.resources.get::<GameInfo>()
    }
}
