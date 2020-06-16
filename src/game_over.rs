use amethyst::{
    assets::Loader,
    core::*,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData,
};

use crate::snake::SnakeGame;

pub struct GameOverState;

impl SimpleState for GameOverState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        _data.world.delete_all();

        let font = _data.world.read_resource::<Loader>().load(
            "chicken.ttf",
            TtfFormat,
            (),
            &_data.world.read_resource(),
        );
        let game_over = UiTransform::new(
            "game_over".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            0.0,
            -50.,
            1.,
            1000.,
            300.,
        );
        _data
            .world
            .create_entity()
            .with(game_over)
            .with(UiText::new(
                font,
                "Game Over".to_string(),
                [1., 1., 1., 1.],
                150.,
            ))
            .build();

        let mut game_over_time = GameOverTime {
            time: Stopwatch::new(),
        };
        game_over_time.time.start();
        _data.world.insert(game_over_time);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let game_over_time = data.world.read_resource::<GameOverTime>();
        let mut trans = SimpleTrans::None;
        if game_over_time.time.elapsed().as_millis() > 2000 {
            trans = SimpleTrans::Replace(Box::new(SnakeGame));
        }
        trans
    }
}

struct GameOverTime {
    time: Stopwatch,
}
