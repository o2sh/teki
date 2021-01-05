use crate::teki::ecs::EnemyType;
use legion::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone, Copy)]
pub struct Position(pub Point);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub rect: Rect,
}

pub struct MyShot {
    pub player_entity: Entity,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub formation_index: u8,
}
