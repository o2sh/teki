use crate::components::*;
use crate::consts::*;
use crate::pad::{Pad, PadBit};
use legion::world::SubWorld;
use legion::*;
use sdl2::rect::Point;

const SPRITE_NAME: &str = "assets/neko.png";

pub struct Player {
    pub shot_enable: bool,
}

pub fn new_player() -> Player {
    Player { shot_enable: true }
}

pub fn player_sprite() -> SpriteDrawable {
    SpriteDrawable { sprite_name: SPRITE_NAME, offset: Point::new(-8, -8) }
}

#[system(for_each)]
#[write_component(Position)]
pub fn move_player(entity: &Entity, #[resource] pad: &Pad, world: &mut SubWorld) {
    do_move_player(pad, *entity, world);
}

pub fn do_move_player(pad: &Pad, entity: Entity, world: &mut SubWorld) {
    let position = <&mut Position>::query().get_mut(world, entity).unwrap();
    let pos = &mut position.0;
    if pad.is_pressed(PadBit::L) {
        pos.x -= PLAYER_SPEED;
    }
    if pad.is_pressed(PadBit::R) {
        pos.x += PLAYER_SPEED;
    }

    if pad.is_pressed(PadBit::U) {
        pos.y -= PLAYER_SPEED;
    }
    if pad.is_pressed(PadBit::D) {
        pos.y += PLAYER_SPEED;
    }
}
