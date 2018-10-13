extern crate amethyst;
extern crate rand;

use amethyst::{
    core::{
        transform::TransformBundle, 
        frame_limiter::FrameRateLimitStrategy,
    },
    prelude::*,
    renderer::{DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage},
    input::InputBundle,
};
use std::time::Duration;

mod systems;
mod snake;
mod game;
use game::SnakeGame;

mod utilities;



fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources_dir = format!("{}/resources/", env!("CARGO_MANIFEST_DIR"));
    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

    let config = DisplayConfig::load(resources_dir + "display.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    let input_bundle = InputBundle::<String, String>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor().with_sprite_visibility_sorting(&["transform_system"]))?
        .with_bundle(systems::SnakeSystemBundle)?
        .with_bundle(input_bundle)?;

    Application::build(assets_dir, SnakeGame)?
        //.with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1000)),60)
        .build(game_data)?
        .run();
    Ok(())
}