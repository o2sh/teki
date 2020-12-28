use crate::components::*;
use crate::consts::*;
use crate::pad::{Pad, PadBit};
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
pub fn move_player(entity: &Entity, #[resource] pad: &Pad, world: &mut SubWorld) {
    do_move_player(pad, *entity, world);
}

pub fn do_move_player(pad: &Pad, entity: Entity, world: &mut SubWorld) {
    let position = <&mut Position>::query().get_mut(world, entity).unwrap();
    let pos = &mut position.0;
    if pad.is_pressed(PadBit::L) {
        if pos.x > 40 {
            pos.x -= PLAYER_SPEED;
        }
    }
    if pad.is_pressed(PadBit::R) {
        if pos.x < WIDTH - 40 {
            pos.x += PLAYER_SPEED;
        }
    }

    if pad.is_pressed(PadBit::U) {
        if pos.y > 40 {
            pos.y -= PLAYER_SPEED;
        }
    }
    if pad.is_pressed(PadBit::D) {
        if pos.y < HEIGHT - 40 {
            pos.y += PLAYER_SPEED;
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
    const MARGIN: i32 = 4;
    const TOP: i32 = -MARGIN;
    const LEFT: i32 = -MARGIN;
    const RIGHT: i32 = WIDTH + MARGIN;
    const BOTTOM: i32 = HEIGHT + MARGIN;
    pos.y < TOP || pos.x < LEFT || pos.x > RIGHT || pos.y > BOTTOM
}

pub fn delete_myshot(entity: Entity, commands: &mut CommandBuffer) {
    commands.remove(entity);
}
