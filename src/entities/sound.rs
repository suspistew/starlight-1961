use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct MenuSound;

impl Component for MenuSound {
    type Storage = DenseVecStorage<Self>;
}
