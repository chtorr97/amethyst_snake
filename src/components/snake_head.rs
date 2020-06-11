use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct SnakeHeadComponent {}

impl Component for SnakeHeadComponent {
    type Storage = NullStorage<Self>;
}
