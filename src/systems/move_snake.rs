use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};

use crate::components::{GamePositionComponent, SnakeHeadComponent, SnakePartComponent};
use crate::snake::{AppleWasEaten, Direction, NextDirection, SnakeSprites, SnakeSpritesKeys};

#[derive(SystemDesc)]
pub struct MoveSnakeSystem;

impl<'s> System<'s> for MoveSnakeSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, SnakePartComponent>,
        WriteStorage<'s, GamePositionComponent>,
        ReadStorage<'s, SnakeHeadComponent>,
        WriteExpect<'s, NextDirection>,
        WriteExpect<'s, AppleWasEaten>,
        ReadExpect<'s, SnakeSprites>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut snake_parts,
            mut positions,
            snake_heads,
            mut next_direction,
            mut apple_was_eaten,
            snake_sprites,
            updater,
        ): Self::SystemData,
    ) {
        let direction = next_direction.direction.clone();
        let stop_watch = &mut next_direction.time_since_last_action;
        if stop_watch.elapsed().as_millis() > 200 {
            let mut new_head_position = glm::vec2(0, 0);
            let mut old_head_position = glm::vec2(0, 0);
            let mut next_option_entity: Option<Entity> = None;
            for (part, position, _) in (&mut snake_parts, &mut positions, &snake_heads).join() {
                old_head_position = position.position.clone();
                new_head_position = old_head_position.clone();
                match direction {
                    Direction::Up => new_head_position.y = new_head_position.y + 1,
                    Direction::Right => new_head_position.x = new_head_position.x + 1,
                    Direction::Down => new_head_position.y = new_head_position.y - 1,
                    Direction::Left => new_head_position.x = new_head_position.x - 1,
                };
                position.position = new_head_position.clone();
                next_option_entity = part.next_snake_part;
            }

            let mut next_part_position = old_head_position;
            let mut last_entity_optional = None;
            loop {
                if let Some(entity) = next_option_entity {
                    let next_position = positions.get_mut(entity).unwrap();
                    std::mem::swap(&mut next_position.position, &mut next_part_position);

                    let next_part = snake_parts.get_mut(entity).unwrap();
                    next_option_entity = next_part.next_snake_part;
                    last_entity_optional = Some(entity);
                } else {
                    match last_entity_optional {
                        Some(last_entity) if apple_was_eaten.was_eaten => {
                            let new_piece_entity = entities.create();
                            snake_parts.insert(
                                new_piece_entity,
                                SnakePartComponent {
                                    next_snake_part: None,
                                },
                            );
                            positions.insert(
                                new_piece_entity,
                                GamePositionComponent::new(
                                    next_part_position.x,
                                    next_part_position.y,
                                ),
                            );
                            updater.insert(new_piece_entity, {
                                let mut transform = Transform::default();
                                transform.set_translation_z(0.5);
                                transform
                            });
                            updater.insert(
                                new_piece_entity,
                                snake_sprites.get_sprite_clone(SnakeSpritesKeys::SnakeTail),
                            );

                            let next_part = snake_parts.get_mut(last_entity).unwrap();
                            next_part.next_snake_part = Some(new_piece_entity);

                            apple_was_eaten.was_eaten = false;
                        }
                        _ => {}
                    }
                    break;
                }
            }

            stop_watch.restart();
        }
    }
}
