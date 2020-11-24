use amethyst::core::ecs::{DenseVecStorage, Component};
use crate::utils::Direction;
#[derive(Debug, Clone)]
pub enum CanonKind {
    Bullet, Smg, Air, _Plasma,
}

pub fn canon_to_shooting_timer(kind: &CanonKind) -> f32{
    match kind {
        CanonKind::Bullet => 1.5,
        CanonKind::Smg => 0.8,
        CanonKind::Air => 0.4,
        _ => 0.
    }
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
    pub kind: CanonKind,
    pub life_duration: f32
}

pub fn canon_kind_to_bullet_life_duration(kind: &CanonKind) -> f32 {
    match kind {
        CanonKind::Bullet | CanonKind::Smg => 3.,
        CanonKind::Air => 0.4,
        _ =>0.
    }
}

pub fn canon_kind_to_bullet_speed(kind: &CanonKind) -> f32 {
    match kind {
        CanonKind::Bullet | CanonKind::Smg => 180.,
        CanonKind::Air => 80.,
        _ =>0.
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}