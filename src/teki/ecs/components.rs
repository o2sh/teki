use legion::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone)]
pub struct Position(pub Point);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub rect: Rect,
}

pub struct MyShot {
    pub player_entity: Entity,
}
