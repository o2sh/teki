use crate::teki::sdl::sdl_audio::SdlAudio;
use crate::teki::utils::consts::*;
use array_macro::*;
use lazy_static::lazy_static;
use sdl2::rect::Point;

pub struct SoundQueue {
    queue: Vec<(u32, &'static str)>,
}

impl SoundQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn flush(&mut self, audio: &mut SdlAudio) {
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
    xtbl: [i32; X_COUNT],
    ytbl: [i32; Y_COUNT],
    to_left: bool,
    pub done_appearance: bool,
    moving_count: u32,
}

impl Default for EnemyFormation {
    fn default() -> Self {
        let mut formation = Self {
            xtbl: Default::default(),
            ytbl: Default::default(),
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
            self.xtbl[j] = BASE_X_TABLE[j];
        }
        for i in 0..Y_COUNT {
            self.ytbl[i] = BASE_Y_TABLE[i];
        }
    }

    pub fn update(&mut self) {
        let dx = 3;
        let dx = if self.to_left { -dx } else { dx };

        for i in 0..X_COUNT {
            self.xtbl[i] += dx;
        }

        if self.xtbl[0] - 20 < 15 || self.xtbl[X_COUNT - 1] + 10 > GAME_WIDTH {
            self.to_left = !self.to_left;
            self.moving_count = 0
        } else {
            self.moving_count += 1;
        }
    }

    pub fn pos(&self, index: &u8) -> Point {
        Point::new(self.xtbl[*index as usize], self.ytbl[*index as usize])
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
