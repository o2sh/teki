use crate::sdl::SdlTextureManager;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, WindowCanvas};
use sdl2::ttf::*;
use teki_common::traits::Renderer;
use teki_common::utils::{consts::*, SpriteSheet};
use vector2d::Vector2D;

pub struct SdlRenderer {
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
    texture_manager: SdlTextureManager,
    sprite_sheet: SpriteSheet,
    scrolling_offset: i32,
}

impl SdlRenderer {
    pub fn new(
        mut canvas: WindowCanvas,
        ttf_context: Sdl2TtfContext,
        logical_size: (u32, u32),
    ) -> Self {
        canvas.set_logical_size(logical_size.0, logical_size.1).expect("set_logical_size failed");

        Self {
            canvas,
            ttf_context,
            texture_manager: SdlTextureManager::new(),
            sprite_sheet: SpriteSheet::default(),
            scrolling_offset: 0,
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    fn set_draw_color_with_alpha(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
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
        let (sheet, tex_name) =
            self.sprite_sheet.get(sprite_name).expect(&format!("No sprite named: {}", sprite_name));

        let texture = self.texture_manager.get(tex_name).expect("No texture");
        self.canvas
            .copy(
                &texture,
                Some(Rect::new(sheet.frame.x, sheet.frame.y, sheet.frame.w, sheet.frame.h)),
                Some(Rect::new(pos.x, pos.y, sheet.frame.w as u32, sheet.frame.h as u32)),
            )
            .expect("copy failed");
    }

    fn draw_sprite_rot(
        &mut self,
        sprite_name: &str,
        pos: &Vector2D<i32>,
        angle: u8,
        center: Option<&Vector2D<i32>>,
    ) {
        let (sheet, tex_name) = self.sprite_sheet.get(sprite_name).expect("No sprite");

        let texture = self.texture_manager.get(tex_name).expect("No texture");
        let center = center.map(|v| Point::new(v.x, v.y));
        self.canvas
            .copy_ex(
                &texture,
                Some(Rect::new(sheet.frame.x, sheet.frame.y, sheet.frame.w, sheet.frame.h)),
                Some(Rect::new(pos.x, pos.y, sheet.frame.w as u32, sheet.frame.h as u32)),
                (angle as f64) * (360.0 / 256.0),
                center,
                false,
                false,
            )
            .expect("copy_ex failed");
    }

    fn draw_scrolling_bg(&mut self, sprite_name: &str, width: i32, height: i32) {
        let (sheet, tex_name) = self.sprite_sheet.get(sprite_name).expect("No sprite");
        let texture = self.texture_manager.get_mut(tex_name).expect("No texture");

        self.scrolling_offset -= 1 * SCROLLING_BG_VEL;
        if self.scrolling_offset < -height {
            self.scrolling_offset = 0;
        }

        texture.set_alpha_mod(BG_ALPHA);

        self.canvas
            .copy(
                &texture,
                Some(Rect::new(sheet.frame.x, sheet.frame.y, sheet.frame.w, sheet.frame.h)),
                Some(Rect::new(0, self.scrolling_offset, width as u32, sheet.frame.h)),
            )
            .expect("copy failed");
    }

    fn draw_vertical_separation(&mut self, width: i32, height: i32) {
        self.set_draw_color(255, 255, 255);
        let rect = Rect::new(width, 0, 2, height as u32);
        self.canvas.fill_rect(rect).expect("");
    }

    fn draw_texture(&mut self, tex_name: &str, width: i32, height: i32) {
        let texture = self.texture_manager.get_mut(tex_name).expect("No texture");
        self.canvas
            .copy(&texture, None, Some(Rect::new(0, 0, width as u32, height as u32)))
            .expect("copy failed");
    }

    fn draw_str(
        &mut self,
        ttf_path: &str,
        x: i32,
        y: i32,
        size: u32,
        text: &str,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        bold: bool,
    ) {
        // Load a font
        let mut font = self.ttf_context.load_font(ttf_path, size as u16).expect("");
        if bold {
            font.set_style(sdl2::ttf::FontStyle::BOLD);
        }

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(Color::RGBA(r, g, b, a))
            .map_err(|e| e.to_string())
            .expect("");

        let texture =
            self.canvas.create_texture_from_surface(&surface).map_err(|e| e.to_string()).expect("");

        self.canvas
            .copy(&texture, None, Some(Rect::new(x, y, surface.width(), surface.height())))
            .expect("");
    }
    fn draw_rect(
        &mut self,
        pos: &Vector2D<i32>,
        width: i32,
        height: i32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) {
        self.canvas.set_blend_mode(BlendMode::Blend);
        self.set_draw_color_with_alpha(r, g, b, a);
        let rect = Rect::new(pos.x, pos.y, width as u32, height as u32);
        self.canvas.fill_rect(rect).expect("");
    }

    fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }
}
