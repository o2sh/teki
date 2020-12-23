use specs::prelude::*;

use crate::ecs::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;
        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Left => {
                    pos.0 = pos.0.offset(-vel.speed, 0);
                }
                Right => {
                    pos.0 = pos.0.offset(vel.speed, 0);
                }
                Up => {
                    pos.0 = pos.0.offset(0, -vel.speed);
                }
                Down => {
                    pos.0 = pos.0.offset(0, vel.speed);
                }
            }
        }
    }
}
