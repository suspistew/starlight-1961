use crate::states::CurrentState;
use crate::utils::sprites::{SCREEN_HEIGHT, SCREEN_WIDTH};
use amethyst::assets::Loader;
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::{GameData, SimpleState, StateData};

pub struct EndLevelState;

impl SimpleState for EndLevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::End;
        let world = data.world;
        initialise_camera(world);
        initialise_texts(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.1);
    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_texts(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/pixel.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let end_transform = UiTransform::new(
        "end".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        200.,
        1.,
        704.,
        400.,
    );
    world
        .create_entity()
        .with(end_transform)
        .with(UiText::new(
            font,
            "Congratulation !".to_string(),
            [1., 1., 1., 1.],
            80.,
            LineMode::Wrap,
            Anchor::Middle,
        ))
        .build();
}
