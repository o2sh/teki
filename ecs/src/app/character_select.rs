use teki_common::game::RGBA;
use teki_common::traits::{Audio, Renderer};
use teki_common::utils::consts::*;
use teki_common::utils::pad::{Pad, PadBit};
use vector2d::Vector2D;

const CHARACTER_SELECT_PORTRAITS: [&str; 2] = ["player_select_reimu", "player_select_marisa"];
const CHARACTER_SELECT_DESCS: [&str; 2] = ["desc_char_reimu", "desc_char_marisa"];

#[derive(Default)]
pub struct CharacterSelect {
    pub index: i32,
    count: u32,
}

impl CharacterSelect {
    pub fn update<A: Audio>(&mut self, pad: &Pad, audio: &mut A) -> Option<(bool, u8)> {
        self.count += 1;
        if pad.is_trigger(PadBit::Z) {
            audio.play_sound(CH_KILL, SE_OK);
            return Some((true, self.index as u8));
        }
        if pad.is_trigger(PadBit::L) {
            audio.play_sound(CH_KILL, SE_SELECT);
            self.index += 1;
            if self.index > 1 {
                self.index = 0;
            }
        }
        if pad.is_trigger(PadBit::R) {
            audio.play_sound(CH_KILL, SE_SELECT);
            self.index -= 1;
            if self.index < 0 {
                self.index = 1;
            }
        }
        None
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_texture("menu_bg", WINDOW_WIDTH, WINDOW_HEIGHT);
        renderer
            .draw_sprite(CHARACTER_SELECT_PORTRAITS[self.index as usize], &Vector2D::new(-150, 50));
        let msg = "Select Character";
        renderer.draw_str(
            IM_FONT,
            10,
            10,
            32,
            msg,
            &RGBA { r: 255, g: 255, b: 255, a: 255 },
            false,
        );
        renderer.draw_sprite(CHARACTER_SELECT_DESCS[self.index as usize], &Vector2D::new(400, 220));
    }
}
