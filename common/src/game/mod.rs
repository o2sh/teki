pub mod appearance_manager;
mod appearance_table;
pub mod formation_table;
mod player_data;
pub mod traj;
mod traj_command;
mod traj_command_table;

pub use self::appearance_manager::AppearanceManager;
pub use self::player_data::PlayerData;
pub use self::traj::Traj;
pub use self::traj_command::TrajCommand;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Fairy,
    BigFairy,
}

pub enum ItemType {
    Red,
    Blue,
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct FormationIndex(pub usize, pub usize);
