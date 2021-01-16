use crate::utils::math::*;

pub const BASE_VOLUME: f32 = 1.0 / 4.0;
pub const CHANNEL_COUNT: u32 = 3;
pub const CH_SHOT: u32 = 0;
pub const CH_BG_MUSIC: u32 = 1;
pub const CH_KILL: u32 = 2;

pub const PADDING: i32 = 16;

pub const WINDOW_WIDTH: i32 = 320 * 3;
pub const WINDOW_HEIGHT: i32 = 256 * 3;

pub const GAME_WIDTH: i32 = 192 * 3;
pub const GAME_HEIGHT: i32 = WINDOW_HEIGHT - (PADDING * 2);

pub const CENTER_X: i32 = GAME_WIDTH * ONE / 2;
pub const PLAYER_Y: i32 = (GAME_HEIGHT - 16 - 8) * ONE;

pub const APP_NAME: &str = "Teki";
pub const PLAYER_SPEED: i32 = 8 * ONE / 2;
pub const MYSHOT_SPEED: i32 = 12 * ONE;
pub const FPS: u32 = 60;

pub const X_COUNT: usize = 10;
pub const Y_COUNT: usize = 6;

pub const BASE_Y: i32 = 50;

pub const FONTS: &str = "font";

pub const GAME_TEXTURE: &str = "ground";
pub const BG_TEXTURE: &str = "grass";

pub const PLAYER_SPRITE: &str = "hero";
pub const ENEMY_SPRITE: &str = "enemy";
pub const BULLET_SPRITE: &str = "orb";

pub const BUBBLE_SOUND: &str = "./assets/audio/bubble";
pub const BG_MUSIC: &str = "./assets/audio/loop";
pub const SE_KILL: &str = "./assets/audio/pop";
