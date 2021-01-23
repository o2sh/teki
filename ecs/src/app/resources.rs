use array_macro::*;
use lazy_static::lazy_static;
use teki_common::traits::{Audio, Renderer};
use teki_common::utils::{consts::*, math::*};
use teki_common::FormationIndex;
use vector2d::Vector2D;

pub const MARGIN: i32 = 20;

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

pub struct EnemyFormation {
    x_indices: [i32; X_COUNT],
    y_indices: [i32; Y_COUNT],
    to_left: bool,
    pub done_appearance: bool,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        let mut formation = Self {
            x_indices: Default::default(),
            y_indices: Default::default(),
            done_appearance: false,
            to_left: false,
        };
        formation.init();
        formation
    }
}

impl EnemyFormation {
    pub fn init(&mut self) {
        *self = Self { done_appearance: false, ..*self };

        for j in 0..X_COUNT {
            self.x_indices[j] = BASE_X_TABLE[j] * ONE;
        }
        for i in 0..Y_COUNT {
            self.y_indices[i] = BASE_Y_TABLE[i] * ONE;
        }
    }

    pub fn update(&mut self) {
        let space = GAME_WIDTH - X_COUNT as i32 * 32;
        let dx = space * ONE / 256;

        let dx = if self.to_left { -dx } else { dx };

        for i in 0..X_COUNT {
            self.x_indices[i] += dx;
        }

        if self.x_indices[0] < (MARGIN + 24) * ONE
            || self.x_indices[X_COUNT - 1] > (GAME_WIDTH - MARGIN - 24) * ONE
        {
            self.to_left = !self.to_left;
        }
    }

    pub fn pos(&self, formation_index: &FormationIndex) -> Vector2D<i32> {
        Vector2D::new(self.x_indices[formation_index.0], self.y_indices[formation_index.1])
    }
}

lazy_static! {
    pub static ref BASE_X_TABLE: [i32; X_COUNT] = {
        let cx = GAME_WIDTH / 2;
        let w = 48;

        array![|j|
            cx - ((X_COUNT - 1) as i32) * w / 2 + (j as i32) * w
        ; X_COUNT]
    };
    pub static ref BASE_Y_TABLE: [i32; Y_COUNT] = {
        let h = 50;

        array![|i|
            BASE_Y + (i as i32) * h
        ; Y_COUNT]
    };
}

pub struct GameInfo {
    pub score: u32,
    pub count: u32,
    pub frame_count: u32,
    pub frame_count_over_2: u32,
    pub frame_count_over_5: u32,
    pub game_state: GameState,
}

impl GameInfo {
    pub fn new() -> Self {
        GameInfo {
            score: 0,
            count: 0,
            game_state: GameState::StartStage,
            frame_count: 0,
            frame_count_over_2: 0,
            frame_count_over_5: 0,
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
            255,
            0,
            0,
            255,
            false,
        );
        renderer.draw_str(
            RE_FONT,
            GAME_WIDTH + left_padding + 8 * 8,
            top_padding,
            16,
            &format!("{}", self.score),
            255,
            255,
            255,
            255,
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
            0,
            0,
            0,
            rect_alpha_gradient,
        );
        renderer.draw_str(
            IM_FONT,
            GAME_WIDTH / 2 - 4 * 7,
            GAME_HEIGHT / 2 - 30,
            16,
            &format!("Stage {}", self.stage),
            255,
            255,
            255,
            text_alpha_gradient,
            false,
        );

        renderer.draw_str(
            IM_FONT,
            GAME_WIDTH / 2 - 7 * 8,
            GAME_HEIGHT / 2 + 15,
            16,
            &format!("From the Ashes"),
            255,
            255,
            255,
            text_alpha_gradient,
            false,
        );

        renderer.draw_str(
            RE_FONT,
            GAME_WIDTH - 8 * 38 - 15,
            GAME_HEIGHT - 28,
            16,
            &format!("BGM: Thirty-three Heavenly War Maidens"),
            255,
            255,
            255,
            text_alpha_gradient,
            false,
        );
    }
}
