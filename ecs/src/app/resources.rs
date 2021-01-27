use teki_common::game::formation_table::*;
use teki_common::game::{FormationIndex, RGBA};
use teki_common::traits::{Audio, Renderer};
use teki_common::utils::{consts::*, math::*};
use vector2d::Vector2D;

#[derive(PartialEq)]
pub enum GameState {
    StartStage,
    Playing,
}

pub struct SoundQueue {
    queue: Vec<(u32, &'static str)>,
}

impl SoundQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn flush<A: Audio>(&mut self, audio: &mut A) {
        for (channel, filename) in self.queue.iter() {
            audio.play_sound(*channel, filename);
        }
        self.queue.clear();
    }

    pub fn push_play(&mut self, channel: u32, filename: &'static str) {
        self.queue.push((channel, filename));
    }
}

pub struct GameInfo {
    pub stage: u16,
    pub score: u32,
    pub count: u32,
    pub game_state: GameState,
    pub frame_count: u32,
    pub frame_count_over_2: u32,
    pub frame_count_over_5: u32,
    pub frame_count_over_10: u32,
}

impl GameInfo {
    pub fn new() -> Self {
        GameInfo {
            stage: 0,
            score: 0,
            count: 0,
            game_state: GameState::StartStage,
            frame_count: 0,
            frame_count_over_2: 0,
            frame_count_over_5: 0,
            frame_count_over_10: 0,
        }
    }

    pub fn add_score(&mut self, add: u32) {
        self.score += add;
    }

    pub fn update(&mut self, stage_indicator: &mut StageIndicator) {
        self.frame_count = self.frame_count.wrapping_add(1);

        if self.frame_count % 5 == 0 {
            self.frame_count_over_5 = self.frame_count_over_5.wrapping_add(1);
        }

        if self.frame_count % 10 == 0 {
            self.frame_count_over_10 = self.frame_count_over_10.wrapping_add(1);
        }

        if self.frame_count % 2 == 0 {
            self.frame_count_over_2 = self.frame_count_over_2.wrapping_add(1);
        }

        match self.game_state {
            GameState::StartStage => {
                if self.count == 0 {
                    stage_indicator.set_stage();
                }
                self.count += 1;
                if self.count >= 180 as u32 {
                    self.game_state = GameState::Playing;
                }
            }
            _ => {}
        }
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        let left_padding = 20;
        let top_padding = 30;
        renderer.draw_str(
            RE_FONT,
            GAME_WIDTH + left_padding,
            top_padding,
            16,
            "Score",
            &RGBA { r: 255, g: 0, b: 0, a: 255 },
            false,
        );
        renderer.draw_str(
            RE_FONT,
            GAME_WIDTH + left_padding + 8 * 8,
            top_padding,
            16,
            &format!("{}", self.score),
            &RGBA { r: 255, g: 255, b: 255, a: 255 },
            false,
        );
    }
}

#[derive(Default)]
pub struct StageIndicator {
    stage: u16,
}

impl StageIndicator {
    pub fn set_stage(&mut self) {
        self.stage += 1;
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R, count: u32) {
        let gradient = (180.0 - count as f32) / 180.0;
        let rect_alpha_gradient = (RECT_STAGE_ALPHA as f32 * gradient).floor() as u8;
        let text_alpha_gradient = (TEXT_STAGE_ALPHA as f32 * gradient).floor() as u8;

        renderer.draw_rect(
            &Vector2D::new(0, GAME_HEIGHT / 2 - 50),
            GAME_WIDTH,
            100,
            RGBA { r: 0, g: 0, b: 0, a: rect_alpha_gradient },
        );
        renderer.draw_str(
            IM_FONT,
            GAME_WIDTH / 2 - 4 * 7,
            GAME_HEIGHT / 2 - 30,
            16,
            &format!("Stage {}", self.stage),
            &RGBA { r: 255, g: 255, b: 255, a: text_alpha_gradient },
            false,
        );

        renderer.draw_str(
            IM_FONT,
            GAME_WIDTH / 2 - 7 * 8,
            GAME_HEIGHT / 2 + 15,
            16,
            &format!("From the Ashes"),
            &RGBA { r: 255, g: 255, b: 255, a: text_alpha_gradient },
            false,
        );

        renderer.draw_str(
            RE_FONT,
            GAME_WIDTH - 8 * 38 - 15,
            GAME_HEIGHT - 28,
            16,
            &format!("BGM: Thirty-three Heavenly War Maidens"),
            &RGBA { r: 255, g: 255, b: 255, a: text_alpha_gradient },
            false,
        );
    }
}

pub struct Formation {
    x_indices: [i32; X_COUNT],
    y_indices: [i32; Y_COUNT],
    pub done_appearance: bool,
}

impl Default for Formation {
    fn default() -> Self {
        let mut formation = Self {
            x_indices: Default::default(),
            y_indices: Default::default(),
            done_appearance: false,
        };
        formation.init();
        formation
    }
}

impl Formation {
    pub fn init(&mut self) {
        *self = Self { done_appearance: false, ..*self };

        for j in 0..X_COUNT {
            self.x_indices[j] = BASE_X_TABLE[j] * ONE;
        }
        for i in 0..Y_COUNT {
            self.y_indices[i] = BASE_Y_TABLE[i] * ONE;
        }
    }

    pub fn pos(&self, formation_index: &FormationIndex) -> Vector2D<i32> {
        Vector2D::new(self.x_indices[formation_index.0], self.y_indices[formation_index.1])
    }
}
