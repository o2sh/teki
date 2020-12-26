use sdl2::rect::Point;

#[derive(Clone)]
pub struct Position(pub Point);

pub struct SpriteDrawable {
    pub sprite_name: &'static str,
    pub offset: Point,
}
