use legion::*;
use teki_common::game::{EnemyType, FormationIndex, ItemType, PlayerData, Traj, RGBA};
use vector2d::Vector2D;

#[derive(Clone, Copy)]
pub struct Posture(pub Vector2D<i32>, pub i32, pub i32);

pub struct Speed(pub i32, pub i32);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub offset: Vector2D<i32>,
    pub alpha: u8,
}

pub struct Text {
    pub msg: String,
    pub color: RGBA,
    pub offset: Vector2D<i32>,
    pub delay: u32,
}

pub struct MyShot {
    pub player_entity: Entity,
}

pub struct EneShot(pub Vector2D<i32>);

pub struct EnemyBase {
    pub traj: Option<Traj>,
    pub attack_frame_count: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AttackType {
    Normal,
    Intense,
}

#[derive(PartialEq)]
pub enum EnemyState {
    Appearance,
    MoveToFormation,
    Formation,
    Attack(AttackType),
    ExitScreen,
    Destroy,
}
pub struct Avatar;

#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Normal,
    Invincible,
}

pub struct Player {
    pub state: PlayerState,
    pub shot_enable: bool,
    pub next_shoot_time: u32,
    pub invincibility_starting_time: u32,
    pub index_x: usize,
    pub index_y: usize,
    pub data: PlayerData<'static>,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub formation_index: FormationIndex,
    pub index_x: usize,
    pub state: EnemyState,
    pub base: EnemyBase,
    pub is_formation: bool,
    pub life: u32,
}

pub struct Item {
    pub item_type: ItemType,
    pub birth_time: u32,
}

pub struct HitBox {
    pub offset: Vector2D<i32>,
    pub size: Vector2D<i32>,
}

pub struct SequentialSpriteAnime {
    pub sprites: &'static [&'static str],
    pub frame_wait: u32,
    pub delay: u32,
    pub offset: Vector2D<i32>,
    pub count: u32,
}
