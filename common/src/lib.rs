pub mod traits;
pub mod utils;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Ghost,
}

pub enum ItemType {
    Red,
    Blue
}
pub struct FormationIndex(pub usize, pub usize);
