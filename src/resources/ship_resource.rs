use rand::Rng;
use crate::states::level::{LevelConfig};
use crate::entities::collision::{Colliders, Collider};
use crate::utils::Point2D;
use geo::Polygon;

pub struct ShipResource {
    pub x_force: f32,
    pub y_force: f32,
    pub current_rotation_angle: f32,
    pub power: usize,
    pub last_sprite: usize,
    pub is_landed: bool,
    gravity: f32,
}

impl ShipResource {
    fn new(x_force: f32, y_force: f32, gravity: f32) -> ShipResource {
        ShipResource {
            x_force,
            y_force,
            gravity,
            current_rotation_angle: 0.,
            power: 0,
            last_sprite: 0,
            is_landed: true
        }
    }

    pub fn new_from_level(_config: &LevelConfig) -> ShipResource {
        ShipResource::new (0., 0., 1.0)
    }

    pub fn init_resource_from_level(&mut self, _config: LevelConfig) {
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

    pub fn sprite_nb(&mut self) -> usize {
        if self.power == 0 {self.last_sprite= 0;  return 0 }
        rand::thread_rng().gen_range(1, 4) as usize
    }
}

impl Default for ShipResource {
    fn default() -> Self {
        ShipResource::new(0., 0., 0.)
    }
}