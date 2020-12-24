use specs::prelude::*;

use crate::ecs::components::*;

pub struct Shooting;

impl<'a> System<'a> for Shooting {
    type SystemData = ReadStorage<'a, Gun>;

    fn run(&mut self, data: Self::SystemData) {
        for gun in data.join() {
            if gun.fire {
                std::println!("Shoot!!");
            }
        }
    }
}
