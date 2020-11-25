pub mod sprites;
pub mod level_reader;

#[derive(Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Left, Right, Top, Bottom
}