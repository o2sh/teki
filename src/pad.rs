use bitflags::bitflags;
use sdl2::keyboard::Keycode;

bitflags! {
    #[derive(Default)]
    pub struct PadBit: u32 {
        const L = 0b00000001;
        const R = 0b00000010;
        const U = 0b00000100;
        const D = 0b00001000;
        const A = 0b00010000;
    }
}

#[derive(Clone, Default)]
pub struct Pad {
    key: PadBit,
    pad: PadBit,
    trg: PadBit,
    last_pad: PadBit,
}

impl Pad {
    pub fn update(&mut self) {
        self.pad = self.key;
        self.trg = self.pad & !self.last_pad;
        self.last_pad = self.pad;
    }

    pub fn is_pressed(&self, btn: PadBit) -> bool {
        self.pad.contains(btn)
    }

    pub fn is_trigger(&self, btn: PadBit) -> bool {
        self.trg.contains(btn)
    }

    pub fn on_key(&mut self, keycode: Keycode, down: bool) {
        let bit = get_key_bit(keycode);
        if down {
            self.key |= bit;
        } else {
            self.key &= !bit;
        }
    }
}

fn get_key_bit(key: Keycode) -> PadBit {
    match key {
        Keycode::Left => PadBit::L,
        Keycode::Right => PadBit::R,
        Keycode::Up => PadBit::U,
        Keycode::Down => PadBit::D,
        Keycode::Space => PadBit::A,
        _ => PadBit::empty(),
    }
}
