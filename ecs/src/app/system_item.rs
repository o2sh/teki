use crate::app::components::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::utils::{consts::*, math::*};
use vector2d::Vector2D;

pub const ITEM_SPRITES: [&str; 2] = ["item0", "item1"];

pub fn spawn_item(pos: &Vector2D<i32>, commands: &mut CommandBuffer) {
    let drawable = SpriteDrawable { sprite_name: ITEM_SPRITES[0], offset: Vector2D::new(-6, -6) };
    let hit_box = HitBox { size: Vector2D::new(12, 12) };
    commands.push((
        Item { rel_pos: Vector2D::new(0, 0) },
        Posture(pos.clone(), 0),
        hit_box,
        drawable,
    ));
}

#[system(for_each)]
#[write_component(Posture)]
#[write_component(Item)]
pub fn move_item(
    item: &mut Item,
    entity: &Entity,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    const DANGLE: i32 = ANGLE * ONE / ANGLE_DIV;
    let pos = <&mut Posture>::query().get_mut(world, *entity).unwrap();
    pos.0.y += ITEM_SPEED;
    item.rel_pos.y += ITEM_SPEED;
    pos.1 += DANGLE;

    if pos.0.y > WINDOW_HEIGHT * ONE {
        commands.remove(*entity);
    }
}
