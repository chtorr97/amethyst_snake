use amethyst::ecs::prelude::*;

pub struct AppleComponent {
    position: glm::IVec2,
}

impl Component for AppleComponent {
    type Storage = DenseVecStorage<Self>;
}
