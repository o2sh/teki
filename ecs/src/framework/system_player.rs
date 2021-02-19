use crate::framework::components::*;
use crate::framework::pos_to_coll_box;
use crate::framework::resources::{GameInfo, SoundQueue, StageIndicator};
use crate::framework::system_effect::create_explosion_effect;
use crate::framework::system_enemy::set_enemy_damage;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::game::PlayerData;
use teki_common::game::RGBA;
use teki_common::utils::math::calc_velocity;
use teki_common::utils::{
    collision::CollBox,
    consts::*,
    math::*,
    pad::{Pad, PadBit},
};
use vector2d::Vector2D;

pub const ANIMATION_SPAN: u32 = 5;
pub const SPECIAL_SPAN: u32 = 50;

pub fn new_player(character_index: u8) -> Player {
    let data = PlayerData::new(character_index);
    Player {
        state: PlayerState::Normal,
        shot_enable: false,
        next_shoot_time: 0,
        invincibility_starting_time: 0,
        index_x: 0,
        index_y: 0,
        data,
    }
}

pub fn enable_player_shot(player: &mut Player, enable: bool) {
    player.shot_enable = enable;
}

pub fn player_hit_box() -> HitBox {
    HitBox { offset: Vector2D::new(-10, -10), size: Vector2D::new(20, 20) }
}

pub fn player_sprite(player: &Player) -> SpriteDrawable {
    SpriteDrawable { sprite_name: player.data.sprite, offset: Vector2D::new(-16, -24), alpha: 255 }
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
        MyShot { player_entity: entity, size: 64, shot_type: ShotType::Normal },
        pos,
        HitBox { offset: Vector2D::new(-8, -32), size: Vector2D::new(16, 16) },
        SpriteDrawable {
            sprite_name: player.data.bullet,
            offset: Vector2D::new(-8, -32),
            alpha: 255,
        },
    ));
}

#[system(for_each)]
#[write_component(Posture)]
pub fn move_myshot(
    my_shot: &MyShot,
    entity: &Entity,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    do_move_myshot(my_shot, *entity, world, commands);
}

pub fn do_move_myshot(
    my_shot: &MyShot,
    entity: Entity,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut cont = false;
    for e in [Some(entity)].iter().flatten() {
        let posture = <&mut Posture>::query().get_mut(world, *e).unwrap();
        let pos = &mut posture.0;
        let angle = &posture.1;

        if *angle == 0 {
            pos.y -= MYSHOT_SPEED;
        } else {
            *pos += calc_velocity(*angle, MYSHOT_SPEED);
        }

        if !out_of_screen(pos, my_shot.size as i32) {
            cont = true;
        }
    }
    if !cont {
        commands.remove(entity);
    }
}

fn out_of_screen(pos: &Vector2D<i32>, size: i32) -> bool {
    let top = -size * ONE;
    let left = -size * ONE;
    let right = (GAME_WIDTH - size / 2) * ONE;
    let bottom = (GAME_HEIGHT + size) * ONE;
    pos.y < top || pos.x < left || pos.x > right || pos.y > bottom
}

#[system]
#[read_component(MyShot)]
#[read_component(Posture)]
#[read_component(HitBox)]
#[write_component(Enemy)]
pub fn player_shot_collision_check(
    world: &mut SubWorld,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &GameInfo,
    commands: &mut CommandBuffer,
) {
    let mut colls: Vec<(Entity, Vector2D<i32>, u32)> = Vec::new();
    for (shot, shot_pos, shot_hit_box, shot_entity) in
        <(&MyShot, &Posture, &HitBox, Entity)>::query().iter(world)
    {
        let shot_coll_box = pos_to_coll_box(&shot_pos.0, &shot_hit_box);

        for (_enemy, enemy_pos, enemy_hit_box, enemy_entity) in
            <(&Enemy, &Posture, &HitBox, Entity)>::query().iter(world)
        {
            let enemy_collbox = CollBox {
                top_left: round_vec(&enemy_pos.0) + enemy_hit_box.offset,
                size: enemy_hit_box.size,
            };
            if shot_coll_box.check_collision(&enemy_collbox) {
                commands.remove(*shot_entity);
                let damage = match shot.shot_type {
                    ShotType::Normal => 1,
                    ShotType::Special => 10,
                };
                colls.push((*enemy_entity, enemy_pos.0, damage));
            }
        }
    }

    for (enemy_entity, enemy_position, damage) in colls {
        let enemy = <&mut Enemy>::query().get_mut(world, enemy_entity).unwrap();
        set_enemy_damage(
            damage,
            enemy,
            enemy_entity,
            &enemy_position,
            sound_queue,
            game_info,
            commands,
        );
    }
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

#[system]
#[read_component(Enemy)]
#[read_component(HitBox)]
#[write_component(Posture)]
#[write_component(Player)]
#[write_component(SpriteDrawable)]
pub fn player_enemy_collision_check(
    world: &mut SubWorld,
    #[resource] sound_queue: &mut SoundQueue,
    #[resource] game_info: &mut GameInfo,
    commands: &mut CommandBuffer,
) {
    let mut colls: Vec<Entity> = Vec::new();

    let (player, player_pos, player_hit_box, player_entity) =
        <(&Player, &Posture, &HitBox, Entity)>::query().iter(world).next().unwrap();

    if player.state == PlayerState::Invincible {
        return;
    }
    let player_collbox = pos_to_coll_box(&player_pos.0, player_hit_box);
    for (_enemy, enemy_pos, enemy_hit_box) in <(&Enemy, &Posture, &HitBox)>::query().iter(world) {
        let enemy_collbox = CollBox {
            top_left: round_vec(&enemy_pos.0) + enemy_hit_box.offset,
            size: enemy_hit_box.size,
        };
        if player_collbox.check_collision(&enemy_collbox) {
            colls.push(*player_entity);
            break;
        }
    }

    for player_entity in colls {
        let (player, player_posture, player_sprite) =
            <(&mut Player, &mut Posture, &mut SpriteDrawable)>::query()
                .get_mut(world, player_entity)
                .unwrap();
        set_damage_to_player(
            player,
            player_posture,
            player_sprite,
            commands,
            sound_queue,
            game_info.frame_count,
        );
    }
}

pub fn set_damage_to_player(
    player: &mut Player,
    player_posture: &mut Posture,
    player_sprite: &mut SpriteDrawable,
    commands: &mut CommandBuffer,
    sound_queue: &mut SoundQueue,
    frame_count: u32,
) {
    create_explosion_effect(&player_posture.0, 1, commands);
    sound_queue.push_play(CH_DAMAGE, SE_DAMAGE);
    player_sprite.alpha = 175;
    player_posture.0 = Vector2D::new(CENTER_X, PLAYER_Y);
    player.state = PlayerState::Invincible;
    player.invincibility_starting_time = frame_count;
}

#[system(for_each)]
pub fn player_invincibility_frames(
    player: &mut Player,
    sprite: &mut SpriteDrawable,
    #[resource] game_info: &mut GameInfo,
) {
    if player.state == PlayerState::Invincible
        && game_info.frame_count > player.invincibility_starting_time + INVINCIBILITY_FRAMES
    {
        sprite.alpha = 255;
        player.state = PlayerState::Normal;
    }
}

#[system(for_each)]
pub fn special_attack(
    player: &mut Player,
    player_pos: &Posture,
    entity: &Entity,
    commands: &mut CommandBuffer,
    #[resource] pad: &Pad,
    #[resource] game_info: &mut GameInfo,
    #[resource] sound_queue: &mut SoundQueue,
) {
    if pad.is_trigger(PadBit::X) {
        sound_queue.push_play(CH_SPELL, SE_SPELL);
        game_info.alpha = 100;
        let text = Text {
            msg: String::from("Magic Sign \"Stellar Vortex\""),
            color: RGBA { r: 255, g: 255, b: 255, a: 255 },
            offset: Vector2D::new(0, 0),
            delay: 30,
            size: 16,
            font: IM_FONT,
        };

        let text_pos = Posture(Vector2D::new(50 * ONE, (GAME_HEIGHT - 75) * ONE), 0, 0);
        commands.push((text, text_pos));

        let sprite_pos = Posture(Vector2D::new((GAME_WIDTH - 330) * ONE, 50 * ONE), 0, 0);

        commands.push((
            SpecialAttack(game_info.frame_count + SPECIAL_SPAN),
            sprite_pos,
            SpriteDrawable {
                sprite_name: player.data.special,
                offset: Vector2D::new(0, 0),
                alpha: 140,
            },
        ));

        const DANGLE: i32 = ANGLE * ONE / ANGLE_DIV;

        for i in 0..8 {
            let angle = DANGLE * i * 9;
            let pos = Posture(Vector2D::new(player_pos.0.x, player_pos.0.y - 32 * ONE), angle, 0);
            commands.push((
                MyShot { player_entity: *entity, size: 64, shot_type: ShotType::Special },
                pos,
                HitBox { offset: Vector2D::new(-32, -32), size: Vector2D::new(64, 64) },
                SpriteDrawable {
                    sprite_name: player.data.big_orb,
                    offset: Vector2D::new(-32, -32),
                    alpha: 255,
                },
            ));
        }
    }
}

#[system(for_each)]
#[write_component(Posture)]
#[write_component(SpriteDrawable)]
pub fn move_special_attack(
    special_attack: &mut SpecialAttack,
    entity: &Entity,
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] game_info: &mut GameInfo,
) {
    let (position, sprite) =
        <(&mut Posture, &mut SpriteDrawable)>::query().get_mut(world, *entity).unwrap();
    let pos = &mut position.0;

    if pos.x > 10 * ONE {
        pos.x -= (1 << 3) * ONE;
    }

    if game_info.frame_count > special_attack.0 {
        commands.remove(*entity);
        game_info.alpha = BG_ALPHA;
        return;
    };

    let sp = (SPECIAL_SPAN >> 1) as f32;
    if ((special_attack.0 - game_info.frame_count) as f32) < sp {
        let ratio = (sp - (special_attack.0 - game_info.frame_count) as f32) / (4.0 * sp);
        sprite.alpha = (sprite.alpha as f32 * (1.0 - ratio)) as u8;
    }
}
