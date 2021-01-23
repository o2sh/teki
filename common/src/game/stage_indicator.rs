use counted_array::counted_array;

use crate::app::consts::*;
use crate::framework::types::Vec2I;
use crate::framework::RendererTrait;

const FLAG50_WIDTH: u16 = 16;
const FLAG30_WIDTH: u16 = 16;
const FLAG20_WIDTH: u16 = 16;
const FLAG10_WIDTH: u16 = 16;
const FLAG5_WIDTH: u16 = 8;
const FLAG1_WIDTH: u16 = 8;

#[derive(Default)]
pub struct StageIndicator {
    stage: u16,
    wait: u32,
    stage_disp: u16,
    width: u16,
}

impl StageIndicator {
    pub fn set_stage(&mut self, stage: u16) {
        self.stage = stage;
        self.wait = 0;
        self.stage_disp = 0;
        self.width = calc_width(stage);
    }

    pub fn update(&mut self) -> bool {
        if self.stage_disp >= self.stage {
            return false;
        }

        if self.wait > 0 {
            self.wait -= 1;
            return false;
        }

        let diff = self.stage - self.stage_disp;
        for flag_info in FLAG_INFO_TABLE.iter() {
            if diff >= flag_info.count {
                self.stage_disp += flag_info.count;
                self.wait = 3;
                return true;
            }
        }
        false
    }

    pub fn draw<R: RendererTrait>(&self, renderer: &mut R) {
        let mut x = WIDTH - self.width as i32;
        let mut count = self.stage_disp;
        renderer.set_sprite_texture_color_mod(FLAG_INFO_TABLE[FLAG_INFO_TABLE.len() - 1].sprite_name, 255, 255, 255);
        for flag_info in FLAG_INFO_TABLE.iter() {
            while count >= flag_info.count {
                renderer.draw_sprite(flag_info.sprite_name, &Vec2I::new(x, HEIGHT - 16));
                x += flag_info.width as i32;
                count -= flag_info.count;
            }
        }
    }
}

struct FlagInfo {
    sprite_name: &'static str,
    count: u16,
    width: u16,
}

counted_array!(const FLAG_INFO_TABLE: [FlagInfo; _] = [
    FlagInfo { sprite_name: "flag50",  count: 50,  width: FLAG50_WIDTH },
    FlagInfo { sprite_name: "flag30",  count: 30,  width: FLAG30_WIDTH },
    FlagInfo { sprite_name: "flag20",  count: 20,  width: FLAG20_WIDTH },
    FlagInfo { sprite_name: "flag10",  count: 10,  width: FLAG10_WIDTH },
    FlagInfo { sprite_name: "flag5",   count: 5,   width: FLAG5_WIDTH },
    FlagInfo { sprite_name: "flag1",   count: 1,   width: FLAG1_WIDTH },
]);

fn calc_width(stage: u16) -> u16 {
    FLAG_INFO_TABLE.iter().fold((0, stage), |(w, s), info| {
        (w + (s / info.count) * info.width, s % info.count)
    }).0
}
