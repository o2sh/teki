use crate::teki::ecs::components::*;
use crate::teki::utils::consts::*;
use crate::teki::utils::pad::{Pad, PadBit};
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use sdl2::rect::{Point, Rect};

const SPRITE_NAME: &str = "assets/neko.png";

pub struct Player {
    pub shot_enable: bool,
}

pub fn new_player() -> Player {
    Player { shot_enable: true }
}

pub fn player_sprite() -> SpriteDrawable {
    SpriteDrawable { sprite_name: SPRITE_NAME, rect: Rect::new(5, 5, 40, 40) }
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
    commands: &mut CommandBuffer,
) {
    if pad.is_trigger(PadBit::A) {
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
        let pos = Position(Point::new(position.0.x, position.0.y - 20));
        commands.push((
            MyShot { player_entity: entity },
            pos,
            SpriteDrawable { sprite_name: "assets/heart.png", rect: Rect::new(0, 0, 20, 20) },
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
        delete_myshot(entity, commands);
    }
}

fn out_of_screen(pos: &Point) -> bool {
    const MARGIN: i32 = 5;
    const TOP: i32 = MARGIN + PADDING;
    const LEFT: i32 = MARGIN + PADDING;
    const RIGHT: i32 = GAME_WIDTH;
    const BOTTOM: i32 = GAME_HEIGHT - PADDING;
    pos.y < TOP || pos.x < LEFT || pos.x > RIGHT || pos.y > BOTTOM
}

pub fn delete_myshot(entity: Entity, commands: &mut CommandBuffer) {
    commands.remove(entity);
}
