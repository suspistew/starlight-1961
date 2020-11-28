pub mod sprites;
pub mod level_reader;
pub mod sound;
pub mod save;

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

pub fn min_of_f32_vec(mut list: [f32; 4]) -> f32 {
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    list[0]
}

pub fn max_of_f32_vec(mut list: [f32; 4]) -> f32 {
    list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    list[0]
}