use bitflags::bitflags;

#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    Space,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Z,
}

bitflags! {
    #[derive(Default)]
    pub struct PadBit: u32 {
        const L = 0b00000001;
        const R = 0b00000010;
        const U = 0b00000100;
        const D = 0b00001000;
        const A = 0b00010000;
        const Z = 0b00100000;
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

    pub fn on_key(&mut self, key: Key, down: bool) {
        let bit = get_key_bit(key);
        if down {
            self.key |= bit;
        } else {
            self.key &= !bit;
        }
    }
}

fn get_key_bit(key: Key) -> PadBit {
    match key {
        Key::Left => PadBit::L,
        Key::Right => PadBit::R,
        Key::Up => PadBit::U,
        Key::Down => PadBit::D,
        Key::Z => PadBit::Z,
        Key::Space => PadBit::A,
        _ => PadBit::empty(),
    }
}
