use crate::teki::ecs::components::*;
use crate::teki::ecs::EnemyType;
use crate::teki::utils::consts::*;
use legion::systems::CommandBuffer;
use legion::*;
use sdl2::rect::{Point, Rect};

#[system]
pub fn spawn_enemy(commands: &mut CommandBuffer) {
    let enemy = Enemy { enemy_type: EnemyType::Corgi };
    let position = Position(Point::new(GAME_WIDTH / 2 + 16, GAME_HEIGHT / 2));
    let drawable = SpriteDrawable { sprite_name: CORGI_SPRITE, rect: Rect::new(5, 5, 40, 40) };
    commands.push((enemy, position, drawable));
}
