use crate::framework::components::HitBox;
use teki_common::utils::collision::CollBox;
use teki_common::utils::math::round_vec;
use vector2d::Vector2D;

pub mod components;
pub mod resources;
pub mod system_avatar;
pub mod system_effect;
pub mod system_enemy;
pub mod system_item;
pub mod system_player;
pub mod system_text;

pub fn pos_to_coll_box(pos: &Vector2D<i32>, coll_rect: &HitBox) -> CollBox {
    CollBox { top_left: round_vec(pos), size: coll_rect.size }
}
