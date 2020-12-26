use crate::components::*;
use legion::systems::CommandBuffer;
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

//#[system(for_each)]
//#[write_component(Position)]
//pub fn move_player(
//    player: &mut Player, entity: &Entity,
//    #[resource] pad: &Pad,
//    #[resource] game_info: &mut GameInfo,
//    #[resource] attack_manager: &mut AttackManager,
//    world: &mut SubWorld, commands: &mut CommandBuffer,
//) {
//    do_move_player(player, pad, *entity, game_info, attack_manager, world, commands);
//}
