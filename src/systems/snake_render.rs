use amethyst::{derive::SystemDesc, ecs::prelude::*, renderer::SpriteRender};

use crate::components::{SnakeHeadComponent, SnakePartComponent};
use crate::snake::{SnakeSprites, SnakeSpritesKeys};

#[derive(SystemDesc)]
pub struct SnakeRenderSystem;

impl<'s> System<'s> for SnakeRenderSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, SnakePartComponent>,
        ReadStorage<'s, SnakeHeadComponent>,
        ReadExpect<'s, SnakeSprites>,
    );

    fn run(&mut self, (mut sprites, parts, heads, sprite_asset): Self::SystemData) {
        for (_, part) in (&heads, &parts).join() {
            let mut next_to_check = part.next_snake_part;
            let mut last_entity = None;
            loop {
                if let Some(next_body_part_entity) = next_to_check {
                    if let Some(sprite) = sprites.get_mut(next_body_part_entity) {
                        *sprite = sprite_asset.get_sprite_clone(SnakeSpritesKeys::SnakeBody);
                    }
                    next_to_check = parts.get(next_body_part_entity).unwrap().next_snake_part;
                    last_entity = Some(next_body_part_entity);
                } else {
                    if let Some(entity) = last_entity {
                        if let Some(sprite) = sprites.get_mut(entity) {
                            *sprite = sprite_asset.get_sprite_clone(SnakeSpritesKeys::SnakeTail);
                        }
                    }
                    break;
                }
            }
        }
    }
}
