use crate::utils::collision::VRect;
use vector2d::Vector2D;

pub trait Renderer {
    fn load_sprite(&mut self, path: &str, vrect: VRect);
    fn set_draw_gradient(&mut self);
    fn clear(&mut self);
    fn draw_str(&mut self, path: &str, x: i32, y: i32, text: &str, r: u8, g: u8, b: u8);
    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>);
    fn draw_bg(&mut self, path: &str, is_fullscreen: bool);
}
