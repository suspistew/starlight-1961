use amethyst::{SimpleState, StateData, GameData, SimpleTrans, Trans, StateEvent};
use amethyst::ui::{UiCreator, UiTransform, Anchor, UiImage, ScaleMode};
use crate::utils::sprites::{load_background, SCREEN_WIDTH, SCREEN_HEIGHT, load_background_2, load_background_3, load_ship_thrusters_spritesheet, load_push_enter};
use amethyst::renderer::{SpriteSheet, SpriteRender, Camera};
use amethyst::assets::Handle;
use amethyst::core::Transform;
use amethyst::core::ecs::{World, WorldExt, Builder};
use crate::entities::main_menu::{MenuBackground, PushEnter};
use crate::entities::ship::Ship;
use crate::states::level::LevelState;
use amethyst::input::{is_key_down, VirtualKeyCode};

pub struct MainMenuState;

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
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
        add_push_enter_text(world);

        initialize_camera(world);

    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                return Trans::Switch(Box::new(LevelState));
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }
}

pub fn add_animated_backgrounds(world: &mut World, background: Handle<SpriteSheet>, parallax: usize){
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
        .with(MenuBackground{ parallax_index: parallax })
        .build();
    world
        .create_entity()
        .with(sprite_render_2)
        .with(t)
        .with(MenuBackground{ parallax_index: parallax })
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

fn add_ship (world: &mut World, ship: Handle<SpriteSheet> ){
    let ship_sprite_render = SpriteRender {
        sprite_sheet: ship,
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(
        0.,
        SCREEN_HEIGHT / -2. + 70.,
        0.04
    );
    world
        .create_entity()
        .with(transform)
        .with(ship_sprite_render)
        .with(Ship)
        .build();

}

fn add_push_enter_text(world: &mut World) {

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
    transform.scale_mode= ScaleMode::Percent;
    world
        .create_entity()
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: push_enter,
            sprite_number: 0,
        }))
        .with(transform)
        .with(PushEnter)
        .build();
}