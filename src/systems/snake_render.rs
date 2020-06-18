use std::f32::consts::PI;

use amethyst::{
    core::Transform, derive::SystemDesc, ecs::prelude::*, prelude::*, renderer::SpriteRender,
};

use crate::components::{GamePositionComponent, SnakeHeadComponent, SnakePartComponent};
use crate::snake::{Direction, SnakeSprites, SnakeSpritesKeys};

#[derive(SystemDesc)]
pub struct SnakeRenderSystem;

impl<'s> System<'s> for SnakeRenderSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, GamePositionComponent>,
        ReadStorage<'s, SnakePartComponent>,
        ReadStorage<'s, SnakeHeadComponent>,
        ReadExpect<'s, SnakeSprites>,
    );

    fn run(
        &mut self,
        (entities, mut sprites, mut transforms, game_positions, parts, heads, sprite_asset): Self::SystemData,
    ) {
        let mut current_position = &glm::vec2(0, 0);
        let mut next_position = &glm::vec2(0, 0);
        let mut previous_position = &glm::vec2(0, 0);
        let mut head_entity = None;
        for (_, position, entity) in (&heads, &game_positions, &entities).join() {
            current_position = &position.position;
            previous_position = &current_position;
            head_entity = Some(entity);
        }

        for (_, part) in (&heads, &parts).join() {
            let mut next_to_check = part.next_snake_part;
            let mut last_entity = None;

            next_position = &game_positions.get(next_to_check.unwrap()).unwrap().position;
            let head_direction = get_next_direction(current_position, next_position);

            let mut angle = match head_direction {
                Direction::Up => PI,
                Direction::Down => 0.0,
                Direction::Right => PI / 2.0,
                Direction::Left => -PI / 2.0,
            };

            let mut head_transform = transforms.get_mut(head_entity.unwrap()).unwrap();
            head_transform.set_rotation_2d(angle);

            loop {
                if let Some(next_body_part_entity) = next_to_check {
                    current_position = next_position;
                    next_to_check = parts.get(next_body_part_entity).unwrap().next_snake_part;
                    if next_to_check.is_some() {
                        next_position =
                            &game_positions.get(next_to_check.unwrap()).unwrap().position;
                    } else {
                        last_entity = Some(next_body_part_entity);
                        break;
                    }

                    if let Some(sprite) = sprites.get_mut(next_body_part_entity) {
                        if are_axis_aligned(previous_position, next_position) {
                            let are_x_aligned = are_x_aligned(previous_position, next_position);

                            let mut angle = match are_x_aligned {
                                true => 0.0,
                                false => PI / 2.0,
                            };

                            let mut transform = transforms.get_mut(next_body_part_entity).unwrap();
                            transform.set_rotation_2d(angle);

                            *sprite = sprite_asset.get_sprite_clone(SnakeSpritesKeys::SnakeBody);
                        } else {
                            let direction_1 =
                                get_next_direction(previous_position, current_position);
                            let direction_2 = get_next_direction(current_position, next_position);
                            let angle = match (direction_1, direction_2) {
                                (Direction::Right, Direction::Down)
                                | (Direction::Up, Direction::Left) => 0.0,
                                (Direction::Right, Direction::Up)
                                | (Direction::Down, Direction::Left) => -PI / 2.0,
                                (Direction::Down, Direction::Right)
                                | (Direction::Left, Direction::Up) => PI,
                                (Direction::Left, Direction::Down)
                                | (Direction::Up, Direction::Right) => PI / 2.0,
                                _ => break,
                            };
                            let mut transform = transforms.get_mut(next_body_part_entity).unwrap();
                            transform.set_rotation_2d(angle);
                            *sprite = sprite_asset.get_sprite_clone(SnakeSpritesKeys::SnakeTurn);
                        }
                    }
                    previous_position = &current_position;
                }
            }
            if let Some(entity) = last_entity {
                current_position = next_position;
                if let Some(sprite) = sprites.get_mut(entity) {
                    let tail_direction = get_next_direction(previous_position, current_position);

                    let mut angle = match tail_direction {
                        Direction::Up => PI,
                        Direction::Down => 0.0,
                        Direction::Right => PI / 2.0,
                        Direction::Left => -PI / 2.0,
                    };

                    let mut tail_transform = transforms.get_mut(entity).unwrap();
                    tail_transform.set_rotation_2d(angle);

                    *sprite = sprite_asset.get_sprite_clone(SnakeSpritesKeys::SnakeTail);
                }
            }
        }
    }
}

fn get_next_direction(current_position: &glm::IVec2, next_position: &glm::IVec2) -> Direction {
    let delta_x = next_position.x - current_position.x;
    let delta_y = next_position.y - current_position.y;

    match (delta_x, delta_y) {
        (1, 0) => Direction::Right,
        (-1, 0) => Direction::Left,
        (0, 1) => Direction::Up,
        (0, -1) => Direction::Down,
        _ => panic!("Some wrong calculations"),
    }
}

fn are_axis_aligned(previous_position: &glm::IVec2, next_position: &glm::IVec2) -> bool {
    let x_aligned = are_x_aligned(previous_position, next_position);
    let y_aligned = are_y_aligned(previous_position, next_position);
    x_aligned ^ y_aligned
}

fn are_x_aligned(previous_position: &glm::IVec2, next_position: &glm::IVec2) -> bool {
    previous_position.x == next_position.x
}

fn are_y_aligned(previous_position: &glm::IVec2, next_position: &glm::IVec2) -> bool {
    previous_position.y == next_position.y
}
