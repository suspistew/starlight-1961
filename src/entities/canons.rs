use amethyst::core::ecs::{DenseVecStorage, Component};
use crate::utils::Direction;
#[derive(Debug)]
pub enum CanonKind {
    Bullet, Plasma, Air
}
#[derive(Debug)]
pub struct Canon {
    pub direction: Direction,
    pub kind: CanonKind,
    pub bullet_x_start: f32,
    pub bullet_y_start: f32
}

impl Component for Canon {
    type Storage = DenseVecStorage<Self>;
}
#[derive(Debug)]
pub struct Bullet {
    pub direction: Direction,
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}