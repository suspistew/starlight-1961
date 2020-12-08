use amethyst::core::ecs::{Component, DenseVecStorage};
use serde::Deserialize;

pub struct BladeSawSprite;
impl Component for BladeSawSprite {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BladeSaw {
    pub direction_x: f32,
    pub direction_y: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub max_x: f32,
}

impl Component for BladeSaw {
    type Storage = DenseVecStorage<Self>;
}
