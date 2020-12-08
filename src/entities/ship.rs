use amethyst::ecs::prelude::{Component, DenseVecStorage};

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

pub struct ShipPowerLeftNumber;

impl Component for ShipPowerLeftNumber {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShipPowerRightNumber;

impl Component for ShipPowerRightNumber {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShipLife {
    pub life_point: u8,
}

impl Component for ShipLife {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShipFuel {
    pub fuel_point: u8,
}

impl Component for ShipFuel {
    type Storage = DenseVecStorage<Self>;
}

pub struct Coin {
    pub coin_id: usize,
}

impl Component for Coin {
    type Storage = DenseVecStorage<Self>;
}
