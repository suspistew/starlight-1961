use amethyst::{SimpleState, StateData, GameData, Trans, SimpleTrans};
use amethyst::core::ecs::{World, WorldExt, Builder, Entity};
use amethyst::renderer::{SpriteSheet, SpriteRender, Camera};
use amethyst::core::{Transform, Parent};
use amethyst::assets::Handle;

use std::fs::File;
use serde_json::from_reader;
use serde::Deserialize;
use std::collections::HashMap;
use crate::entities::ship::{Ship, ShipParent, Thrusters, ShipPowerLeftNumber, ShipLife, ShipFuel, ShipPowerRightNumber};
use crate::resources::main_resource::{MainResource, MainSprites};
use crate::utils::sprites::sprite_to_entities::{sprite_to_colliders, is_landing_platform_start, sprite_to_canon, is_arrival};
use crate::entities::collision::{LandingPlatform, Arrival};
use amethyst::utils::application_root_dir;
use amethyst::core::math::Point3;
use std::borrow::Borrow;
use crate::utils::sprites::plasma_doors::is_plasma_door_part;
use crate::entities::doors::{PlasmaDoor, DoorState};
use amethyst::ui::{UiCreator, UiTransform, Anchor, UiImage, ScaleMode};
use crate::utils::sprites::*;
use crate::states::CurrentState;
use crate::states::next_level::NextLevelState;
use crate::states::end_state::EndLevelState;

pub struct LevelState{
    pub level_nb: usize
}

const MAX_LVL: usize = 2;

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        *world.write_resource::<CurrentState>() = CurrentState::Level;
        load_level(self.level_nb, world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
        *data.world.write_resource::<CurrentState>() = CurrentState::Level;
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world;
        let (victory, current_level) = {
            let resource = world.read_resource::<MainResource>();
            (resource.victory, resource.current_level)
        };
        if victory {
            let new_level =current_level + 1;
            if new_level <= MAX_LVL {
                return Trans::Switch(Box::new(NextLevelState{ next_level_nb: new_level }));
            }else{
                return Trans::Switch(Box::new(EndLevelState));
            }
        }
        return Trans::None;
    }
}

fn load_level(lvl_number: usize, world: &mut World) {
    let level = read_level(lvl_number);
    let misc_spritesheet_handle = load_misc_spritesheet(world);
    let level_spritesheet_handle = load_level_spritesheet(world, 1);
    let ship_spritesheet_handle = load_ship_spritesheet(world);
    let bullet_spritesheet_handle = load_bullets_spritesheet(world);
    let ship_explosion_handle = load_explosion_spritesheet(world);
    let numbers_spritesheet_handle = load_numbers_spritesheet(world);

    initialize_level_tileset(world, level_spritesheet_handle, &level);
    initialize_colliders_with_entitites(world, &level, misc_spritesheet_handle);
    let ship = initialize_ship(world, &level, ship_spritesheet_handle);
    initialize_camera(world, ship);
    let mut ship_resource = MainResource::new_from_level(Some(level), lvl_number);
    ship_resource.sprites = Some(MainSprites { explosion_sprite_render: ship_explosion_handle, bullet_sprite_render: bullet_spritesheet_handle });
    world.insert(ship_resource);
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create("ui/ui.ron", ());
    });
    initialise_level_ui(world, numbers_spritesheet_handle.clone());
    initialise_power_ui(world, numbers_spritesheet_handle);
    initialise_life_and_fuel_ui(world);
}

fn initialize_colliders_with_entitites(world: &mut World, level: &LevelConfig, sprite_sheet_handle: Handle<SpriteSheet>) {
    for (point, sprite) in level.tiles.borrow() {
        let collider = sprite_to_colliders(*sprite, point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE);
        if collider.is_some() {
            let mut builder = world
                .create_entity()
                .with(collider.unwrap());
            if is_plasma_door_part(*sprite) {
                let mut transform = Transform::default();
                transform.set_translation_xyz(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE,0.6);
                builder = builder.with(PlasmaDoor{ initial_sprite: *sprite, state: DoorState::Closed }).with(SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: *sprite,
                }).with(transform);
            }
            if is_landing_platform_start(*sprite) { builder = builder.with(LandingPlatform); }
            if is_arrival(*sprite) { builder = builder.with(Arrival); }
            if let Some(canon) = sprite_to_canon(*sprite, point.x as usize, point.y as usize) {builder = builder.with(canon); }
            builder.build();
        }
    }
}

fn initialize_level_tileset(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    level: &LevelConfig
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    let mut t = Transform::default();
    t.set_translation_xyz( (TILE_SIZE * level.width as f32) / 2. - TILE_SIZE / 2., (TILE_SIZE * level.height as f32) / 2. - TILE_SIZE / 2., 0.);
    world
        .create_entity()
        .with(sprite_render)
        .with(t)
        .build();
}

fn initialize_ship(
    world: &mut World,
    level: &LevelConfig,
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> Entity {
    let ship_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let thrusters_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 5,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(
        level.start_x as f32 * TILE_SIZE - 16.,
        (level.height - level.start_y) as f32 * TILE_SIZE,
        0.04,
    );

    let parent = world
        .create_entity()
        .with(ShipParent)
        .with(transform)
        .build();
    let transform_ship = Transform::default();
    let ship = world
        .create_entity()
        .with(ship_sprite_render)
        .with(Ship)
        .with(transform_ship)
        .with(Parent { entity: parent })
        .build();

    let mut transform_thruster = Transform::default();
    transform_thruster.set_translation_xyz(0., -32., 0.);
    world
        .create_entity()
        .with(thrusters_sprite_render)
        .with(Thrusters)
        .with(transform_thruster)
        .with(Parent { entity: ship })
        .build();

    parent
}

pub fn initialize_camera(world: &mut World, ship: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.1);
    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .with(Parent { entity: ship })
        .build();
}

fn read_level(lvl_number: usize) -> LevelConfig {
    let input_path = format!(
        "assets/levels/level_{}.json",
        lvl_number
    );
    let app_root = application_root_dir().unwrap();
    let input_path = app_root.join(input_path.as_str());
    let f = File::open(&input_path.as_path()).expect("Failed opening file");
    let res: TiledLevel = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load level {}: {}", lvl_number, e);
            std::process::exit(1);
        }
    };

    LevelConfig::new(res)
}

#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub height: u32,
    pub width: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub tiles: HashMap<Point3<u32>,usize>
}

impl LevelConfig {
    fn new(level: TiledLevel) -> Self {
        let mut tiles: HashMap<Point3<u32>,usize> = HashMap::new();
        for layer in level.layers {
            let z = get_z_from_layer_name(layer.name.as_str());
            for y in 0..level.height {
                let line = &layer.data[((y * level.width) as usize)..((y * level.width + level.width) as usize)];
                for (x, tile) in line.iter().enumerate() {
                    let (tile_x, tile_y, tile_z) = (x as u32, (level.height - y - 1), z as u32);
                    let tile_number: i32 = match *tile {
                        0 => NO_TILE_ID,
                        any => (any as i32),
                    };
                    if tile_number != NO_TILE_ID && tile_number > 0 {
                        tiles.insert(Point3::new(tile_x, tile_y, tile_z), (tile_number - 1) as usize);
                    }
                }
            }
        }

        LevelConfig{
            height: level.height,
            width: level.width,
            start_x: level.start_x,
            start_y: level.start_y,
            tiles
        }
    }
}

fn get_z_from_layer_name(name: &str) -> usize{
    match name {
        "Structures" => 0,
        "Entities" => 1,
        "Interactives" => 2,
        "Background" => 3,
        _ => 99
    }
}

fn initialise_power_ui(world: &mut World, spritesheet: Handle<SpriteSheet>) {
    let mut power_nb_left_transform = UiTransform::new(
        "power_left".to_string(),
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        0.82954545454,
        0.02056944444,
        10.,
        0.01420454545,
        0.02430555555,
    );
    power_nb_left_transform.scale_mode = ScaleMode::Percent;

    world
        .create_entity()
        .with(power_nb_left_transform)
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: spritesheet.clone(),
            sprite_number: 0,
        })
        )
        .with(ShipPowerLeftNumber)
        .build();

    let mut power_nb_right_transform = UiTransform::new(
        "power_right".to_string(),
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        0.84754545454,
        0.02056944444,
        10.,
        0.01420454545,
        0.02430555555,
    );
    power_nb_right_transform.scale_mode = ScaleMode::Percent;

    world
        .create_entity()
        .with(power_nb_right_transform)
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: spritesheet.clone(),
            sprite_number: 0,
        })
        )
        .with(ShipPowerRightNumber)
        .build();
}

fn initialise_level_ui(world: &mut World, spritesheet: Handle<SpriteSheet>) {

    let mut level_nb_left_transform = UiTransform::new(
        "level_left".to_string(),
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        0.64488636363,
        0.02056944444,
        10.,
        0.01420454545,
        0.02430555555,
    );
    level_nb_left_transform.scale_mode = ScaleMode::Percent;

    world
        .create_entity()
        .with(level_nb_left_transform)
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: spritesheet.clone(),
            sprite_number: 0,
        })
        )
        .build();


    let mut level_nb_right_transform = UiTransform::new(
        "level_right".to_string(),
        Anchor::BottomLeft,
        Anchor::BottomLeft,
        0.66088636363,
        0.02056944444,
        10.,
        0.01420454545,
        0.02430555555,
    );
    level_nb_right_transform.scale_mode = ScaleMode::Percent;

    world
        .create_entity()
        .with(level_nb_right_transform)
        .with(UiImage::Sprite(SpriteRender {
            sprite_sheet: spritesheet.clone(),
            sprite_number: 1,
        })
        )
        .build();
}

fn initialise_life_and_fuel_ui(world: &mut World) {
    let power_spritesheet_handle = load_power_spritesheet(world);
    for life_point in 0..3 {
        let mut life_point_transform = UiTransform::new(
            format!("life_{}", life_point.to_string()),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            0.17755681818 + (life_point as f32 * (0.00426136363 + 0.01420454545 )),
            0.01215277777,
            10.,
            0.01420454545,
            0.04166666666,
        );
        life_point_transform.scale_mode = ScaleMode::Percent;
        world
            .create_entity()
            .with(life_point_transform)
            .with(ShipLife{ life_point: life_point as u8 + 1 })
            .with(UiImage::Sprite(SpriteRender {
                sprite_sheet: power_spritesheet_handle.clone(),
                sprite_number: 0,
            }))
            .build();
    }

    for fuel_point in 0..10 {
        let mut fuel_point_transform = UiTransform::new(
            format!("fuel_{}", fuel_point.to_string()),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            0.32244318181 + (fuel_point as f32 * (0.00426136363 + 0.01420454545 )),
            0.01215277777,
            10.,
            0.01420454545,
            0.04166666666,
        );
        fuel_point_transform.scale_mode = ScaleMode::Percent;
        world
            .create_entity()
            .with(fuel_point_transform)
            .with(ShipFuel{ fuel_point: fuel_point as u8 })
            .with(UiImage::Sprite(SpriteRender {
                sprite_sheet: power_spritesheet_handle.clone(),
                sprite_number: 0,
            }))
            .build();
    }
}

#[derive(Debug, Deserialize)]
pub struct TiledLevel {
    pub height: u32,
    pub width: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub layers: Vec<TiledLayer>,
}

#[derive(Debug, Deserialize)]
pub struct TiledLayer{
    data: Vec<usize>,
    name: String,
}