use counted_array::counted_array;

use crate::game::traj_command_table::*;
use crate::game::{FormationIndex, TrajCommand};

const fn p(x: u8, y: u8) -> FormationIndex {
    FormationIndex(x as usize, y as usize)
}

pub const ORDER: [FormationIndex; 40] = [
    p(4, 2),
    p(5, 2),
    p(4, 3),
    p(5, 3),
    p(4, 4),
    p(5, 4),
    p(4, 5),
    p(5, 5),
    p(3, 1),
    p(4, 1),
    p(5, 1),
    p(6, 1),
    p(3, 2),
    p(6, 2),
    p(3, 3),
    p(6, 3),
    p(8, 2),
    p(7, 2),
    p(8, 3),
    p(7, 3),
    p(1, 2),
    p(2, 2),
    p(1, 3),
    p(2, 3),
    p(7, 4),
    p(6, 4),
    p(7, 5),
    p(6, 5),
    p(3, 4),
    p(2, 4),
    p(3, 5),
    p(2, 5),
    p(9, 4),
    p(8, 4),
    p(9, 5),
    p(8, 5),
    p(0, 4),
    p(1, 4),
    p(0, 5),
    p(1, 5),
];

pub struct UnitTableEntry<'a> {
    pub pat: usize,
    pub table: &'a [TrajCommand],
    pub flip_x: bool,
}

counted_array!(pub const UNIT_TABLE: [[UnitTableEntry; 5]; _] = [
    [
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE2, flip_x: true },
        UnitTableEntry { pat: 2, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 2, table: &COMMAND_TABLE1, flip_x: true },
    ],
    [
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE3, flip_x: true },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE2, flip_x: true },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE1, flip_x: true },
    ],
    [
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE2, flip_x: true },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE1, flip_x: false },
    ],
    [
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE3, flip_x: true },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE2, flip_x: true },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE3, flip_x: false },
        UnitTableEntry { pat: 3, table: &COMMAND_TABLE3, flip_x: true },
    ],
]);
