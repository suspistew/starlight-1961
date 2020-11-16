use rand::Rng;
use crate::states::level::{LevelConfig, TILE_SIZE};
use crate::entities::collision::{Colliders, Collider};
use crate::utils::Point2D;
use geo::Polygon;
use amethyst::core::math::UnitQuaternion;
use amethyst::core::alga::linear::Similarity;
use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;

pub struct MainResource {
    pub x_force: f32,
    pub y_force: f32,
    pub current_rotation_angle: f32,
    pub power: usize,
    pub is_landed: bool,
    pub is_exploding: bool,
    pub should_be_reset: bool,
    gravity: f32,
    current_level_config: Option<LevelConfig>,
    pub sprites: Option<ShipSprites>

}

pub struct ShipSprites {
    pub explosion_sprite_render: Handle<SpriteSheet>
}

impl MainResource {
    fn new(x_force: f32, y_force: f32, gravity: f32, current_level_config: Option<LevelConfig>) -> MainResource {
        MainResource {
            x_force,
            y_force,
            gravity,
            current_rotation_angle: 0.,
            power: 0,
            is_landed: true,
            is_exploding: false,
            should_be_reset: false,
            current_level_config,
            sprites: None
        }
    }

    pub fn level_config(&self) -> &LevelConfig {
        &(self.current_level_config.as_ref().unwrap())
    }

    pub fn new_from_level(config: Option<LevelConfig>) -> MainResource {
        MainResource::new (0., 0., 1.0, config)
    }

    pub fn reset(&mut self) {
        self.is_landed = true;
        self.y_force = 0.;
        self.x_force = 0.;
        self.power = 0;
        self.current_rotation_angle = 0.;
        self.is_exploding = false;
        self.should_be_reset = false;
    }

    pub fn power(&mut self, rotation: &UnitQuaternion<f32>) {
        self.is_landed = false;
        self.y_force += calculate_y_force(rotation.rotation().quaternion().k);
        self.x_force += calculate_x_force(rotation.rotation().quaternion().k);
        self.power += 1;
    }

    pub fn apply_gravity(&mut self) {
        if self.is_landed {return;}
        self.y_force -= 0.02;
        // TODO : Add x force lose
        self.power = 0;
    }



    pub fn init_resource_from_level(&mut self, config: LevelConfig) {
        self.x_force= 0.;
        self.y_force = 0.;
        self.gravity = 1.0;
        self.current_rotation_angle = 0.;
    }

    pub fn get_colliders_polygons(&self, x: f32, y:f32) -> Vec<Polygon<f32>> {
        // println!("x:y {}:{}", x/ 32., y/32.);
        let main_collider = Collider::new(Point2D{x, y}, 32., -32.);
        // println!("collider : {:?}", main_collider);
        let colliders = Colliders::from_vec(vec![main_collider]);
        colliders.to_owned_polygons()
    }

    pub fn sprite_nb(&self) -> usize {
        if self.power == 0 {
            0
        } else {
            rand::thread_rng().gen_range(1, 4) as usize
        }
    }
}

impl Default for MainResource {
    fn default() -> Self {
        MainResource::new(0., 0., 0., None)
    }
}


fn calculate_y_force(z_rotation: f32) -> f32 {
    0.02 * ((0.75 - (z_rotation.abs())) / 0.75)
}

fn calculate_x_force(z_rotation: f32) -> f32 {
    -0.05 * ((z_rotation) / 0.50)
}