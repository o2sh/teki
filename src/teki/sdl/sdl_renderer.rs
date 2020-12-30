use crate::teki::consts::*;
use crate::teki::ecs::components::*;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
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

    pub fn set_draw_gradient(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::IYUV, 256, 256)
            .map_err(|e| e.to_string())
            .expect("");
        // Create a U-V gradient
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                // `pitch` is the width of the Y component
                // The U and V components are half the width and height of Y

                let w = 256;
                let h = 256;

                // Set Y (constant)
                for y in 0..h {
                    for x in 0..w {
                        let offset = y * pitch + x;
                        buffer[offset] = 128;
                    }
                }

                let y_size = pitch * h;

                // Set U and V (X and Y)
                for y in 0..h / 2 {
                    for x in 0..w / 2 {
                        let u_offset = y_size + y * pitch / 2 + x;
                        let v_offset = y_size + (pitch / 2 * h / 2) + y * pitch / 2 + x;
                        buffer[u_offset] = (x * 2) as u8;
                        buffer[v_offset] = (y * 2) as u8;
                    }
                }
            })
            .expect("");

        self.canvas.copy(&texture, None, None).expect("copy failed");
    }

    pub fn draw_sprite(&mut self, sprite: &SpriteDrawable, pos: Point) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(sprite.sprite_name).expect("No texture");

        let screen_rect = Rect::from_center(pos, sprite.rect.width(), sprite.rect.height());

        self.canvas.copy(&texture, sprite.rect, screen_rect).expect("copy failed");
    }

    pub fn draw_bg(&mut self, path: &str, is_fullscreen: bool) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(path).expect("No texture");
        let (w, h) = if is_fullscreen {
            (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        } else {
            (GAME_WIDTH as u32, GAME_HEIGHT as u32)
        };
        self.canvas
            .copy(&texture, None, Some(Rect::new(PADDING, PADDING, w, h)))
            .expect("copy failed");
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

    pub fn clear(&mut self) {
        self.canvas.clear();
    }
}
