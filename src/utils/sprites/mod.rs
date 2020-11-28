use amethyst::core::ecs::{World, WorldExt};
use amethyst::assets::{Handle, Loader, AssetStorage};
use amethyst::renderer::{SpriteSheet, Texture, ImageFormat, SpriteSheetFormat};

pub mod sprite_to_entities;
pub mod plasma_doors;

pub const SCREEN_HEIGHT: f32 = 576.0;
pub const SCREEN_WIDTH: f32 = 704.0;
pub const NO_TILE_ID: i32 = -1;
pub const TILE_SIZE: f32 = 32.0;

const IMAGE_MISC: &str = "sprites/main.png";
const CONFIG_MISC: &str = "sprites/main.ron";

const IMAGE_SHIP: &str = "sprites/space_ship.png";
const CONFIG_SHIP_THRUSTERS: &str = "sprites/space_ship_with_thrusters.ron";
const CONFIG_SHIP: &str = "sprites/space_ship.ron";

const IMAGE_BULLETS: &str = "sprites/bullets.png";
const CONFIG_BULLETS: &str = "sprites/bullets.ron";

const IMAGE_POWER: &str = "sprites/power.png";
const CONFIG_POWER: &str = "sprites/power.ron";

const IMAGE_NUMBERS: &str = "sprites/numbers.png";
const CONFIG_NUMBERS: &str = "sprites/numbers.ron";

const IMAGE_EXPLOSION: &str = "sprites/explosion.png";
const CONFIG_EXPLOSION: &str = "sprites/explosion.ron";

const IMAGE_PUSH_ENTER: &str = "sprites/push_enter.png";
const CONFIG_PUSH_ENTER: &str = "sprites/push_enter.ron";

const IMAGE_MENU: &str = "sprites/menu.png";
const CONFIG_MENU: &str = "sprites/menu.ron";

const IMAGE_BACKGROUND: &str = "sprites/background.png";
const IMAGE_BACKGROUND2: &str = "sprites/background2.png";
const IMAGE_BACKGROUND3: &str = "sprites/background3.png";
const CONFIG_BACKGROUND: &str = "sprites/background.ron";


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

pub fn load_misc_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_MISC, CONFIG_MISC)
}

pub fn load_power_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_POWER, CONFIG_POWER)
}

pub fn load_ship_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_SHIP, CONFIG_SHIP)
}

pub fn load_ship_thrusters_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_SHIP, CONFIG_SHIP_THRUSTERS)
}

pub fn load_bullets_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_BULLETS, CONFIG_BULLETS)
}

pub fn load_menu_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_MENU, CONFIG_MENU)
}

pub fn load_explosion_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_EXPLOSION, CONFIG_EXPLOSION)
}

pub fn load_numbers_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_NUMBERS, CONFIG_NUMBERS)
}

pub fn load_background(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_BACKGROUND, CONFIG_BACKGROUND)
}

pub fn load_background_2(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_BACKGROUND2, CONFIG_BACKGROUND)
}

pub fn load_background_3(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_BACKGROUND3, CONFIG_BACKGROUND)
}

pub fn load_push_enter(world: &mut World) -> Handle<SpriteSheet> {
    load_texture(world, IMAGE_PUSH_ENTER, CONFIG_PUSH_ENTER)
}