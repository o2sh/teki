use crate::framework::components::EneShot;
use crate::framework::components::*;
use crate::framework::system_player::{enable_player_shot, enum_player_target_pos};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
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
    pub game_state: GameState,
    pub frame_count: u32,
}

impl GameInfo {
    pub fn new() -> Self {
        GameInfo { stage: 0, score: 0, game_state: GameState::StartStage, frame_count: 0 }
    }

    pub fn add_score(&mut self, add: u32) {
        self.score += add;
    }

    pub fn update(&mut self, stage_indicator: &mut StageIndicator, world: &mut SubWorld) {
        self.frame_count = self.frame_count.wrapping_add(1);

        match self.game_state {
            GameState::StartStage => {
                if self.frame_count == 1 {
                    stage_indicator.set_stage();
                }
                if self.frame_count >= 180 as u32 {
                    self.game_state = GameState::Playing;
                }

                if self.frame_count >= 30 as u32 {
                    for player in <&mut Player>::query().iter_mut(world) {
                        enable_player_shot(player, true);
                    }
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

#[derive(Default)]
pub struct EneShotSpawner {
    queue: Vec<Vector2D<i32>>,
    shot_paused_count: u32,
}

impl EneShotSpawner {
    pub fn push(&mut self, pos: &Vector2D<i32>) {
        self.queue.push(pos.clone());
    }

    pub fn update(&mut self, game_info: &GameInfo, world: &SubWorld, commands: &mut CommandBuffer) {
        if self.shot_paused_count > 0 {
            self.shot_paused_count -= 1;
        } else {
            self.process_queue(game_info, world, commands);
        }
        self.queue.clear();
    }

    pub fn pause_enemy_shot(&mut self, wait: u32) {
        self.shot_paused_count = wait;
    }

    pub fn restart(&mut self) {
        self.shot_paused_count = 0;
        self.queue.clear();
    }

    fn process_queue(
        &mut self,
        game_info: &GameInfo,
        world: &SubWorld,
        commands: &mut CommandBuffer,
    ) {
        let shot_count = <&EneShot>::query().iter(world).count();
        let target = enum_player_target_pos(world);
        for (pos, _i) in self.queue.iter().zip(shot_count..MAX_ENE_SHOT_COUNT) {
            let d = &target - pos;
            let angle = atan2_lut(d.y, -d.x);
            let limit = ANGLE * ONE * 30 / 360;
            let angle = clamp(angle, -limit, limit);
            let vel = calc_velocity(angle + ANGLE * ONE / 2, calc_ene_shot_speed(game_info.stage));
            commands.push((
                EneShot(vel),
                Posture(*pos, 0, 0),
                HitBox { size: Vector2D::new(16, 16) },
                SpriteDrawable {
                    sprite_name: "orb_green_full",
                    offset: Vector2D::new(-8, -8),
                    alpha: 255,
                },
            ));
        }
    }
}

fn calc_ene_shot_speed(stage: u16) -> i32 {
    const MAX_STAGE: i32 = 64;
    let per = std::cmp::min(stage as i32, MAX_STAGE) * ONE / MAX_STAGE;
    (ENE_SHOT_SPEED2 - ENE_SHOT_SPEED1) * per / ONE + ENE_SHOT_SPEED1
}
