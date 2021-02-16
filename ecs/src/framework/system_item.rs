use crate::framework::{components::*, pos_to_coll_box, resources::*};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use rand::Rng;
use teki_common::game::{ItemType, RGBA};
use teki_common::utils::{collision::CollBox, consts::*, math::*};
use vector2d::Vector2D;

pub const ITEM_SPRITES: [&str; 2] = ["item0", "item1"];

pub fn spawn_item(pos: &Vector2D<i32>, frame_count: u32, commands: &mut CommandBuffer) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0, 2);
    let drawable =
        SpriteDrawable { sprite_name: ITEM_SPRITES[i], offset: Vector2D::new(-6, -6), alpha: 255 };
    let hit_box = HitBox { offset: Vector2D::new(-6, -6), size: Vector2D::new(12, 12) };

    let item_type = match i {
        0 => ItemType::Red,
        1 => ItemType::Blue,
        _ => {
            panic!("Illegal");
        }
    };
    commands.push((
        Item { item_type, birth_time: frame_count },
        Posture(pos.clone(), 0, 0),
        hit_box,
        drawable,
    ));
}

#[system(for_each)]
#[write_component(Posture)]
pub fn move_item(
    item: &mut Item,
    entity: &Entity,
    world: &mut SubWorld,
    #[resource] game_info: &GameInfo,
    commands: &mut CommandBuffer,
) {
    const DANGLE: i32 = ANGLE * ONE / ANGLE_DIV;
    let pos = <&mut Posture>::query().get_mut(world, *entity).unwrap();

    let t = (game_info.frame_count - item.birth_time) as i32;

    pos.0.y += (((t - 30) << 1) / 8) * ONE;

    if t < 30 {
        pos.1 += DANGLE;
    } else {
        let ang = ANGLE * ONE / 128;
        pos.1 -= clamp(pos.1, -ang, ang);
    }

    if pos.0.y > WINDOW_HEIGHT * ONE {
        commands.remove(*entity);
    }
}

#[system]
#[read_component(Item)]
#[read_component(Posture)]
#[read_component(HitBox)]
#[write_component(Player)]
pub fn item_player_collision_check(
    world: &mut SubWorld,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &mut GameInfo,
    commands: &mut CommandBuffer,
) {
    for (_, player_pos, player_hit_box) in <(&Player, &Posture, &HitBox)>::query().iter(world) {
        let player_coll_box = pos_to_coll_box(&player_pos.0, &player_hit_box);

        for (item, item_pos, item_hit_box, item_entity) in
            <(&Item, &Posture, &HitBox, Entity)>::query().iter(world)
        {
            let item_collbox = CollBox {
                top_left: round_vec(&item_pos.0) + item_hit_box.offset,
                size: item_hit_box.size,
            };
            if player_coll_box.check_collision(&item_collbox) {
                commands.remove(*item_entity);
                sound_queue.push_play(CH_ITEM, SE_ITEM);
                let (points, color) = match item.item_type {
                    ItemType::Red => (100, RGBA { r: 255, g: 215, b: 0, a: 255 }),
                    ItemType::Blue => (200, RGBA { r: 255, g: 255, b: 255, a: 255 }),
                };
                game_info.add_score(points);

                let text = Text {
                    msg: format!("+{}", points),
                    color,
                    offset: Vector2D::new(-32, -32),
                    delay: 12,
                };

                commands.push((text, player_pos.clone()));
            }
        }
    }
}
