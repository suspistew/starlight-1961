use crate::states::level_state::LevelState;
use crate::states::main_menu_state::{add_ship};
use crate::states::CurrentState;
use crate::utils::level_reader::{read_level, LevelConfig};
use crate::utils::save::save_progress;
use crate::utils::sprites::{
    load_ship_thrusters_spritesheet,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use amethyst::assets::Loader;
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};

pub struct NextLevelState {
    pub next_level_nb: usize,
    frame_counter: f32,
}
impl NextLevelState {
    pub fn new(next_level_nb: usize) -> Self {
        NextLevelState {
            next_level_nb,
            frame_counter: 120.,
        }
    }
}

impl SimpleState for NextLevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::MainMenu;
        if self.next_level_nb > 0 {
            save_progress(self.next_level_nb);
        }
        let level = read_level(self.next_level_nb);
        let world = data.world;
        initialise_camera(world);
        initialise_texts(world, self.next_level_nb, level);
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.frame_counter <= 0. {
            return Trans::Switch(Box::new(LevelState {
                level_nb: self.next_level_nb,
            }));
        }
        self.frame_counter -= 1.;
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
        *data.world.write_resource::<CurrentState>() = CurrentState::NextLevel;
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.1);
    let ship = load_ship_thrusters_spritesheet(world);
    add_ship(world, ship);
    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_texts(world: &mut World, lvl_number: usize, config: LevelConfig) {
    let font = world.read_resource::<Loader>().load(
        "fonts/pixel.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let level_nb_transform = UiTransform::new(
        "level".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        200.,
        1.,
        400.,
        300.,
    );
    let level_text_transform = UiTransform::new(
        "level".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        -100.,
        1.,
        600.,
        400.,
    );
    let text = format!("Level {}", lvl_number);
    world
        .create_entity()
        .with(level_nb_transform)
        .with(UiText::new(
            font.clone(),
            text,
            [1., 1., 1., 1.],
            70.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();
    world
        .create_entity()
        .with(level_text_transform)
        .with(UiText::new(
            font,
            config.text,
            [1., 1., 1., 1.],
            50.,
            LineMode::Wrap,
            Anchor::Middle,
        ))
        .build();
}
