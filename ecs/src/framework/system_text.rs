use crate::framework::components::*;
use legion::systems::CommandBuffer;
use legion::*;

#[system(for_each)]
pub fn clear_text(text: &mut Text, entity: &Entity, commands: &mut CommandBuffer) {
    if text.delay > 0 {
        text.delay -= 1;
        if text.delay > 0 {
            return;
        }
    }

    commands.remove(*entity);
}
