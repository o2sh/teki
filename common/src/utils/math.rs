use vector2d::Vector2D;

pub const ONE_BIT: i32 = 8;
pub const ONE: i32 = 1 << ONE_BIT;

pub fn round_vec(v: &Vector2D<i32>) -> Vector2D<i32> {
    Vector2D::new(round_up_i32(v.x), round_up_i32(v.y))
}

pub const fn round_up_i32(v: i32) -> i32 {
    (v + ONE / 2) >> ONE_BIT
}
