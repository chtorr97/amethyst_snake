use amethyst::{
    assets::{AssetStorage, Loader},
    core::{timing::Stopwatch, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;

use crate::components::{
    AppleComponent, GamePositionComponent, SnakeHeadComponent, SnakePartComponent,
};
use crate::game_over::GameOverState;
use crate::snake::GameState::GameOver;
use std::ops::Deref;

pub const ARENA_WIDTH: i32 = 52;
pub const ARENA_HEIGHT: i32 = 32;
pub const ARENA_PLAYABLE_WIDTH: i32 = 50;
pub const ARENA_PLAYABLE_HEIGHT: i32 = 30;

pub struct SnakeGame;

impl SimpleState for SnakeGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.delete_all();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprites = load_sprites(world);

        let snake_sprites = SnakeSprites {
            sprite_renders: sprites,
        };
        let mut next_direction = NextDirection {
            direction: Direction::Right,
            time_since_last_action: Stopwatch::new(),
        };
        next_direction.time_since_last_action.start();

        world.insert(snake_sprites);
        world.insert(next_direction);
        world.insert(GameState::Playing);

        init_camera(world, &dimensions);
        init_board(world);
        init_snake(world);
        init_apple(world);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let state = _data.world.read_resource::<GameState>();

        let mut trans = SimpleTrans::None;
        if *state == GameState::GameOver {
            trans = SimpleTrans::Replace(Box::new(GameOverState))
        }
        trans
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.width() * 0.5 - 16.,
        dimensions.height() * 0.5 - 16.,
        1.,
    );

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/snake.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/snake.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    (0..7)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn read_sprite_renderer(world: &World, sprite_key: SnakeSpritesKeys) -> SpriteRender {
    world
        .read_resource::<SnakeSprites>()
        .get_sprite_clone(sprite_key)
}

fn init_board(world: &mut World) {
    for x in 0..ARENA_WIDTH {
        for y in 0..ARENA_HEIGHT {
            let sprite_key = if x == 0 || y == 0 || x == ARENA_WIDTH - 1 || y == ARENA_HEIGHT - 1 {
                SnakeSpritesKeys::Wall
            } else {
                SnakeSpritesKeys::Grass
            };

            let sprite_render = read_sprite_renderer(world, sprite_key);

            world
                .create_entity()
                .with(sprite_render)
                .with(GamePositionComponent::new(x, y))
                .with({
                    let mut transform = Transform::default();
                    transform.set_translation_z(0.0);
                    transform
                })
                .build();
        }
    }
}

fn init_snake(world: &mut World) {
    let sprite_renderer_body = read_sprite_renderer(world, SnakeSpritesKeys::SnakeBody);
    let sprite_renderer_head = read_sprite_renderer(world, SnakeSpritesKeys::SnakeHead);

    let mut transform_head = Transform::default();
    transform_head.set_translation_xyz((11 * 32) as f32, (10 * 32) as f32, 0.);
    let mut transform_e1 = Transform::default();
    transform_e1.set_translation_xyz((10 * 32) as f32, (10 * 32) as f32, 0.);
    let mut transform_e2 = Transform::default();
    transform_e2.set_translation_xyz((10 * 32) as f32, (9 * 32) as f32, 0.);

    let e2 = world
        .create_entity()
        .with(SnakePartComponent {
            next_snake_part: None,
        })
        .with(GamePositionComponent::new(10, 10))
        .with(sprite_renderer_body.clone())
        .with({
            let mut transform = Transform::default();
            transform.set_translation_z(0.5);
            transform
        })
        .build();
    let e1 = world
        .create_entity()
        .with(SnakePartComponent {
            next_snake_part: Some(e2),
        })
        .with(GamePositionComponent::new(11, 10))
        .with(sprite_renderer_body)
        .with({
            let mut transform = Transform::default();
            transform.set_translation_z(0.5);
            transform
        })
        .build();
    world
        .create_entity()
        .with(SnakePartComponent {
            next_snake_part: Some(e1),
        })
        .with(GamePositionComponent::new(12, 10))
        .with(sprite_renderer_head)
        .with({
            let mut transform = Transform::default();
            transform.set_translation_z(0.5);
            transform
        })
        .with(SnakeHeadComponent {})
        .build();
}

fn init_apple(world: &mut World) {
    let apple_sprite = read_sprite_renderer(world, SnakeSpritesKeys::Apple);
    let mut apple_transform = Transform::default();
    apple_transform.set_translation_xyz(5.0, 5.0, 0.5);
    world
        .create_entity()
        .with(AppleComponent {})
        .with(apple_transform)
        .with(apple_sprite)
        .with(GamePositionComponent::new(5, 5))
        .build();

    world.insert(AppleWasEaten { was_eaten: false });
}

pub enum SnakeSpritesKeys {
    SnakeHead,
    SnakeBody,
    SnakeTail,
    SnakeTurn,
    Apple,
    Wall,
    Grass,
}

pub struct SnakeSprites {
    sprite_renders: Vec<SpriteRender>,
}

impl SnakeSprites {
    pub fn get_sprite_clone(&self, snake_sprite: SnakeSpritesKeys) -> SpriteRender {
        self.sprite_renders[snake_sprite as usize].clone()
    }
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct NextDirection {
    pub direction: Direction,
    pub time_since_last_action: Stopwatch,
}

pub struct AppleWasEaten {
    pub was_eaten: bool,
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}
