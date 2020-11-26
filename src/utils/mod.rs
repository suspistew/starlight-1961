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

pub fn distance_between_two_points(xa: f32, ya:f32, xb:f32, yb:f32) -> f32 {
    ((xa - xb) * (xa - xb) + (ya - yb) * (ya - yb)).sqrt()
}