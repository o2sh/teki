use crate::app::components::*;
use crate::app::resources::EnemyFormation;
use crate::app::resources::GameInfo;
use lazy_static::lazy_static;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use teki_common::utils::consts::*;
use teki_common::{EnemyType, FormationIndex};
use vector2d::Vector2D;

const GHOST_SPRITES: [&str; 4] = ["enemy1", "enemy2", "enemy3", "enemy4"];

lazy_static! {
    pub static ref POSITION_ZERO: Position = Position(Vector2D::new(0, 0));
}

#[system]
pub fn spawn_enemy(#[resource] enemy_formation: &mut EnemyFormation, commands: &mut CommandBuffer) {
    if enemy_formation.done_appearance {
        return;
    }
    let mut enemies: Vec<Enemy> = Vec::new();

    for x in 0..X_COUNT {
        for y in 0..Y_COUNT {
            enemies.push(Enemy {
                enemy_type: EnemyType::Ghost,
                formation_index: FormationIndex(x, y),
            });
        }
    }

    for enemy in enemies {
        let drawable =
            SpriteDrawable { sprite_name: ENEMY_SPRITE, offset: Vector2D::new(-16, -16) };
        let hit_box = HitBox { size: Vector2D::new(32, 32) };
        commands.push((enemy, *POSITION_ZERO, hit_box, drawable));
    }
    enemy_formation.done_appearance = true;
}

#[system]
pub fn update_enemy_formation(#[resource] enemy_formation: &mut EnemyFormation) {
    enemy_formation.update();
}

#[system(for_each)]
#[write_component(Position)]
pub fn move_enemy_formation(
    enemy: &mut Enemy,
    entity: &Entity,
    world: &mut SubWorld,
    #[resource] enemy_formation: &mut EnemyFormation,
) {
    let position = <&mut Position>::query().get_mut(world, *entity).unwrap();
    position.0 = enemy_formation.pos(&enemy.formation_index);
}

#[system(for_each)]
pub fn animate_enemy(
    enemy: &mut Enemy,
    sprite: &mut SpriteDrawable,
    #[resource] game_info: &mut GameInfo,
) {
    do_animate_enemy(enemy.enemy_type, sprite, game_info.frame_count_over_5);
}

pub fn do_animate_enemy(enemy_type: EnemyType, sprite: &mut SpriteDrawable, frame_count: u32) {
    let pat = frame_count % 4;
    sprite.sprite_name = match enemy_type {
        EnemyType::Ghost => GHOST_SPRITES[pat as usize],
    };
}
