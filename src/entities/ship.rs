use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::Rng;

pub struct ShipParent;

impl Component for ShipParent {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ship {
    pub x_force: f32,
    pub y_force: f32,
    pub power: usize,
    pub last_sprite: usize,
    gravity: f32,
    x: f32,
    y: f32
}

impl Ship {
    pub fn new(x_force: f32, y_force: f32, gravity: f32, x: f32, y: f32) -> Ship {
        Ship {
            x_force,
            y_force,
            gravity,
            x,
            y,
            power: 0,
            last_sprite: 0
        }
    }

    pub fn sprite_nb(&mut self) -> usize {
        if self.power == 0 {self.last_sprite= 0;  return 0 }
        rand::thread_rng().gen_range(1, 4) as usize
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

