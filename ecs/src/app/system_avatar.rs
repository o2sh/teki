use crate::app::components::*;
use legion::*;
use teki_common::utils::consts::*;
use teki_common::utils::pad::{Pad, PadBit};
use vector2d::Vector2D;

const AVATAR_SPRITES: [&str; 2] = ["a_reimu0", "a_reimu1"];

pub struct Avatar;

pub fn avatar_sprite() -> SpriteDrawable {
    SpriteDrawable { sprite_name: AVATAR_SPRITE, offset: Vector2D::new(-50, -50) }
}

#[system(for_each)]
pub fn animate_avatar(_: &mut Avatar, sprite: &mut SpriteDrawable, #[resource] pad: &Pad) {
    if pad.is_pressed(PadBit::Z) {
        sprite.sprite_name = AVATAR_SPRITES[1]
    } else {
        sprite.sprite_name = AVATAR_SPRITES[0]
    }
}
