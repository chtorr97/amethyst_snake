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
    utils::application_root_dir,
};

mod components;
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
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::InputSystem, "snake_input_system", &[])
        .with(
            systems::MoveSnakeSystem,
            "move_snake_system",
            &["snake_input_system"],
        )
        .with(
            systems::SnakeRenderSystem,
            "snake_render_system",
            &["move_snake_system"],
        );

    let mut game = Application::new(resources, snake::SnakeGame, game_data)?;
    game.run();

    Ok(())
}
