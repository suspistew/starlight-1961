mod entities;
mod resources;
mod states;
mod systems;
mod utils;

use crate::states::main_menu_state::MainMenuState;
use crate::states::CurrentState;
use crate::systems::blade_saw_system::BladeSawSystem;
use crate::systems::bonus_system::BonusSystem;
use crate::systems::bullet_system::BulletSystem;
use crate::systems::canon_system::CanonSystem;
use crate::systems::collision_system::CollisionSystem;
use crate::systems::doors::plasma_door_system::PlasmaDoorSystem;
use crate::systems::explosion_systems::ExplosionSystem;
use crate::systems::landing_system::LandingSystem;
use crate::systems::menu_background_system::MenuBackgroundSystem;
use crate::systems::score_system::ScoreSystem;
use crate::systems::ship_systems::ShipSystem;
use crate::systems::thruster_system::ThrustersSystem;
use crate::systems::ui_system::UISystem;
use crate::utils::sound::Sounds;
use amethyst::audio::{AudioBundle, DjSystem, DjSystemDesc};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::renderer::palette::Srgba;
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
                    RenderToWindow::from_config_path(display_config)?.with_clear([r, g, b, a]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(AudioBundle::default())?
        .with(
            DjSystem::new(|music: &mut Sounds| music.menu_music.next()),
            "dj",
            &[],
        )
        .with(
            ShipSystem::default().pausable(CurrentState::Level),
            "ship_system",
            &["input_system"],
        )
        .with(
            CollisionSystem.pausable(CurrentState::Level),
            "collision_system",
            &[],
        )
        .with(
            ThrustersSystem.pausable(CurrentState::Level),
            "thrusters_system",
            &[],
        )
        .with(
            LandingSystem.pausable(CurrentState::Level),
            "landing_system",
            &[],
        )
        .with(
            ExplosionSystem::new().pausable(CurrentState::Level),
            "explosion_system",
            &[],
        )
        .with(
            CanonSystem::default().pausable(CurrentState::Level),
            "canon_system",
            &[],
        )
        .with(
            BulletSystem::default().pausable(CurrentState::Level),
            "bullet_system",
            &[],
        )
        .with(
            PlasmaDoorSystem::default().pausable(CurrentState::Level),
            "plasma_door_system",
            &[],
        )
        .with(UISystem.pausable(CurrentState::Level), "ui_system", &[])
        .with(
            MenuBackgroundSystem::default().pausable(CurrentState::MainMenu),
            "menu_background_system",
            &[],
        )
        .with(
            ScoreSystem::default().pausable(CurrentState::Level),
            "score_system",
            &[],
        )
        .with(
            BladeSawSystem.pausable(CurrentState::Level),
            "blade_saw_system",
            &[],
        )
        .with(
            BonusSystem::default().pausable(CurrentState::Level),
            "bonus_system",
            &[],
        );

    let mut game = Application::build(resources, MainMenuState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .build(game_data)?;
    game.run();
    Ok(())
}
