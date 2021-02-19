pub struct PlayerData<'a> {
    pub sprite: &'a str,
    pub bullet: &'a str,
    pub special: &'a str,
    pub big_orb: &'a str,
    pub neutral_face: &'a str,
    pub attack_face: &'a str,
    pub dialog_face: &'a str,
    pub damage_face: &'a str,
    pub animation_table: [[&'a str; 8]; 3],
}

impl<'a> PlayerData<'a> {
    pub fn new(character_index: u8) -> Self {
        match character_index {
            0 => Self {
                sprite: "reimu0",
                bullet: "spell0",
                special: "special0",
                big_orb: "big_orb0",
                neutral_face: "a_reimu0",
                attack_face: "a_reimu1",
                damage_face: "a_reimu2",
                dialog_face: "a_reimu3",
                animation_table: [
                    [
                        "reimu0", "reimu1", "reimu2", "reimu3", "reimu4", "reimu5", "reimu6",
                        "reimu7",
                    ],
                    [
                        "reimu8", "reimu9", "reimu10", "reimu11", "reimu12", "reimu13", "reimu14",
                        "reimu15",
                    ],
                    [
                        "reimu16", "reimu17", "reimu18", "reimu19", "reimu20", "reimu21",
                        "reimu22", "reimu23",
                    ],
                ],
            },
            _ => Self {
                sprite: "marisa0",
                bullet: "spell4",
                special: "special1",
                big_orb: "big_orb1",
                neutral_face: "a_marisa0",
                attack_face: "a_marisa1",
                damage_face: "a_marisa2",
                dialog_face: "a_marisa3",
                animation_table: [
                    [
                        "marisa0", "marisa1", "marisa2", "marisa3", "marisa4", "marisa5",
                        "marisa6", "marisa7",
                    ],
                    [
                        "marisa8", "marisa9", "marisa10", "marisa11", "marisa12", "marisa13",
                        "marisa14", "marisa15",
                    ],
                    [
                        "marisa16", "marisa17", "marisa18", "marisa19", "marisa20", "marisa21",
                        "marisa22", "marisa23",
                    ],
                ],
            },
        }
    }
}
