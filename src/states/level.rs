use amethyst::{SimpleState, StateData, GameData};
use amethyst::core::ecs::{World, WorldExt, Builder, Entity};
use amethyst::renderer::{SpriteSheet, SpriteRender, ImageFormat, Texture, SpriteSheetFormat, Camera};
use std::io::BufRead;
use amethyst::core::{Transform, Parent};
use amethyst::assets::{Handle, Loader, AssetStorage};

use std::fs::File;
use serde_json::from_reader;
use serde::Deserialize;
use std::collections::HashMap;
use crate::entities::ship::{Ship, ShipParent};

pub const SCREEN_HEIGHT: f32 = 576.0;
pub const SCREEN_WIDTH: f32 = 704.0;
pub const NO_TILE_ID: i32 = -1;
pub const TILE_SIZE: f32 = 32.0;

const IMAGE_MISC: &str = "sprites/main.png";
const CONFIG_MISC: &str = "sprites/main.ron";

const IMAGE_SHIP: &str = "sprites/space_ship.png";
const CONFIG_SHIP: &str = "sprites/space_ship.ron";

pub struct LevelState;

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let level = read_level(1);
        let world = data.world;
        let misc_spritesheet_handle = load_misc_spritesheet(world);
        let ship_spritesheet_handle = load_ship_spritesheet(world);

        initialize_layer(world, &level, misc_spritesheet_handle.clone(), "background", 0.01);
        initialize_layer(world, &level, misc_spritesheet_handle.clone(), "structures", 0.05);
        initialize_layer(world, &level, misc_spritesheet_handle, "entities", 0.03);

        let ship = initialize_ship(world, &level, ship_spritesheet_handle);

        initialize_camera(world, &level, ship);
    }
}

pub fn load_misc_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_MISC, CONFIG_MISC)
}

pub fn load_ship_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_SHIP, CONFIG_SHIP)
}

fn load_texture(world: &mut World, image: &str, config: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let asset_loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        asset_loader.load(image, ImageFormat::default(), (), &texture_storage)
    };

    let asset_loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    asset_loader.load(
        config,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_layer(
    world: &mut World,
    level: &LevelConfig,
    sprite_sheet_handle: Handle<SpriteSheet>,
    layer: &str,
    layer_position: f32,
) {
    match level.layers.get(layer) {
        Some(sprites) => {
            let lines: Vec<_> = sprites.split(',').collect();
            for y in 0..level.height {
                let line = &lines[((y * level.width) as usize)..((y * level.width + level.width) as usize)];
                for (x, tile) in line.iter().enumerate() {
                    let (tile_x, tile_y) = (x, level.height - y - 1);

                    let tile_number: i32 = match tile.trim().parse() {
                        Ok(num) => num,
                        Err(_) => NO_TILE_ID,
                    };

                    if tile_number != NO_TILE_ID && tile_number > 0 {
                        let sprite_render = SpriteRender {
                            sprite_sheet: sprite_sheet_handle.clone(),
                            sprite_number: (tile_number - 1) as usize,
                        };

                        let mut transform = Transform::default();
                        transform.set_translation_xyz(
                            tile_x as f32 * TILE_SIZE,
                            tile_y as f32 * TILE_SIZE,
                            layer_position,
                        );
                        world
                            .create_entity()
                            .with(sprite_render)
                            .with(transform)
                            .build();
                    }
                }
            }
        }
        None => {
            println!("Impossible to find the layer {} in the level config", layer);
        }
    };
}

fn initialize_ship(
    world: &mut World,
    level: &LevelConfig,
    sprite_sheet_handle: Handle<SpriteSheet>,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(
        level.start_x as f32 * TILE_SIZE,
        ((level.height - level.start_y) as f32 * TILE_SIZE) - 26.,
        0.04,
    );

    let parent = world
        .create_entity()
        .with(ShipParent)
        .with(transform)
        .build();
    let transform_ship = Transform::default();
    world
        .create_entity()
        .with(sprite_render)
        .with(Ship::new(0., 0., 0., level.start_x as f32 * TILE_SIZE, ((level.height - level.start_y) as f32 * TILE_SIZE) - 26.))
        .with(transform_ship)
        .with(Parent { entity: parent })
        .build();
    parent
}

pub fn initialize_camera(world: &mut World, _level_config: &LevelConfig, ship: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    //transform.set_translation_x((level_config.start_x as f32 * TILE_SIZE));
    //transform.set_translation_y((level_config.height - level_config.start_y) as f32 * TILE_SIZE);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .with(Parent { entity: ship })
        .build();
}

fn read_level(lvl_number: usize) -> LevelConfig {
    let input_path = format!(
        "{}/assets/levels/level_{}.json",
        env!("CARGO_MANIFEST_DIR"),
        lvl_number
    );
    let f = File::open(&input_path).expect("Failed opening file");
    let res: LevelConfig = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load level {}: {}", lvl_number, e);
            std::process::exit(1);
        }
    };
    res
}

#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub height: u16,
    pub width: u16,
    pub start_x: u16,
    pub start_y: u16,
    pub layers: HashMap<String, String>,
}