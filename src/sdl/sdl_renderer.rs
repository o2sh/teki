use crate::sdl::SdlTextureManager;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use teki_common::traits::Renderer;
use teki_common::utils::SpriteSheet;
use vector2d::Vector2D;

pub struct SdlRenderer {
    canvas: WindowCanvas,
    texture_manager: SdlTextureManager,
    sprite_sheet: SpriteSheet,
}

impl SdlRenderer {
    pub fn new(mut canvas: WindowCanvas, logical_size: (u32, u32)) -> Self {
        canvas.set_logical_size(logical_size.0, logical_size.1).expect("set_logical_size failed");

        Self {
            canvas,
            texture_manager: SdlTextureManager::new(),
            sprite_sheet: SpriteSheet::default(),
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

impl Renderer for SdlRenderer {
    fn load_textures(&mut self, base_path: &str, filenames: &[&str]) {
        self.texture_manager
            .load(&mut self.canvas, base_path, filenames)
            .expect("load_textures failed");
    }

    fn load_sprite_sheet(&mut self, filename: &str) {
        let text = std::fs::read_to_string(filename).expect("load sprite sheet failed");
        self.sprite_sheet.load_sprite_sheet(&text);
    }

    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>) {
        let (sheet, tex_name) = self.sprite_sheet.get(sprite_name).expect("No sprite");

        let texture = self.texture_manager.get(tex_name).expect("No texture");
        self.canvas
            .copy(
                &texture,
                Some(Rect::new(sheet.frame.x, sheet.frame.y, sheet.frame.w, sheet.frame.h)),
                Some(Rect::new(pos.x, pos.y, sheet.frame.w as u32, sheet.frame.h as u32)),
            )
            .expect("copy failed");
    }

    fn draw_gradient(&mut self, width: i32, height: i32, padding: i32) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())
            .expect("");
        // Create a red-green gradient
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..256 {
                    for x in 0..256 {
                        let offset = y * pitch + x * 3;
                        buffer[offset + 2] = x as u8;
                        buffer[offset] = y as u8;
                        buffer[offset + 1] = 0;
                    }
                }
            })
            .expect("");

        self.canvas
            .copy(&texture, None, Some(Rect::new(padding, padding, width as u32, height as u32)))
            .expect("copy failed");
    }

    fn draw_bg(&mut self, sprite_name: &str, width: i32, height: i32, padding: i32) {
        let (sheet, tex_name) = self.sprite_sheet.get(sprite_name).expect("No sprite");

        let texture = self.texture_manager.get_mut(tex_name).expect("No texture");

        let repeat_x = width / sheet.frame.w as i32;
        let repeat_y = height / sheet.frame.h as i32;

        for x in 0..repeat_x {
            for y in 0..repeat_y {
                self.canvas
                    .copy(
                        &texture,
                        Some(Rect::new(sheet.frame.x, sheet.frame.y, sheet.frame.w, sheet.frame.h)),
                        Some(Rect::new(
                            padding + x * sheet.frame.w as i32,
                            padding + y * sheet.frame.h as i32,
                            sheet.frame.w as u32,
                            sheet.frame.h as u32,
                        )),
                    )
                    .expect("copy failed");
            }
        }
    }

    fn draw_str(&mut self, tex_name: &str, x: i32, y: i32, text: &str, r: u8, g: u8, b: u8) {
        let texture = self.texture_manager.get_mut(tex_name).expect("No texture");
        texture.set_color_mod(r, g, b);
        let w = 16;
        let h = 16;
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

    fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }
}
