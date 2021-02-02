use counted_array::counted_array;

use crate::game::traj_command_table::*;
use crate::game::{FormationIndex, TrajCommand};

const fn p(x: u8, y: u8) -> FormationIndex {
    FormationIndex(x as usize, y as usize)
}

pub const ORDER: [FormationIndex; 12] = [
    p(3, 1),
    p(4, 1),
    p(5, 1),
    p(6, 1),
    p(0, 4),
    p(0, 3),
    p(9, 4),
    p(9, 3),
    p(3, 5),
    p(4, 5),
    p(5, 5),
    p(6, 5),
];

pub struct UnitTableEntry<'a> {
    pub pat: usize,
    pub table: &'a [TrajCommand],
    pub flip_x: bool,
}

counted_array!(pub const UNIT_TABLE: [[UnitTableEntry; 5]; _] = [
    [
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE3, flip_x: false },
        UnitTableEntry { pat: 2, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 2, table: &COMMAND_TABLE1, flip_x: true },
    ]
]);
