use legion::systems::CommandBuffer;
use legion::*;
use vector2d::Vector2D;

use crate::app::components::*;

pub fn new_seqanime(
    sprites: &'static [&'static str],
    offset: Vector2D<i32>,
    frame_wait: u32,
    delay: u32,
) -> SequentialSpriteAnime {
    SequentialSpriteAnime { sprites, frame_wait, delay: delay + 1, offset, count: 0 }
}

#[system(for_each)]
pub fn move_sequential_anime(
    anime: &mut SequentialSpriteAnime,
    drawable: Option<&mut SpriteDrawable>,
    entity: &Entity,
    commands: &mut CommandBuffer,
) {
    update_seqanime(anime, drawable, *entity, commands);
}

pub fn update_seqanime(
    anime: &mut SequentialSpriteAnime,
    drawable: Option<&mut SpriteDrawable>,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    if anime.delay > 0 {
        anime.delay -= 1;
        if anime.delay > 0 {
            return;
        }
        // Add SpriteDrawable component.
        if drawable.is_none() {
            std::println!("{}", anime.sprites[0]);
            commands.add_component(
                entity,
                SpriteDrawable { sprite_name: anime.sprites[0], offset: anime.offset },
            );
        }
    }

    anime.count += 1;
    if anime.count >= anime.frame_wait {
        anime.count = 0;
        anime.sprites = &anime.sprites[1..];
        if anime.sprites.len() == 0 {
            commands.remove(entity);
            return;
        }
        drawable.unwrap().sprite_name = anime.sprites[0];
    }
}

pub const ENEMY_EXPLOSION_SPRITE_TABLE: [&str; 1] = ["shockwave0"];

pub const ENEMY_EXPLOSION_FRAME: u32 = 0;

pub fn create_enemy_explosion_effect(
    pos: &Vector2D<i32>,
    delay: u32,
    commands: &mut CommandBuffer,
) {
    assert!(delay > 0);
    let anime_table = &ENEMY_EXPLOSION_SPRITE_TABLE;
    let sprite_name = anime_table[0];
    let offset = Vector2D::new(-32, -32);
    commands.push((
        Posture(pos.clone(), 0),
        new_seqanime(anime_table, offset, ENEMY_EXPLOSION_FRAME, delay),
        SpriteDrawable { sprite_name, offset },
    ));
}
