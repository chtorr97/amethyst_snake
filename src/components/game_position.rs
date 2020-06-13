use amethyst::{core::transform::Transform, ecs::prelude::*};

pub struct GamePositionComponent {
    pub position: glm::IVec2,
}

impl Component for GamePositionComponent {
    type Storage = DenseVecStorage<Self>;
}

impl GamePositionComponent {
    pub fn new(x: i32, y: i32) -> Self {
        let position = glm::vec2(x, y);

        GamePositionComponent { position }
    }

    pub fn to_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.position.x * 32) as f32,
            (self.position.y * 32) as f32,
            0.5,
        );

        transform
    }

    pub fn to_transform_with_z(&self, z: f32) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.position.x * 32) as f32,
            (self.position.y * 32) as f32,
            z,
        );

        transform
    }
}
