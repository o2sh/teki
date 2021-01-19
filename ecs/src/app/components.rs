use legion::*;
use teki_common::{EnemyType, FormationIndex};
use vector2d::Vector2D;

#[derive(Clone, Copy)]
pub struct Position(pub Vector2D<i32>);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub offset: Vector2D<i32>,
}

pub struct MyShot {
    pub player_entity: Entity,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub formation_index: FormationIndex,
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
