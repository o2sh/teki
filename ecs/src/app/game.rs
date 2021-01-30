use crate::framework::{
    components::*, resources::*, system_avatar::*, system_effect::*, system_enemy::*,
    system_item::*, system_player::*, system_text::*,
};
use legion::*;
use teki_common::game::AppearanceManager;
use teki_common::traits::{Audio, Renderer};
use teki_common::utils::{consts::*, math::*, pad::Pad};
use vector2d::Vector2D;

pub struct Game {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl Game {
    pub fn new(character_index: u8) -> Self {
        let schedule = Schedule::builder()
            .add_system(update_game_info_system())
            .add_system(move_player_system())
            .add_system(fire_myshot_system())
            .add_system(move_myshot_system())
            .add_system(run_appearance_enemy_system())
            .flush()
            .add_system(animate_enemy_system())
            .add_system(animate_player_system())
            .add_system(animate_avatar_system())
            .add_system(move_enemy_system())
            .add_system(move_item_system())
            .add_system(shot_collision_check_system())
            .add_system(item_collision_check_system())
            .add_system(move_sequential_anime_system())
            .add_system(clear_text_system())
            .build();
        let mut world = World::default();

        let player = new_player(character_index);
        let player_sprite = player_sprite(&player);

        world.push((
            Avatar,
            Posture(
                Vector2D::new(
                    (GAME_WIDTH + (WINDOW_WIDTH - GAME_WIDTH) / 2) * ONE,
                    PLAYER_Y - 25 * ONE,
                ),
                0,
                0,
            ),
            SpriteDrawable {
                sprite_name: player.data.neutral_face,
                offset: Vector2D::new(-50, -50),
            },
        ));

        world.push((
            player,
            Posture(Vector2D::new(CENTER_X, PLAYER_Y), 0, 0),
            player_hit_box(),
            player_sprite,
        ));

        let mut resources = Resources::default();
        resources.insert(SoundQueue::new());
        resources.insert(AppearanceManager::default());
        resources.insert(Formation::default());
        resources.insert(GameInfo::new());
        resources.insert(StageIndicator::default());
        Self { world, resources, schedule }
    }

    pub fn update<A: Audio>(&mut self, pad: &Pad, audio: &mut A) -> bool {
        self.resources.insert(pad.clone());
        self.schedule.execute(&mut self.world, &mut self.resources);
        let mut sound_queue = self.resources.get_mut::<SoundQueue>().unwrap();
        sound_queue.flush(audio);
        true
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_scrolling_bg(BG1_TEXTURE, GAME_WIDTH, GAME_HEIGHT);
        renderer.draw_vertical_separation(GAME_WIDTH, GAME_HEIGHT);
        for (posture, drawable) in <(&Posture, &SpriteDrawable)>::query().iter(&self.world) {
            let pos = round_vec(&posture.0) + drawable.offset;
            let angle = quantize_angle(posture.1, ANGLE_DIV);
            if angle == 0 {
                renderer.draw_sprite(drawable.sprite_name, &pos);
            } else {
                renderer.draw_sprite_rot(drawable.sprite_name, &pos, angle, None);
            }
        }
        for (posture, text) in <(&Posture, &Text)>::query().iter(&self.world) {
            let pos = round_vec(&posture.0) + text.offset;
            renderer.draw_str(RE_FONT, pos.x, pos.y, 10, &text.msg, &text.color, false);
        }
        if let Some(game_info) = get_game_info(&self.resources) {
            game_info.draw(renderer);
            match game_info.game_state {
                GameState::StartStage => {
                    if let Some(stage_indicator) = get_stage_indicator(&self.resources) {
                        stage_indicator.draw(renderer, game_info.frame_count);
                    }
                }
                _ => {}
            }
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
