use crate::app::{
    components::*, resources::*, system_avatar::*, system_effect::*, system_enemy::*,
    system_game::*, system_player::*,
};
use legion::*;
use teki_common::traits::{App, Audio, Renderer, Timer};
use teki_common::utils::{
    consts::*,
    math::*,
    pad::{Key, Pad, PadBit},
    FpsCalc,
};
use vector2d::Vector2D;

enum AppState {
    Title(Title),
    CharacterSelect(CharacterSelect),
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

    fn select_character(&mut self) {
        self.state = AppState::CharacterSelect(CharacterSelect);
    }

    fn start_game(&mut self) {
        self.state = AppState::Game(Game::new());
        self.audio.play_music(CH_BG_MUSIC, BG_MUSIC);
    }

    fn back_to_title(&mut self) {
        self.state = AppState::Title(Title);
        self.audio.play_music(CH_BG_MUSIC, TITLE_MUSIC);
    }
}

impl<R: Renderer, A: Audio, T: Timer> App<R> for EcsApp<A, T> {
    fn init(&mut self, renderer: &mut R) {
        renderer.load_textures(
            "assets",
            &[
                "sanae.png",
                "marisa.png",
                "reimu.png",
                "enemy.png",
                //"orbs.png",
                "bg.png",
                "spells.png",
                "shockwave.png",
                "a_reimu.png",
                "a_marisa.png",
                "title_bg.png",
            ],
        );
        renderer.load_sprite_sheet("assets/sanae.json");
        renderer.load_sprite_sheet("assets/marisa.json");
        renderer.load_sprite_sheet("assets/reimu.json");
        renderer.load_sprite_sheet("assets/enemy.json");
        //renderer.load_sprite_sheet("assets/orbs.json");
        renderer.load_sprite_sheet("assets/bg.json");
        renderer.load_sprite_sheet("assets/spells.json");
        renderer.load_sprite_sheet("assets/shockwave.json");
        renderer.load_sprite_sheet("assets/a_reimu.json");
        renderer.load_sprite_sheet("assets/a_marisa.json");
        renderer.load_sprite_sheet("assets/title_bg.json");

        self.audio.load_musics("assets/audio", &["bgm.ogg", "title.ogg"]).expect("");
    }

    fn start_title_song(&mut self) {
        self.audio.play_music(CH_BG_MUSIC, TITLE_MUSIC);
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
                        self.select_character();
                    } else {
                        return false;
                    }
                }
            }
            AppState::CharacterSelect(character_select) => {
                if let Some(value) = character_select.update(&self.pad) {
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
        renderer.set_draw_color(0, 0, 0);
        renderer.clear();
        match &self.state {
            AppState::Title(title) => title.draw(renderer),
            AppState::CharacterSelect(character_select) => character_select.draw(renderer),
            AppState::Game(game) => game.draw(renderer),
        }

        self.fps_calc.update();

        renderer.draw_str(
            RE_FONT,
            WINDOW_WIDTH - 7 * 8,
            WINDOW_HEIGHT - 28,
            16,
            &format!("{:2}fps", self.fps_calc.fps()),
            255,
            255,
            255,
            255,
            false,
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
        renderer.clear();
        renderer.draw_gradient(WINDOW_WIDTH, WINDOW_HEIGHT);
        renderer.draw_sprite("title_bg", &Vector2D::new(0, 0));
        let title = "Teki";
        renderer.draw_str(
            IM_FONT,
            300 + ((WINDOW_WIDTH - 300) / 2) - (title.len() as i32 / 2) * 8,
            8 * 16,
            17,
            title,
            255,
            255,
            255,
            255,
            true,
        );

        let msg = "Press space key to start";
        renderer.draw_str(
            IM_FONT,
            300 + ((WINDOW_WIDTH - 300) / 2) - (msg.len() as i32 / 2) * 8,
            15 * 16,
            17,
            msg,
            255,
            255,
            255,
            255,
            true,
        );
    }
}

struct CharacterSelect;

impl CharacterSelect {
    fn update(&mut self, pad: &Pad) -> Option<bool> {
        if pad.is_trigger(PadBit::Z) {
            return Some(true);
        }
        None
    }

    fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.clear();

        let msg = "Character Select";
        renderer.draw_str(
            IM_FONT,
            300 + ((WINDOW_WIDTH - 300) / 2) - (msg.len() as i32 / 2) * 8,
            15 * 16,
            17,
            msg,
            255,
            255,
            255,
            255,
            true,
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
            .add_system(animate_avatar_system())
            .flush()
            .add_system(move_enemy_formation_system())
            .add_system(collision_check_system())
            .add_system(move_sequential_anime_system())
            .build();
        let mut world = World::default();
        world.push((
            new_player(),
            Position(Vector2D::new(CENTER_X, PLAYER_Y)),
            player_hit_box(),
            player_sprite(),
        ));

        world.push((
            Avatar,
            Position(Vector2D::new(
                (GAME_WIDTH + (WINDOW_WIDTH - GAME_WIDTH) / 2) * ONE,
                PLAYER_Y - 25 * ONE,
            )),
            avatar_sprite(),
        ));
        let mut resources = Resources::default();
        resources.insert(SoundQueue::new());
        resources.insert(EnemyFormation::default());
        resources.insert(GameInfo::new());
        resources.insert(StageIndicator::default());
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
        renderer.clear();
        renderer.draw_scrolling_bg(BG1_TEXTURE, GAME_WIDTH, GAME_HEIGHT);
        renderer.draw_vertical_separation(GAME_WIDTH, GAME_HEIGHT);
        for (position, drawable) in <(&Position, &SpriteDrawable)>::query().iter(&self.world) {
            let pos = round_vec(&position.0) + drawable.offset;
            renderer.draw_sprite(drawable.sprite_name, &pos);
        }

        if let Some(game_info) = self.get_game_info() {
            game_info.draw(renderer);

            match game_info.game_state {
                GameState::StartStage => {
                    if let Some(stage_indicator) = self.get_stage_indicator() {
                        stage_indicator.draw(renderer, game_info.count);
                    }
                }
                _ => {}
            }
        }
    }

    fn get_game_info(&self) -> Option<legion::systems::Fetch<'_, GameInfo>> {
        self.resources.get::<GameInfo>()
    }

    fn get_stage_indicator(&self) -> Option<legion::systems::Fetch<'_, StageIndicator>> {
        self.resources.get::<StageIndicator>()
    }
}
