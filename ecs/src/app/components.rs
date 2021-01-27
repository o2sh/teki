use legion::*;
use teki_common::game::{EnemyType, FormationIndex, ItemType, Traj, RGBA};
use vector2d::Vector2D;

#[derive(Clone, Copy)]
pub struct Posture(pub Vector2D<i32>, pub i32);

pub struct Speed(pub i32, pub i32);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub offset: Vector2D<i32>,
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

pub struct EnemyBase {
    pub traj: Option<Traj>
}

#[derive(PartialEq)]
pub enum EnemyState {
    Appearance,
    MoveToFormation,
    Formation,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub formation_index: FormationIndex,
    pub state: EnemyState,
    pub base: EnemyBase,
    pub is_formation: bool,
}

pub struct Item {
    pub item_type: ItemType,
}

pub struct HitBox {
    pub size: Vector2D<i32>,
}

pub struct SequentialSpriteAnime {
    pub sprites: &'static [&'static str],
    pub frame_wait: u32,
    pub delay: u32,
    pub offset: Vector2D<i32>,
    pub count: u32,
}
