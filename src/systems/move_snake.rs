use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};

use crate::components::{SnakeHeadComponent, SnakePartComponent};
use crate::snake::{Direction, NextDirection};

#[derive(SystemDesc)]
pub struct MoveSnakeSystem;

impl<'s> System<'s> for MoveSnakeSystem {
    type SystemData = (
        WriteStorage<'s, SnakePartComponent>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, SnakeHeadComponent>,
        WriteExpect<'s, NextDirection>,
    );

    fn run(
        &mut self,
        (mut snake_parts, mut transforms, snake_heads, mut next_direction): Self::SystemData,
    ) {
        let direction = next_direction.direction.clone();
        let stop_watch = &mut next_direction.time_since_last_action;
        if stop_watch.elapsed().as_millis() > 500 {
            let mut new_head_position = glm::vec2(0, 0);
            let mut old_head_position = glm::vec2(0, 0);
            let mut next_option_entity: Option<Entity> = None;
            for (part, _) in (&mut snake_parts, &snake_heads).join() {
                old_head_position = part.position.clone();
                new_head_position = old_head_position.clone();
                match direction {
                    Direction::Up => new_head_position.y = new_head_position.y + 1,
                    Direction::Right => new_head_position.x = new_head_position.x + 1,
                    Direction::Down => new_head_position.y = new_head_position.y - 1,
                    Direction::Left => new_head_position.x = new_head_position.x - 1,
                };
                part.position = new_head_position.clone();
                next_option_entity = part.next_snake_part;
            }

            let mut next_part_position = old_head_position;
            loop {
                if let Some(entity) = next_option_entity {
                    let next_part = snake_parts.get_mut(entity).unwrap();
                    std::mem::swap(&mut next_part.position, &mut next_part_position);
                    next_option_entity = next_part.next_snake_part;
                } else {
                    break;
                }
            }

            for (part, transform) in (&snake_parts, &mut transforms).join() {
                transform.set_translation_x((part.position.x * 32) as f32);
                transform.set_translation_y((part.position.y * 32) as f32);
            }

            stop_watch.restart();
        }
    }
}
