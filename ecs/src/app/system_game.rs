use crate::app::components::*;
use crate::app::resources::{GameInfo, GameState, StageIndicator};
use legion::world::SubWorld;
use legion::*;
use teki_common::traits::Renderer;
use teki_common::utils::{consts::*, math::*};

#[system]
#[write_component(Player)]
pub fn update_game_info(
    #[resource] game_info: &mut GameInfo,
    #[resource] stage_indicator: &mut StageIndicator,
    world: &mut SubWorld,
) {
    game_info.update(stage_indicator, world);
}

pub fn draw_game<R: Renderer>(world: &World, resources: &Resources, renderer: &mut R) {
    renderer.draw_scrolling_bg(BG1_TEXTURE, GAME_WIDTH, GAME_HEIGHT);
    renderer.draw_vertical_separation(GAME_WIDTH, GAME_HEIGHT);
    for (posture, drawable) in <(&Posture, &SpriteDrawable)>::query().iter(world) {
        //std::println!("{}", drawable.sprite_name);
        let pos = round_vec(&posture.0) + drawable.offset;
        let angle = quantize_angle(posture.1, ANGLE_DIV);

        if angle == 0 {
            renderer.draw_sprite(drawable.sprite_name, &pos);
        } else {
            renderer.draw_sprite_rot(drawable.sprite_name, &pos, angle, None);
        }
    }

    for (posture, text) in <(&Posture, &Text)>::query().iter(world) {
        let pos = round_vec(&posture.0) + text.offset;
        renderer.draw_str(RE_FONT, pos.x, pos.y, 10, &text.msg, &text.color, false);
    }

    if let Some(game_info) = get_game_info(resources) {
        game_info.draw(renderer);

        match game_info.game_state {
            GameState::StartStage => {
                if let Some(stage_indicator) = get_stage_indicator(resources) {
                    stage_indicator.draw(renderer, game_info.count);
                }
            }
            _ => {}
        }
    }
}

fn get_game_info(resources: &Resources) -> Option<legion::systems::Fetch<'_, GameInfo>> {
    resources.get::<GameInfo>()
}

fn get_stage_indicator(
    resources: &Resources,
) -> Option<legion::systems::Fetch<'_, StageIndicator>> {
    resources.get::<StageIndicator>()
}
