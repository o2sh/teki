use counted_array::counted_array;

use crate::game::TrajCommand;
use crate::game::TrajCommand::*;

use crate::utils::consts::*;
use crate::utils::math::{ANGLE, ONE};

counted_array!(pub const COMMAND_TABLE1: [TrajCommand; _] = [
    Pos((GAME_WIDTH / 2 + 40) * ONE, -8 * ONE),
    Speed(3 * ONE),
    Angle((ANGLE / 2) * ONE),
    VAngle(0),
    Delay(8),
]);

counted_array!(pub const COMMAND_TABLE2: [TrajCommand; _] = [
    Pos(100 * ONE, -8 * ONE),
    Speed(4 * ONE),
    Angle((ANGLE / 2) * ONE),
    VAngle(0),
    Delay(10)
]);

counted_array!(pub const COMMAND_TABLE3: [TrajCommand; _] = [
    Pos(-8 * ONE, (GAME_WIDTH / 2 -40) * ONE),
    Speed(3 * ONE),
    Angle((ANGLE / 4) * ONE),
    VAngle(-2 * ONE),
    Delay(16),
    VAngle(0),
    Delay(10),
    VAngle(-2 * ONE),
    Delay(17),
    Accelerate,
    DestAngle((-ANGLE + (ANGLE / 8)) * ONE, 20 * ONE),
    VAngle(0),
]);
