use crate::app::components::*;
use crate::app::resources::{GameInfo, SoundQueue};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::utils::collision::CollBox;
use teki_common::utils::consts::*;
use teki_common::utils::pad::{Pad, PadBit};
use vector2d::Vector2D;

pub struct Player {
    pub shot_enable: bool,
}

pub fn new_player() -> Player {
    Player { shot_enable: true }
}

pub fn player_hit_box() -> HitBox {
    HitBox { size: Vector2D::new(40, 40) }
}

pub fn player_sprite() -> SpriteDrawable {
    SpriteDrawable { sprite_name: NEKO_SPRITE }
}

#[system(for_each)]
#[write_component(Position)]
pub fn move_player(_: &mut Player, entity: &Entity, #[resource] pad: &Pad, world: &mut SubWorld) {
    do_move_player(pad, *entity, world);
}

pub fn do_move_player(pad: &Pad, entity: Entity, world: &mut SubWorld) {
    let position = <&mut Position>::query().get_mut(world, entity).unwrap();
    let pos = &mut position.0;
    if pad.is_pressed(PadBit::L) {
        pos.x -= PLAYER_SPEED;
        let left = PADDING + 15;
        if pos.x < left {
            pos.x = left;
        }
    }
    if pad.is_pressed(PadBit::R) {
        pos.x += PLAYER_SPEED;
        let right = GAME_WIDTH - 10;
        if pos.x > right {
            pos.x = right;
        }
    }

    if pad.is_pressed(PadBit::U) {
        pos.y -= PLAYER_SPEED;
        let top = PADDING + 20;
        if pos.y < top {
            pos.y = top;
        }
    }
    if pad.is_pressed(PadBit::D) {
        pos.y += PLAYER_SPEED;
        let bottom = GAME_HEIGHT - 10;
        if pos.y > bottom {
            pos.y = bottom;
        }
    }
}

#[system(for_each)]
#[read_component(MyShot)]
pub fn fire_myshot(
    player: &Player,
    position: &Position,
    entity: &Entity,
    #[resource] pad: &Pad,
    #[resource] sound_queue: &mut SoundQueue,
    commands: &mut CommandBuffer,
) {
    if pad.is_trigger(PadBit::A) {
        sound_queue.push_play(CH_SHOT, BUBBLE_SOUND);
        do_fire_myshot(player, position, *entity, commands)
    }
}

pub fn can_player_fire(player: &Player) -> bool {
    if !player.shot_enable {
        return false;
    }
    true
}
pub fn do_fire_myshot(
    player: &Player,
    position: &Position,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    if can_player_fire(player) {
        let pos = Position(Vector2D::new(position.0.x, position.0.y - 20));
        commands.push((
            MyShot { player_entity: entity },
            pos,
            HitBox { size: Vector2D::new(20, 20) },
            SpriteDrawable { sprite_name: HEART_SPRITE },
        ));
    }
}

#[system(for_each)]
#[write_component(Position)]
pub fn move_myshot(
    _: &MyShot,
    entity: &Entity,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    do_move_myshot(*entity, world, commands);
}

pub fn do_move_myshot(entity: Entity, world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut cont = false;
    for e in [Some(entity)].iter().flat_map(|x| x) {
        let position = <&mut Position>::query().get_mut(world, *e).unwrap();
        let pos = &mut position.0;

        pos.y -= MYSHOT_SPEED;
        if !out_of_screen(pos) {
            cont = true;
        }
    }
    if !cont {
        delete_entity(entity, commands);
    }
}

fn out_of_screen(pos: &Vector2D<i32>) -> bool {
    const MARGIN: i32 = 5;
    const TOP: i32 = MARGIN + PADDING;
    const LEFT: i32 = MARGIN + PADDING;
    const RIGHT: i32 = GAME_WIDTH;
    const BOTTOM: i32 = GAME_HEIGHT - PADDING;
    pos.y < TOP || pos.x < LEFT || pos.x > RIGHT || pos.y > BOTTOM
}

pub fn delete_entity(entity: Entity, commands: &mut CommandBuffer) {
    commands.remove(entity);
}

#[system]
#[read_component(MyShot)]
#[read_component(Position)]
#[read_component(HitBox)]
#[write_component(Enemy)]
#[write_component(SpriteDrawable)]
pub fn collision_check(
    world: &mut SubWorld,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &mut GameInfo,
    commands: &mut CommandBuffer,
) {
    for (_, shot_pos, shot_hit_box, shot_entity) in
        <(&MyShot, &Position, &HitBox, Entity)>::query().iter(world)
    {
        let shot_coll_box = pos_to_coll_box(&shot_pos.0, &shot_hit_box);

        for (_enemy, enemy_pos, enemy_hit_box, enemy_entity) in
            <(&Enemy, &Position, &HitBox, Entity)>::query().iter(world)
        {
            let enemy_collbox = CollBox { top_left: enemy_pos.0, size: enemy_hit_box.size };
            if shot_coll_box.check_collision(&enemy_collbox) {
                delete_entity(*shot_entity, commands);
                delete_entity(*enemy_entity, commands);
                sound_queue.push_play(CH_KILL, SE_KILL);
                game_info.add_score(20);
            }
        }
    }
}

fn pos_to_coll_box(pos: &Vector2D<i32>, coll_rect: &HitBox) -> CollBox {
    CollBox { top_left: *pos, size: coll_rect.size }
}
