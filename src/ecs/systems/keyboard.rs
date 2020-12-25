use specs::prelude::*;

use crate::consts::*;
use crate::ecs::components::*;
use crate::MovementCommand;
use crate::ShootCommand;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadExpect<'a, Option<ShootCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Gun>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        if let Some(movement_command) = &*data.0 {
            for (_, vel) in (&data.2, &mut data.3).join() {
                match movement_command {
                    &MovementCommand::Move(direction) => {
                        vel.speed = PLAYER_MOVEMENT_SPEED;
                        vel.direction = direction;
                    }
                    MovementCommand::Stop => vel.speed = 0,
                }
            }
        };

        if let Some(shoot_command) = &*data.1 {
            for (_, gun) in (&data.2, &mut data.4).join() {
                match shoot_command {
                    ShootCommand::Fire => gun.fire = true,
                    ShootCommand::Idle => gun.fire = false,
                }
            }
        };
    }
}
