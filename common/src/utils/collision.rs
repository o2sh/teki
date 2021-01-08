use vector2d::Vector2D;

pub struct CollBox {
    pub top_left: Vector2D<i32>,
    pub size: Vector2D<i32>,
}

impl CollBox {
    pub fn check_collision(&self, target: &CollBox) -> bool {
        let br1 = &self.top_left + &self.size;
        let br2 = &target.top_left + &target.size;

        self.top_left.x < br2.x
            && self.top_left.y < br2.y
            && target.top_left.x < br1.x
            && target.top_left.y < br1.y
    }
}
