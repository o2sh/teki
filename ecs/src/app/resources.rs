use array_macro::*;
use lazy_static::lazy_static;
use teki_common::traits::Audio;
use teki_common::traits::Renderer;
use teki_common::utils::consts::*;
use vector2d::Vector2D;

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
    moving_count: u32,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        let mut formation = Self {
            x_indices: Default::default(),
            y_indices: Default::default(),
            done_appearance: false,
            moving_count: 0,
            to_left: false,
        };
        formation.init();
        formation
    }
}

impl EnemyFormation {
    pub fn init(&mut self) {
        *self = Self { moving_count: 0, done_appearance: false, ..*self };

        for j in 0..X_COUNT {
            self.x_indices[j] = BASE_X_TABLE[j];
        }
        for i in 0..Y_COUNT {
            self.y_indices[i] = BASE_Y_TABLE[i];
        }
    }

    pub fn update(&mut self) {
        let dx = 3;
        let dx = if self.to_left { -dx } else { dx };

        for i in 0..X_COUNT {
            self.x_indices[i] += dx;
        }

        if self.x_indices[0] < 35 || self.x_indices[X_COUNT - 1] > GAME_WIDTH - 10 {
            self.to_left = !self.to_left;
            self.moving_count = 0
        } else {
            self.moving_count += 1;
        }
    }

    pub fn pos(&self, index: &u8) -> Vector2D<i32> {
        Vector2D::new(self.x_indices[*index as usize], self.y_indices[*index as usize])
    }
}

lazy_static! {
    pub static ref BASE_X_TABLE: [i32; X_COUNT] = {
        let cx = GAME_WIDTH / 2;
        let w = 40;

        array![|j|
            cx - ((X_COUNT - 1) as i32) * w / 2 + (j as i32) * w
        ; X_COUNT]
    };
    pub static ref BASE_Y_TABLE: [i32; Y_COUNT] = {
        array![
            BASE_Y
        ; Y_COUNT]
    };
}

#[derive(Default)]
pub struct GameInfo {
    pub score: u32,
}

impl GameInfo {
    pub fn add_score(&mut self, add: u32) {
        self.score += add;
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_str(FONTS, GAME_WIDTH + 25, 35, "SCORE", 255, 0, 0);
        renderer.draw_str(
            FONTS,
            GAME_WIDTH + 25 + 8 * 6,
            35,
            &format!("{}", self.score),
            255,
            255,
            255,
        );
    }
}
