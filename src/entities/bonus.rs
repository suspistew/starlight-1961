use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Bonus {
    pub initial_sprite: usize,
    pub kind: BonusKind,
    pub taken: bool
}

impl Component for Bonus {
    type Storage = DenseVecStorage<Self>;
}

pub enum BonusKind{
    Fuel, Wrench, Coin
}