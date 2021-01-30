use crate::utils::math::*;

pub const SFX_VOLUME: f32 = 0.05;
pub const BGM_VOLUME: f32 = 0.1;
pub const CHANNEL_COUNT: u32 = 4;
pub const CH_BG_MUSIC: u32 = 0;
pub const CH_SHOT: u32 = 1;
pub const CH_KILL: u32 = 2;
pub const CH_ITEM: u32 = 3;

pub const WINDOW_WIDTH: i32 = 700;
pub const WINDOW_HEIGHT: i32 = 256 * 2;
pub const ANGLE_DIV: i32 = 24;

pub const GAME_WIDTH: i32 = 512;
pub const GAME_HEIGHT: i32 = 256 * 2;

pub const X_COUNT: usize = 10;
pub const Y_COUNT: usize = 6;

pub const CENTER_X: i32 = GAME_WIDTH * ONE / 2;
pub const PLAYER_Y: i32 = (GAME_HEIGHT - 96) * ONE;

pub const APP_NAME: &str = "Teki";
pub const PLAYER_SPEED: i32 = 7 * ONE;
pub const MYSHOT_SPEED: i32 = 12 * ONE;
pub const ITEM_SPEED: i32 = 3 * ONE;
pub const SHOT_DELAY: u32 = 4;
pub const FPS: u32 = 60;
pub const MIN_FPS: u32 = 15;

pub const SCROLLING_BG_VEL: i32 = 3;

pub const BG_ALPHA: u8 = 190;
pub const RECT_STAGE_ALPHA: u8 = 120;
pub const TEXT_STAGE_ALPHA: u8 = 255;

pub const BASE_Y: i32 = 75;

pub const BG1_TEXTURE: &str = "bg_ground";
pub const BG3_TEXTURE: &str = "bg_grass";
pub const BG2_TEXTURE: &str = "bg_water";

pub const BG_MUSIC: &str = "stage01";
pub const TITLE_MUSIC: &str = "menu";

pub const SE_KILL: &str = "assets/sfx/se_kill";
pub const SE_SHOT: &str = "assets/sfx/se_graze";
pub const SE_OK: &str = "assets/sfx/se_ok00";
pub const SE_SELECT: &str = "assets/sfx/se_select00";
pub const SE_ITEM: &str = "assets/sfx/se_item00";

pub const RE_FONT: &str = "assets/fonts/regular.ttf";
pub const IM_FONT: &str = "assets/fonts/immortal.ttf";
