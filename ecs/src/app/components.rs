use legion::*;
use teki_common::EnemyType;
use vector2d::Vector2D;

#[derive(Clone, Copy)]
pub struct Position(pub Vector2D<i32>);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
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
