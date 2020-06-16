extern crate nalgebra_glm as glm;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod components;
mod game_over;
mod snake;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = resources.join("display_config.ron");
    let bindings_config = resources.join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.3, 0.3, 0.3, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(input_bundle)?
        .with(systems::InputSystem, "snake_input", &[])
        .with(systems::MoveSnakeSystem, "snake_move", &["snake_input"])
        .with(
            systems::SnakeCollisionSystem,
            "snake_collision",
            &["snake_move"],
        )
        .with(
            systems::AppleHandlerSystem,
            "apple_handler",
            &["snake_collision"],
        )
        .with(
            systems::TransformPositionsSystem,
            "transform_position",
            &["apple_handler"],
        )
        .with_bundle(TransformBundle::new().with_dep(&["transform_position"]))?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            systems::SnakeRenderSystem,
            "snake_render",
            &["transform_system"],
        );

    let mut game = Application::new(resources, snake::SnakeGame, game_data)?;
    game.run();

    Ok(())
}
