use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use teki_common::traits::Renderer;
use teki_common::utils::collision::VRect;
use teki_common::utils::consts::*;
use vector2d::Vector2D;

pub struct SdlRenderer {
    canvas: WindowCanvas,
    sprite_sheet: HashMap<String, VRect>,
}

impl SdlRenderer {
    pub fn new(mut canvas: WindowCanvas, logical_size: (u32, u32)) -> Self {
        canvas.set_logical_size(logical_size.0, logical_size.1).expect("set_logical_size failed");

        Self { canvas, sprite_sheet: HashMap::new() }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

impl Renderer for SdlRenderer {
    fn load_sprite(&mut self, path: &str, vrect: VRect) {
        self.sprite_sheet.insert(String::from(path), vrect);
    }

    fn set_draw_gradient(&mut self) {
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

    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(sprite_name).expect("No texture");
        let sprite_rect = self.sprite_sheet.get(sprite_name).unwrap();
        let screen_rect = Rect::from_center(Point::new(pos.x, pos.y), sprite_rect.w, sprite_rect.h);

        self.canvas
            .copy(
                &texture,
                Rect::new(sprite_rect.x, sprite_rect.y, sprite_rect.w, sprite_rect.h),
                screen_rect,
            )
            .expect("copy failed");
    }

    fn draw_bg(&mut self, path: &str) {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(path).expect("No texture");
        let (w, h) = (GAME_WIDTH as u32, GAME_HEIGHT as u32);

        self.canvas
            .copy(&texture, None, Some(Rect::new(PADDING, PADDING, w, h)))
            .expect("copy failed");
    }

    fn draw_str(&mut self, path: &str, x: i32, y: i32, text: &str, r: u8, g: u8, b: u8) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.load_texture(path).expect("No texture");
        texture.set_color_mod(r, g, b);
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

    fn clear(&mut self) {
        self.canvas.clear();
    }
}
