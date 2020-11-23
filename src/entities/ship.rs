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

pub struct ShipPower;

impl Component for ShipPower {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShipLife {
    pub life_point: u8
}

impl Component for ShipLife {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShipFuel {
    pub fuel_point: u8
}

impl Component for ShipFuel {
    type Storage = DenseVecStorage<Self>;
}

