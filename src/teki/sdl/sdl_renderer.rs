use crate::teki::ecs::components::*;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
pub struct SdlRenderer {
    canvas: WindowCanvas,
}

impl SdlRenderer {
    pub fn new(mut canvas: WindowCanvas, logical_size: (u32, u32)) -> Self {
        canvas.set_logical_size(logical_size.0, logical_size.1).expect("set_logical_size failed");

        Self { canvas }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn draw_sprite(&mut self, sprite: &SpriteDrawable, pos: Point) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(sprite.sprite_name).expect("No texture");

        let screen_rect = Rect::from_center(pos, sprite.rect.width(), sprite.rect.height());

        self.canvas.copy(&texture, sprite.rect, screen_rect).expect("copy failed");
    }

    pub fn draw_bg(&mut self, path: &str) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(path).expect("No texture");

        self.canvas.copy(&texture, None, None).expect("copy failed");
    }

    pub fn draw_str(&mut self, path: &str, x: i32, y: i32, text: &str) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(path).expect("No texture");

        let w = 8;
        let h = 8;
        let mut x = x;

        for c in text.chars() {
            let u: i32 = ((c as i32) - (' ' as i32)) % 16 * 8;
            let v: i32 = ((c as i32) - (' ' as i32)) / 16 * 8;
            self.canvas
                .copy(&texture, Some(Rect::new(u, v, 8, 8)), Some(Rect::new(x, y, w, h)))
                .expect("copy failed");
            x += w as i32;
        }
    }
}
