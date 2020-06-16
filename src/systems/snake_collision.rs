use crate::components::{GamePositionComponent, SnakeHeadComponent, SnakePartComponent};
use crate::snake::GameState::GameOver;
use crate::snake::{GameState, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};

pub struct SnakeCollisionSystem;

impl<'s> System<'s> for SnakeCollisionSystem {
    type SystemData = (
        ReadStorage<'s, SnakePartComponent>,
        ReadStorage<'s, GamePositionComponent>,
        ReadStorage<'s, SnakeHeadComponent>,
        WriteExpect<'s, GameState>,
    );

    fn run(&mut self, (snake_parts, positions, snake_heads, mut game_state): Self::SystemData) {
        let mut collision = false;
        for (snake_part, position, _) in (&snake_parts, &positions, &snake_heads).join() {
            for (other_snake_part, other_position) in (&snake_parts, &positions).join() {
                if position.position == other_position.position
                    && snake_part.next_snake_part != other_snake_part.next_snake_part
                {
                    collision = true;
                }
            }
            let pos = position.position;
            if !collision
                && (pos.x <= 0
                    || pos.x >= ARENA_WIDTH - 1
                    || pos.y <= 0
                    || pos.y >= ARENA_HEIGHT - 1)
            {
                collision = true;
            }

            if collision {
                *game_state = GameState::GameOver;
            }
        }
    }
}
