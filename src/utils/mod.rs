pub mod sprite_to_entities;
pub mod starlight_tile;

#[derive(Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left, Right, Top, Bottom
}