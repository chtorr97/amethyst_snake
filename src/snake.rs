use amethyst::{
    assets::{AssetStorage, Loader},
    core::{timing::Stopwatch, transform::Transform},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;

use crate::components::{SnakeHeadComponent, SnakePartComponent};

const ARENA_WIDTH: i32 = 52;
const ARENA_HEIGHT: i32 = 32;

pub struct SnakeGame;

impl SimpleState for SnakeGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.register::<SnakePartComponent>();

        init_camera(world, &dimensions);

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

        init_board(world);
        init_snake(world);
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
            let x_pos = x * 32;
            let y_pos = y * 32;
            let mut transform = Transform::default();
            transform.set_translation_xyz(x_pos as f32, y_pos as f32, -1.0);

            let sprite_key = if x == 0 || y == 0 || x == ARENA_WIDTH - 1 || y == ARENA_HEIGHT - 1 {
                SnakeSpritesKeys::Wall
            } else {
                SnakeSpritesKeys::Grass
            };

            let sprite_render = read_sprite_renderer(world, sprite_key);

            world
                .create_entity()
                .with(sprite_render)
                .with(transform)
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
            position: glm::vec2(10, 10),
            next_snake_part: None,
        })
        .with(sprite_renderer_body.clone())
        .with(transform_e2)
        .build();
    let e1 = world
        .create_entity()
        .with(SnakePartComponent {
            position: glm::vec2(10, 10),
            next_snake_part: Some(e2),
        })
        .with(sprite_renderer_body)
        .with(transform_e1)
        .build();
    world
        .create_entity()
        .with(SnakePartComponent {
            position: glm::vec2(11, 10),
            next_snake_part: Some(e1),
        })
        .with(sprite_renderer_head)
        .with(transform_head)
        .with(SnakeHeadComponent {})
        .build();
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
