use crate::entities::main_menu::{MenuBackground, PushEnter};
use crate::entities::ship::Ship;
use crate::entities::sound::MenuSound;
use crate::states::next_level::NextLevelState;
use crate::states::CurrentState;
use crate::utils::save::{read_saved_level, StarlightSave};
use crate::utils::sound::initialise_audio;
use crate::utils::sprites::{
    load_background, load_background_2, load_background_3, load_menu_spritesheet, load_push_enter,
    load_ship_thrusters_spritesheet, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use amethyst::assets::Handle;
use amethyst::core::ecs::{Builder, Entity, World, WorldExt};
use amethyst::core::Transform;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::renderer::{Camera, SpriteRender, SpriteSheet};
use amethyst::ui::{Anchor, ScaleMode, UiCreator, UiImage, UiTransform};
use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};

pub struct MainMenuState {
    enter_pressed: bool,
    enter_press_entity: Option<Entity>,
    saved_level_progress: Option<StarlightSave>,
    arrow: Option<Entity>,
    menu_position: usize,
}

impl Default for MainMenuState {
    fn default() -> Self {
        MainMenuState {
            enter_pressed: false,
            enter_press_entity: None,
            saved_level_progress: None,
            arrow: None,
            menu_position: 0,
        }
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.saved_level_progress = read_saved_level();
        let world = data.world;
        *world.write_resource::<CurrentState>() = CurrentState::MainMenu;
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/title.ron", ());
        });

        let background = load_background(world);
        let background2 = load_background_2(world);
        let background3 = load_background_3(world);
        let ship = load_ship_thrusters_spritesheet(world);
        add_animated_backgrounds(world, background, 3);
        add_animated_backgrounds(world, background2, 2);
        add_animated_backgrounds(world, background3, 1);

        add_ship(world, ship);
        self.enter_press_entity = Some(add_push_enter_text(world));

        initialize_camera(world);
        initialise_audio(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                if !self.enter_pressed {
                    data.world.create_entity().with(MenuSound).build();
                    self.enter_pressed = true;
                    data.world.delete_entity(self.enter_press_entity.unwrap());
                    self.arrow = Some(add_new_game_continue(
                        data.world,
                        &self.saved_level_progress,
                    ));
                } else if self.saved_level_progress.is_some() && self.menu_position == 0 {
                    data.world.insert(MenuSound);
                    return Trans::Switch(Box::new(NextLevelState::new(
                        self.saved_level_progress.as_ref().unwrap().save,
                    )));
                } else {
                    data.world.create_entity().with(MenuSound).build();
                    return Trans::Switch(Box::new(NextLevelState::new(0)));
                }
            } else if is_key_down(&event, VirtualKeyCode::Up) {
                if self.saved_level_progress.is_some() && self.menu_position == 1 {
                    data.world.create_entity().with(MenuSound).build();
                    data.world.delete_entity(self.arrow.unwrap());
                    self.menu_position = 0;
                    self.arrow = Some(init_arrow(data.world, 0));
                }
            } else if is_key_down(&event, VirtualKeyCode::Down) {
                data.world.create_entity().with(MenuSound).build();
                if self.saved_level_progress.is_some() && self.menu_position == 0 {
                    data.world.delete_entity(self.arrow.unwrap());
                    self.menu_position = 1;
                    self.arrow = Some(init_arrow(data.world, 1));
                }
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
        *data.world.write_resource::<CurrentState>() = CurrentState::MainMenu;
    }
}

pub fn add_animated_backgrounds(
    world: &mut World,
    background: Handle<SpriteSheet>,
    parallax: usize,
) {
    let sprite_render_1 = SpriteRender {
        sprite_sheet: background.clone(),
        sprite_number: 0,
    };
    let sprite_render_2 = SpriteRender {
        sprite_sheet: background.clone(),
        sprite_number: 0,
    };
    let mut t = Transform::default();
    t.set_translation_xyz(0., 800., 0.);
    world
        .create_entity()
        .with(sprite_render_1)
        .with(Transform::default())
        .with(MenuBackground {
            parallax_index: parallax,
        })
        .build();
    world
        .create_entity()
        .with(sprite_render_2)
        .with(t)
        .with(MenuBackground {
            parallax_index: parallax,
        })
        .build();
}

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.1);
    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

pub fn add_ship(world: &mut World, ship: Handle<SpriteSheet>) {
    let ship_sprite_render = SpriteRender {
        sprite_sheet: ship,
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., SCREEN_HEIGHT / -2. + 70., 0.04);
    world
        .create_entity()
        .with(transform)
        .with(ship_sprite_render)
        .with(Ship)
        .build();
}

fn add_push_enter_text(world: &mut World) -> Entity {
    let push_enter = load_push_enter(world);
    let mut transform = UiTransform::new(
        "level".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        10.,
        1.,
        0.09722222222,
    );
    transform.scale_mode = ScaleMode::Percent;
    world
        .create_entity()
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: push_enter,
            sprite_number: 0,
        }))
        .with(transform)
        .with(PushEnter)
        .build()
}

fn add_new_game_continue(world: &mut World, save: &Option<StarlightSave>) -> Entity {
    let sprite = load_menu_spritesheet(world);
    let shift = if save.is_some() { 0.12027777777 } else { 0. };
    let mut transform = UiTransform::new(
        "new_game".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0. - shift,
        10.,
        0.2713068182,
        0.09027777777,
    );
    transform.scale_mode = ScaleMode::Percent;
    world
        .create_entity()
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: sprite.clone(),
            sprite_number: 0,
        }))
        .with(transform)
        .build();

    if save.is_some() {
        let mut transform_continue = UiTransform::new(
            "continue".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            10.,
            0.2713068182,
            0.09027777777,
        );
        transform_continue.scale_mode = ScaleMode::Percent;
        world
            .create_entity()
            .with(UiImage::Sprite(SpriteRender {
                sprite_sheet: sprite.clone(),
                sprite_number: 1,
            }))
            .with(transform_continue)
            .build();
    }

    init_arrow(world, 0)
}

fn init_arrow(world: &mut World, shif: usize) -> Entity {
    let sprite = load_menu_spritesheet(world);
    let mut transform_arrow = UiTransform::new(
        "arrow".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        -0.1568181,
        0. - shif as f32 * 0.12027777777,
        10.,
        0.05555555,
        0.05555555,
    );
    transform_arrow.scale_mode = ScaleMode::Percent;
    world
        .create_entity()
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: sprite,
            sprite_number: 2,
        }))
        .with(transform_arrow)
        .build()
}
