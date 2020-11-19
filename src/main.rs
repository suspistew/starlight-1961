mod states;
mod entities;
mod systems;
mod resources;
mod utils;

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
use crate::states::level::LevelState;
use crate::systems::ship_systems::ShipSystem;
use crate::systems::collision_system::CollisionSystem;
use crate::entities::ship::Thrusters;
use crate::systems::thruster_system::ThrustersSystem;
use crate::systems::landing_system::LandingSystem;
use amethyst::renderer::palette::Srgba;
use crate::systems::explosion_systems::ExplosionSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("assets/config/display_config.ron");
    let key_bindings_path = app_root.join("assets/config/bindings.ron");

    let (r, g, b, a) = Srgba::new(31. / 255., 54. / 255., 50. / 255., 1.)
         .into_linear()
         .into_components();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([r, g, b, a]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(
            ShipSystem,
            "ship_system",
            &["input_system"],
        )
        .with(
            CollisionSystem,
            "collision_system",
            &[],
        )
        .with(
            ThrustersSystem,
            "thrusters_system",
            &[],
        )
        .with(
            LandingSystem,
            "landing_system",
            &[],
        )
        .with(
            ExplosionSystem::new(),
            "explosion_system",
            &[],
        )
        ;

    let mut game = Application::new(resources, LevelState, game_data)?;
    game.run();

    Ok(())
}