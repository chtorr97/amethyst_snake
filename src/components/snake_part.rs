use amethyst::ecs::prelude::*;

pub enum SnakePartType {
    Head,
    Body,
    Tail,
}

pub struct SnakePartComponent {
    pub position: glm::IVec2,
    pub next_snake_part: Option<Entity>,
}

impl Component for SnakePartComponent {
    type Storage = DenseVecStorage<Self>;
}
