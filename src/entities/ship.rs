use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::Rng;

pub struct ShipParent;

impl Component for ShipParent {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ship;

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

pub struct Thrusters;

impl Component for Thrusters {
    type Storage = DenseVecStorage<Self>;
}

