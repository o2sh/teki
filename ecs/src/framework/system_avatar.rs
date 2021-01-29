use crate::framework::components::*;
use legion::world::SubWorld;
use legion::*;
use teki_common::utils::pad::{Pad, PadBit};

#[system(for_each)]
#[write_component(Player)]
pub fn animate_avatar(
    _: &mut Avatar,
    sprite: &mut SpriteDrawable,
    #[resource] pad: &Pad,
    world: &mut SubWorld,
) {
    let player = <&mut Player>::query().iter_mut(world).next();
    if let Some(p) = player {
        if pad.is_pressed(PadBit::Z) {
            sprite.sprite_name = p.data.attack_face;
        } else {
            sprite.sprite_name = p.data.neutral_face;
        }
    }
}
