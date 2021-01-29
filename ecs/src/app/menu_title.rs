use teki_common::game::RGBA;
use teki_common::traits::{Audio, Renderer};
use teki_common::utils::consts::*;
use teki_common::utils::pad::{Pad, PadBit};
use vector2d::Vector2D;

pub struct Title;

impl Title {
    pub fn update<A: Audio>(&mut self, pad: &Pad, audio: &mut A) -> Option<bool> {
        if pad.is_pressed(PadBit::Z) {
            audio.play_sound(CH_KILL, SE_OK);
            return Some(true);
        }
        None
    }

    pub fn draw<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_texture("menu_bg", WINDOW_WIDTH, WINDOW_HEIGHT);
        renderer.draw_sprite("title", &Vector2D::new(WINDOW_WIDTH - 432, 0));
        let title = "Teki";
        renderer.draw_str(
            IM_FONT,
            430,
            180,
            50,
            title,
            &RGBA { r: 255, g: 255, b: 255, a: 255 },
            false,
        );

        let msg = "Press z to start";
        renderer.draw_str(
            RE_FONT,
            50,
            WINDOW_HEIGHT / 2 - 50,
            18,
            msg,
            &RGBA { r: 255, g: 255, b: 255, a: 255 },
            false,
        );
    }
}
