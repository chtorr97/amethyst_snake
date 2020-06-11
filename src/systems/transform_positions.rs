use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};

use crate::components::GamePositionComponent;

#[derive(SystemDesc)]
pub struct TransformPositionsSystem;

impl<'s> System<'s> for TransformPositionsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, GamePositionComponent>,
    );

    fn run(&mut self, (mut transforms, game_positions): Self::SystemData) {
        for (transform, game_position) in (&mut transforms, &game_positions).join() {
            *transform = game_position.to_transform_with_z(transform.translation().z);
        }
    }
}
