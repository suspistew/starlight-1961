use rand::Rng;
use crate::entities::collision::{Colliders, Collider};
use crate::utils::Point2D;
use geo::Polygon;
use amethyst::core::math::UnitQuaternion;
use amethyst::core::alga::linear::Similarity;
use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;
use core::cmp;
use crate::utils::level_reader::LevelConfig;

pub struct MainResource {
    pub x_force: f32,
    pub y_force: f32,
    pub current_rotation_angle: f32,
    pub power: usize,
    pub is_landed: bool,
    pub is_exploding: bool,
    pub should_be_reset: bool,
    pub victory: bool,
    _gravity: f32,
    current_level_config: Option<LevelConfig>,
    pub current_level: usize,
    pub sprites: Option<MainSprites>,
    pub ship_life: u8,
    pub ship_fuel: f32,
    pub should_reset_plasma_timers: bool,
    pub should_reset_bonuses: bool,
    pub bullet_hit_timer: f32,
    pub collected_coin: usize,
    pub should_go_to_next_level: bool
}

pub struct MainSprites {
    pub explosion_sprite_render: Handle<SpriteSheet>,
    pub bullet_sprite_render: Handle<SpriteSheet>
}

impl MainResource {
    fn new(x_force: f32, y_force: f32, gravity: f32, current_level_config: Option<LevelConfig>, lvl_nb: usize) -> MainResource {
        MainResource {
            x_force,
            y_force,
            _gravity: gravity,
            victory: false,
            current_rotation_angle: 0.,
            power: 0,
            is_landed: true,
            is_exploding: false,
            should_be_reset: false,
            current_level_config,
            sprites: None,
            ship_life: 3,
            current_level: lvl_nb,
            ship_fuel: 10. * 50.,
            should_reset_plasma_timers: true,
            should_reset_bonuses: true,
            bullet_hit_timer: 0.,
            collected_coin: 0,
            should_go_to_next_level: false
        }
    }

    pub fn bonus_heal(&mut self){
        if self.ship_life < 3 { self.ship_life += 1 ;}
    }
    pub fn bonus_coin(&mut self){
        self.collected_coin += 1;
    }

    pub fn bonus_fuel(&mut self){
        self.ship_fuel += 4. * 50.;
        if self.ship_fuel > 10. * 50. {
            self.ship_fuel = 10. * 50.;
        }
    }

    pub fn level_config(&self) -> &LevelConfig {
        &(self.current_level_config.as_ref().unwrap())
    }

    pub fn new_from_level(config: Option<LevelConfig>, lvl_nb: usize) -> MainResource {
        MainResource::new (0., 0., 1.0, config, lvl_nb)
    }

    pub fn reset(&mut self) {
        self.is_landed = true;
        self.y_force = 0.;
        self.x_force = 0.;
        self.power = 0;
        self.current_rotation_angle = 0.;
        self.is_exploding = false;
        self.should_be_reset = false;
        self.ship_life = 3;
        self.ship_fuel = 10. * 50.;
        self.bullet_hit_timer = 0.;
        self.should_reset_bonuses= true;
        self.collected_coin= 0;
    }

    pub fn power(&mut self, delta_time: f32,  rotation: &UnitQuaternion<f32>) {
        self.is_landed = false;
        self.y_force += delta_time * calculate_y_force(rotation.rotation().quaternion().k);
        self.x_force += delta_time * calculate_x_force(rotation.rotation().quaternion().k);
        self.ship_fuel -= cmp::max(self.power, 30) as f32 * delta_time;
        self.power += 1;
    }

    pub fn fuel_up(&mut self, delta_time: f32) {
        self.ship_fuel += 100. * delta_time;
        if self.ship_fuel > 500. {
            self.ship_fuel = 500.;
        }
    }

    pub fn apply_gravity(&mut self, delta_time: f32) {
        if self.is_landed {return;}
        self.y_force -= 1.5 * delta_time;
        if self.x_force > 0. {
            self.x_force -= 0.2 * delta_time;
            if self.x_force < 0. { self.x_force = 0.}
        }else if self.x_force < 0. {
            self.x_force += 0.2 * delta_time;
            if self.x_force > 0. { self.x_force = 0.}
        }

        self.power = 0;
    }

    pub fn bullet_hit(&mut self){
        if self.ship_life > 0 {
            self.ship_life -= 1;
        }
        self.bullet_hit_timer = 0.3;
    }

    pub fn get_colliders_polygons_for_collision(&self, x: f32, y:f32) -> Vec<Polygon<f32>> {
        self.get_colliders_for_collision(x, y).to_owned_polygons()
    }

    pub fn get_colliders_for_collision(&self, x: f32, y:f32) -> Colliders {
        let main_collider = Collider::new(Point2D{x: x+2., y: y-2.}, 28., -28.);
        let colliders = Colliders::from_vec(vec![main_collider]);
        colliders
    }

    pub fn get_colliders_polygons_for_landing(&self, x: f32, y:f32) -> Vec<Polygon<f32>> {
        let main_collider = Collider::new(Point2D{x, y}, 32., -32.);
        let colliders = Colliders::from_vec(vec![main_collider]);
        colliders.to_owned_polygons()
    }

    pub fn sprite_nb(&self) -> usize {
        if self.power == 0 {
            if self.bullet_hit_timer > 0. {
                return 4;
            }
            0
        } else {
            if self.bullet_hit_timer > 0. {
                return 9;
            }
            rand::thread_rng().gen_range(1, 4) as usize
        }
    }
}

impl Default for MainResource {
    fn default() -> Self {
        MainResource::new(0., 0., 0., None, 0)
    }
}


fn calculate_y_force(z_rotation: f32) -> f32 {
    2.5 * ((0.75 - (z_rotation.abs())) / 0.75)
}

fn calculate_x_force(z_rotation: f32) -> f32 {
    -1.5 * ((z_rotation) / 0.50)
}