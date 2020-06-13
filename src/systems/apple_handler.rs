use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};
use rand::prelude::*;

use crate::components::{
    AppleComponent, GamePositionComponent, SnakeHeadComponent, SnakePartComponent,
};
use crate::snake::{
    AppleWasEaten, Direction, NextDirection, ARENA_PLAYABLE_HEIGHT, ARENA_PLAYABLE_WIDTH,
};

#[derive(SystemDesc)]
pub struct AppleHandlerSystem;

impl<'s> System<'s> for AppleHandlerSystem {
    type SystemData = (
        WriteStorage<'s, GamePositionComponent>,
        ReadStorage<'s, SnakeHeadComponent>,
        ReadStorage<'s, SnakePartComponent>,
        ReadStorage<'s, AppleComponent>,
        WriteExpect<'s, AppleWasEaten>,
    );

    fn run(
        &mut self,
        (mut game_positions, snake_heads, snake_parts, apples, mut apple_was_eaten): Self::SystemData,
    ) {
        let mut snake_head_positions: Vec<glm::IVec2> = vec![];
        for (_, game_position) in (&snake_heads, &game_positions).join() {
            snake_head_positions.push(game_position.position.clone());
        }

        let mut snake_ate_apple = false;

        for (_, game_position) in (&apples, &mut game_positions).join() {
            let apple_position = &mut game_position.position;
            for snake_head_position in &snake_head_positions {
                if *snake_head_position == *apple_position {
                    snake_ate_apple = true;
                    let mut rng = thread_rng();
                    // TODO: make sure it is not on other snake pieces.
                    let x = rng.gen_range(0, ARENA_PLAYABLE_WIDTH) + 1;
                    let y = rng.gen_range(0, ARENA_PLAYABLE_HEIGHT) + 1;

                    *apple_position = glm::vec2(x, y);

                    apple_was_eaten.was_eaten = true;
                }
            }
        }
    }
}
