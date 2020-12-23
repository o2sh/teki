use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;

use crate::ecs::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        let screen_rect = Rect::from_center(pos.0, current_frame.width(), current_frame.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
