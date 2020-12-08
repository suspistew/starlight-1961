use amethyst::core::ecs::{Component, DenseVecStorage};

pub enum DoorState {
    Open,
    Closed,
}

pub struct PlasmaDoor {
    pub initial_sprite: usize,
    pub state: DoorState,
}

impl Component for PlasmaDoor {
    type Storage = DenseVecStorage<Self>;
}
