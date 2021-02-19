use teki_common::game::RGBA;
use teki_common::traits::{App, Audio, Renderer, Timer};
use teki_common::utils::{
    consts::*,
    pad::{Key, Pad},
    FpsCalc,
};

use crate::app::character_select::CharacterSelect;
use crate::app::game::Game;
use crate::app::menu_title::Title;

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
        self.state = AppState::CharacterSelect(CharacterSelect::default());
    }

    fn start_game(&mut self, character_index: u8) {
        self.state = AppState::Game(Game::new(character_index));
        self.audio.play_music(CH_BG_MUSIC, BG_MUSIC);
    }

    fn back_to_title(&mut self, play: bool) {
        self.state = AppState::Title(Title);
        if play {
            self.audio.play_music(CH_BG_MUSIC, TITLE_MUSIC);
        }
    }
}

impl<R: Renderer, A: Audio, T: Timer> App<R> for EcsApp<A, T> {
    fn init(&mut self, renderer: &mut R) {
        renderer.load_textures(
            "assets/gfx",
            &[
                "marisa.png",
                "reimu.png",
                "enemy_a.png",
                "enemy_b.png",
                "orbs.png",
                "bg.png",
                "spells.png",
                "shockwave.png",
                "avatar_reimu.png",
                "avatar_marisa.png",
                "player_select.png",
                "menu_bg.png",
                "desc_char.png",
                "title.png",
                "items.png",
                "special.png",
                "big_orbs.png",
            ],
        );
        renderer.load_sprite_sheet("assets/gfx/marisa.json");
        renderer.load_sprite_sheet("assets/gfx/reimu.json");
        renderer.load_sprite_sheet("assets/gfx/enemy_a.json");
        renderer.load_sprite_sheet("assets/gfx/enemy_b.json");
        renderer.load_sprite_sheet("assets/gfx/orbs.json");
        renderer.load_sprite_sheet("assets/gfx/bg.json");
        renderer.load_sprite_sheet("assets/gfx/spells.json");
        renderer.load_sprite_sheet("assets/gfx/shockwave.json");
        renderer.load_sprite_sheet("assets/gfx/avatar_reimu.json");
        renderer.load_sprite_sheet("assets/gfx/avatar_marisa.json");
        renderer.load_sprite_sheet("assets/gfx/player_select.json");
        renderer.load_sprite_sheet("assets/gfx/desc_char.json");
        renderer.load_sprite_sheet("assets/gfx/title.json");
        renderer.load_sprite_sheet("assets/gfx/items.json");
        renderer.load_sprite_sheet("assets/gfx/special.json");
        renderer.load_sprite_sheet("assets/gfx/big_orbs.json");

        self.audio.load_musics("assets/bgm", &["stage01.ogg", "menu.ogg"]).expect("");
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
                AppState::CharacterSelect(_character_select) => self.back_to_title(false),
                _ => self.back_to_title(true),
            }
        }

        match &mut self.state {
            AppState::Title(title) => {
                if let Some(value) = title.update(&self.pad, &mut self.audio) {
                    if value {
                        self.select_character();
                    } else {
                        return false;
                    }
                }
            }
            AppState::CharacterSelect(character_select) => {
                if let Some((value, character_index)) =
                    character_select.update(&self.pad, &mut self.audio)
                {
                    if value {
                        self.start_game(character_index);
                    } else {
                        return false;
                    }
                }
            }
            AppState::Game(game) => {
                if !game.update(&self.pad, &mut self.audio) {
                    self.back_to_title(true);
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
            &RGBA { r: 255, g: 255, b: 255, a: 255 },
            false,
        );
    }
}
