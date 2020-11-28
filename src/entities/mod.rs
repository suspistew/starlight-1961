use amethyst::core::ecs::{Component, DenseVecStorage};

pub mod ship;
pub mod collision;
pub mod explosion;
pub mod canons;
pub mod doors;
pub mod main_menu;
pub mod blade_saw;
pub mod bonus;
pub mod sound;

pub struct TransitionFade;

impl Component for TransitionFade {
    type Storage = DenseVecStorage<Self>;
}