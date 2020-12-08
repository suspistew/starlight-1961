use amethyst::core::ecs::{Component, DenseVecStorage};

pub mod blade_saw;
pub mod bonus;
pub mod canons;
pub mod collision;
pub mod doors;
pub mod explosion;
pub mod main_menu;
pub mod ship;
pub mod sound;

pub struct TransitionFade;

impl Component for TransitionFade {
    type Storage = DenseVecStorage<Self>;
}
