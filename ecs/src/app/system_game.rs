use crate::app::resources::{GameInfo, StageIndicator};
use legion::*;

#[system]
pub fn update_game_info(
    #[resource] game_info: &mut GameInfo,
    #[resource] stage_indicator: &mut StageIndicator,
) {
    game_info.update(stage_indicator);
}
