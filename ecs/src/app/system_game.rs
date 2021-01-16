use crate::app::resources::GameInfo;
use legion::*;

#[system]
pub fn update_game_info(#[resource] game_info: &mut GameInfo) {
    game_info.update();
}
