use amethyst::{SimpleState, StateData, GameData};
use amethyst::core::ecs::{World, WorldExt, Builder, Entity};
use amethyst::renderer::{SpriteSheet, SpriteRender, ImageFormat, Texture, SpriteSheetFormat, Camera};
use std::io::BufRead;
use amethyst::core::{Transform, Parent};
use amethyst::assets::{Handle, PrefabLoader, RonFormat, Loader, AssetStorage};

use std::fs::File;
use serde_json::from_reader;
use serde::Deserialize;
use std::collections::HashMap;
use crate::entities::ship::{Ship, ShipParent, Thrusters};
use crate::resources::main_resource::{MainResource, MainSprites};
use crate::utils::sprite_to_entities::{sprite_to_colliders, is_landing_platform_start, sprite_to_canon};
use crate::entities::collision::{Transparent, LandingPlatform};
use amethyst::utils::application_root_dir;
use amethyst::core::math::{Point3, Vector3};
use amethyst::core::ecs::hibitset::BitSetLike;
use crate::utils::starlight_tile::StartLightTile;
use amethyst_tiles::{TileMap, MortonEncoder2D};
use std::borrow::Borrow;

pub const SCREEN_HEIGHT: f32 = 576.0;
pub const SCREEN_WIDTH: f32 = 704.0;
pub const NO_TILE_ID: i32 = -1;
pub const TILE_SIZE: f32 = 32.0;

const IMAGE_MISC: &str = "sprites/main.png";
const CONFIG_MISC: &str = "sprites/main.ron";

const IMAGE_SHIP: &str = "sprites/space_ship.png";
const CONFIG_SHIP: &str = "sprites/space_ship.ron";

const IMAGE_BULLETS: &str = "sprites/bullets.png";
const CONFIG_BULLETS: &str = "sprites/bullets.ron";

const IMAGE_EXPLOSION: &str = "sprites/explosion.png";
const CONFIG_EXPLOSION: &str = "sprites/explosion.ron";

pub struct LevelState;

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let level = read_level(1);
        let world = data.world;
        //let misc_spritesheet_handle = load_misc_spritesheet(world);
        let level_spritesheet_handle = load_level_spritesheet(world, 1);
        let ship_spritesheet_handle = load_ship_spritesheet(world);
        let bullet_spritesheet_handle = load_bullets_spritesheet(world);
        let ship_explosion_handle = load_explosion_spritesheet(world);

        initialize_level_tileset(world, level_spritesheet_handle, &level);
        initialize_colliders_with_entitites(world, &level);
        let ship = initialize_ship(world, &level, ship_spritesheet_handle);
        initialize_camera(world, &level, ship);
        let mut ship_resource = MainResource::new_from_level(Some(level));
        ship_resource.sprites = Some(MainSprites { explosion_sprite_render: ship_explosion_handle, bullet_sprite_render: bullet_spritesheet_handle });
        world.insert(ship_resource);
    }
}

pub fn load_misc_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_MISC, CONFIG_MISC)
}

pub fn load_ship_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_SHIP, CONFIG_SHIP)
}

pub fn load_bullets_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_BULLETS, CONFIG_BULLETS)
}

pub fn load_explosion_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_EXPLOSION, CONFIG_EXPLOSION)
}

pub fn load_level_spritesheet(world: &mut World, lvl_number: usize) -> Handle<SpriteSheet> {
    let image = format!(
        "levels/level_{}.png",
        lvl_number
    );
    let config = format!(
        "levels/level_{}.ron",
        lvl_number
    );
    load_texture(world, image.as_str(), config.as_str())
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

fn initialize_colliders_with_entitites(world: &mut World, level: &LevelConfig) {
    for (point, sprite) in level.tiles.borrow() {
        let collider = sprite_to_colliders(*sprite, point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE);
        if collider.is_some() {
            let mut builder = world
                .create_entity()
                .with(collider.unwrap());

            if is_landing_platform_start(*sprite) { builder = builder.with(LandingPlatform); }
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
        ((level.height - level.start_y) as f32 * TILE_SIZE),
        0.04,
    );

    let parent = world
        .create_entity()
        .with(ShipParent)
        .with(transform)
        .build();
    let mut transform_ship = Transform::default();
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

pub fn initialize_camera(world: &mut World, level_config: &LevelConfig, ship: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., -100., 1.1);
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
        "Background" => 2,
        _ => 99
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