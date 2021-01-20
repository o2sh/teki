use vector2d::Vector2D;

pub trait Renderer {
    fn load_textures(&mut self, base_path: &str, filenames: &[&str]);
    fn load_sprite_sheet(&mut self, filename: &str);
    fn clear(&mut self);
    fn draw_str(
        &mut self,
        sprite_name: &str,
        x: i32,
        y: i32,
        size: u32,
        text: &str,
        r: u8,
        g: u8,
        b: u8,
    );
    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>);
    fn draw_scrolling_bg(&mut self, sprite_name: &str, width: i32, height: i32);
    fn set_draw_color(&mut self, r: u8, g: u8, b: u8);
    fn draw_vertical_separation(&mut self, width: i32, height: i32);
}
