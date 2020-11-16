use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Explosion;

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}

