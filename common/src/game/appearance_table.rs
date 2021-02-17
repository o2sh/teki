use counted_array::counted_array;

use crate::game::traj_command_table::*;
use crate::game::{FormationIndex, TrajCommand};

const fn p(x: u8, y: u8) -> FormationIndex {
    FormationIndex(x as usize, y as usize)
}

pub const UNIT_BASE: [usize; 3] = [0, 4, 8];

pub const ORDER: [FormationIndex; 14] = [
    //
    p(3, 1),
    p(4, 1),
    p(5, 1),
    p(6, 1),
    //
    p(0, 4),
    p(0, 3),
    p(9, 4),
    p(9, 3),
    //
    p(2, 2),
    p(6, 2),
    //
    p(3, 4),
    p(4, 4),
    p(5, 4),
    p(6, 4),
];

pub struct UnitTableEntry<'a> {
    pub pat: usize,
    pub table: &'a [TrajCommand],
    pub flip_x: bool,
}

counted_array!(pub const UNIT_TABLE: [[UnitTableEntry; 4]; _] = [
    [
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 0, table: &COMMAND_TABLE2, flip_x: false },
        UnitTableEntry { pat: 4, table: &COMMAND_TABLE1, flip_x: false },
        UnitTableEntry { pat: 1, table: &COMMAND_TABLE3, flip_x: false },
    ]
]);
