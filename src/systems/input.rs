use amethyst::{
    derive::SystemDesc,
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

use crate::snake::{Direction, NextDirection};

#[derive(SystemDesc)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteExpect<'s, NextDirection>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut next_direction, input): Self::SystemData) {
        if input.action_is_down("up").unwrap() {
            next_direction.direction = Direction::Up;
        }
        if input.action_is_down("down").unwrap() {
            next_direction.direction = Direction::Down;
        }
        if input.action_is_down("right").unwrap() {
            next_direction.direction = Direction::Right;
        }
        if input.action_is_down("left").unwrap() {
            next_direction.direction = Direction::Left;
        }
    }
}
