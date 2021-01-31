use crate::framework::components::*;
use crate::framework::resources::{GameInfo, SoundQueue, StageIndicator};
use crate::framework::system_effect::create_enemy_explosion_effect;
use crate::framework::system_item::spawn_item;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::game::PlayerData;
use teki_common::utils::{
    collision::CollBox,
    consts::*,
    math::*,
    pad::{Pad, PadBit},
};
use vector2d::Vector2D;

pub const ANIMATION_SPAN: u32 = 5;

pub fn new_player(character_index: u8) -> Player {
    let data = PlayerData::new(character_index);
    Player { shot_enable: false, next_shoot_time: 0, index_x: 0, index_y: 0, data }
}

pub fn enable_player_shot(player: &mut Player, enable: bool) {
    player.shot_enable = enable;
}

pub fn player_hit_box() -> HitBox {
    HitBox { size: Vector2D::new(40, 40) }
}

pub fn player_sprite(player: &Player) -> SpriteDrawable {
    SpriteDrawable { sprite_name: player.data.sprite, offset: Vector2D::new(-16, -24) }
}

#[system]
#[write_component(Player)]
pub fn update_game_info(
    #[resource] game_info: &mut GameInfo,
    #[resource] stage_indicator: &mut StageIndicator,
    world: &mut SubWorld,
) {
    game_info.update(stage_indicator, world);
}

#[system(for_each)]
#[write_component(Posture)]
pub fn move_player(
    player: &mut Player,
    entity: &Entity,
    #[resource] pad: &Pad,
    world: &mut SubWorld,
) {
    do_move_player(player, pad, *entity, world);
}

pub fn do_move_player(player: &mut Player, pad: &Pad, entity: Entity, world: &mut SubWorld) {
    let position = <&mut Posture>::query().get_mut(world, entity).unwrap();
    let pos = &mut position.0;
    if pad.is_pressed(PadBit::L) {
        pos.x -= PLAYER_SPEED;
        let left = 16 * ONE;
        if pos.x < left {
            pos.x = left;
        }

        player.index_y = 1;
        player.index_x = 7;
    }
    if pad.is_pressed(PadBit::R) {
        pos.x += PLAYER_SPEED;
        let right = (GAME_WIDTH - 16) * ONE;
        if pos.x > right {
            pos.x = right;
        }

        player.index_y = 2;
        player.index_x = 7;
    }

    if pad.is_pressed(PadBit::U) {
        pos.y -= PLAYER_SPEED;
        let top = 22 * ONE;
        if pos.y < top {
            pos.y = top;
        }
    }
    if pad.is_pressed(PadBit::D) {
        pos.y += PLAYER_SPEED;
        let bottom = (GAME_HEIGHT - 22) * ONE;
        if pos.y > bottom {
            pos.y = bottom;
        }
    }

    if pad.is_pressed(PadBit::L) && pad.is_pressed(PadBit::R)
        || !pad.is_pressed(PadBit::L) && !pad.is_pressed(PadBit::R)
    {
        player.index_y = 0;
    }
}

#[system(for_each)]
#[read_component(MyShot)]
pub fn fire_myshot(
    player: &mut Player,
    position: &Posture,
    entity: &Entity,
    #[resource] pad: &Pad,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &mut GameInfo,
    commands: &mut CommandBuffer,
) {
    if pad.is_pressed(PadBit::Z) && can_player_fire(player) {
        if player.next_shoot_time < game_info.frame_count {
            sound_queue.push_play(CH_SHOT, SE_SHOT);
            do_fire_myshot(player, position, *entity, commands);
            player.next_shoot_time = game_info.frame_count + SHOT_DELAY;
        }
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
    position: &Posture,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    let pos = Posture(Vector2D::new(position.0.x, position.0.y - 16 * ONE), 0, 0);
    commands.push((
        MyShot { player_entity: entity },
        pos,
        HitBox { size: Vector2D::new(10, 20) },
        SpriteDrawable { sprite_name: player.data.bullet, offset: Vector2D::new(-8, -32) },
    ));
}

#[system(for_each)]
#[write_component(Posture)]
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
    for e in [Some(entity)].iter().flatten() {
        let position = <&mut Posture>::query().get_mut(world, *e).unwrap();
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
    const MARGIN: i32 = 10;
    const TOP: i32 = (MARGIN) * ONE;
    pos.y < TOP
}

pub fn delete_entity(entity: Entity, commands: &mut CommandBuffer) {
    commands.remove(entity);
}

#[system]
#[read_component(MyShot)]
#[read_component(Posture)]
#[read_component(HitBox)]
#[write_component(Enemy)]
pub fn shot_collision_check(
    world: &mut SubWorld,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &GameInfo,
    commands: &mut CommandBuffer,
) {
    for (_, shot_pos, shot_hit_box, shot_entity) in
        <(&MyShot, &Posture, &HitBox, Entity)>::query().iter(world)
    {
        let shot_coll_box = pos_to_coll_box(&shot_pos.0, &shot_hit_box);

        for (_enemy, enemy_pos, enemy_hit_box, enemy_entity) in
            <(&Enemy, &Posture, &HitBox, Entity)>::query().iter(world)
        {
            let enemy_collbox =
                CollBox { top_left: round_vec(&enemy_pos.0), size: enemy_hit_box.size };
            if shot_coll_box.check_collision(&enemy_collbox) {
                delete_entity(*shot_entity, commands);
                delete_entity(*enemy_entity, commands);
                create_enemy_explosion_effect(&enemy_pos.0, 1, commands);
                sound_queue.push_play(CH_KILL, SE_KILL);
                spawn_item(&enemy_pos.0, game_info.frame_count, commands);
            }
        }
    }
}

pub fn pos_to_coll_box(pos: &Vector2D<i32>, coll_rect: &HitBox) -> CollBox {
    CollBox { top_left: round_vec(pos), size: coll_rect.size }
}

#[system(for_each)]
pub fn animate_player(
    player: &mut Player,
    sprite: &mut SpriteDrawable,
    #[resource] game_info: &mut GameInfo,
) {
    do_animate_player(player, sprite, game_info.frame_count);
}

pub fn do_animate_player(player: &mut Player, sprite: &mut SpriteDrawable, frame_count: u32) {
    if frame_count % ANIMATION_SPAN == 0 {
        player.index_x += 1;
        if player.index_x > 7 && player.index_y == 0 {
            player.index_x = 0;
        } else if player.index_x > 7 && player.index_y > 0 {
            player.index_x = 4;
        }

        sprite.sprite_name = player.data.animation_table[player.index_y][player.index_x];
    }
}

pub fn enum_player_target_pos(world: &SubWorld) -> Vector2D<i32> {
    let (_, posture) = <(&Player, &Posture)>::query().iter(world).next().unwrap();
    posture.0.clone()
}
