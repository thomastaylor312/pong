extern crate rand;
use crate::resources::Music;
use crate::states::Pong;
use amethyst::{
    assets::PrefabLoaderSystemDesc,
    audio::{AudioBundle, DjSystem},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, scene::BasicScenePrefab},
};

mod components;
mod resources;
mod states;
mod systems;

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let binding_path = config_path.join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    // let mut world = World::new();
    let game_data = GameDataBuilder::default()
        .with_system_desc(PrefabLoaderSystemDesc::<MyPrefabData>::default(), "", &[])
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        // TODO: Figure out how to make the music stop on pause
        .with(
            DjSystem::new(|music: &mut Music| music.music.next())
                .pausable(resources::CurrentState::Running),
            "dj_system",
            &[],
        )
        .with(
            systems::PaddleSystem.pausable(resources::CurrentState::Running),
            "paddle_system",
            &["input_system"],
        )
        .with(
            systems::MoveBallsSystem.pausable(resources::CurrentState::Running),
            "move_balls_system",
            &[],
        )
        .with(
            systems::BounceSystem.pausable(resources::CurrentState::Running),
            "bounce_system",
            &["paddle_system", "move_balls_system"],
        )
        .with(
            systems::ScoringSystem::default(),
            "scoring_system",
            &["move_balls_system"],
        );
    let assets_dir = app_root.join("assets");
    // let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    let mut game: Application<GameData> =
        CoreApplication::build(assets_dir, Pong::default())?.build(game_data)?;
    game.run();

    Ok(())
}
