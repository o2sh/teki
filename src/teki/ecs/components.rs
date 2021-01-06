use crate::teki::ecs::EnemyType;
use legion::*;
use sdl2::rect::Rect;
use vector2d::Vector2D;

#[derive(Clone, Copy)]
pub struct Position(pub Vector2D<i32>);

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

pub struct HitBox {
    pub size: Vector2D<i32>,
}
