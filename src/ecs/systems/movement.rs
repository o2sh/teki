use specs::prelude::*;

use crate::ecs::components::*;

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData =
        (ReadExpect<'a, (u32, u32)>, WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        let (width, height) = &*data.0;

        for (pos, vel) in (&mut data.1, &data.2).join() {
            match vel.direction {
                Left => {
                    if pos.0.x > 0 {
                        pos.0 = pos.0.offset(-vel.speed, 0);
                    }
                }
                Right => {
                    if pos.0.x < *width as i32 {
                        pos.0 = pos.0.offset(vel.speed, 0);
                    }
                }
                Up => {
                    if pos.0.y > 0 {
                        pos.0 = pos.0.offset(0, -vel.speed);
                    }
                }
                Down => {
                    if pos.0.y < *height as i32 {
                        pos.0 = pos.0.offset(0, vel.speed);
                    }
                }
            }
        }
    }
}
