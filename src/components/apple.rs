use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct AppleComponent {}

impl Component for AppleComponent {
    type Storage = NullStorage<Self>;
}
