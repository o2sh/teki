use array_macro::*;
use lazy_static::lazy_static;
use teki_common::traits::Audio;
use teki_common::traits::Renderer;
use teki_common::utils::consts::*;
use teki_common::utils::math::*;
use teki_common::FormationIndex;
use vector2d::Vector2D;

pub const MARGIN: i32 = 20;

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
    pub frame_count: u32,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        let mut formation = Self {
            x_indices: Default::default(),
            y_indices: Default::default(),
            done_appearance: false,
            to_left: false,
            frame_count: 0,
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

    pub fn update(&mut self, frame_count: u32) {
        if frame_count % 5 == 0 {
            self.frame_count = self.frame_count.wrapping_add(1);
        }
        let space = GAME_WIDTH - X_COUNT as i32 * 32;
        let dx = space * ONE / 256;

        let dx = if self.to_left { -dx } else { dx };

        for i in 0..X_COUNT {
            self.x_indices[i] += dx;
        }

        if self.x_indices[0] < (PADDING + MARGIN + 24) * ONE
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

#[derive(Default)]
pub struct GameInfo {
    pub score: u32,
    pub frame_count: u32,
}

impl GameInfo {
    pub fn add_score(&mut self, add: u32) {
        self.score += add;
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        let left_padding = 50;
        let top_padding = 80;
        renderer.draw_str(FONTS, GAME_WIDTH + left_padding, top_padding, "SCORE", 25, 158, 88);
        renderer.draw_str(
            FONTS,
            GAME_WIDTH + left_padding + 16 * 6,
            top_padding,
            &format!("{}", self.score),
            255,
            255,
            255,
        );
    }
}
