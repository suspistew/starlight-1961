use crate::utils::Point2D;
use amethyst::core::ecs::{Component, DenseVecStorage};
use geo::{Polygon, CoordinateType, LineString};
use geo::intersects::Intersects;

pub struct LandingPlatform;

impl Component for LandingPlatform {
    type Storage = DenseVecStorage<Self>;
}

pub struct Transparent;

impl Component for Transparent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Colliders {
    colliders: Vec<Collider>,
    polygons: Vec<Polygon<f32>>
}

impl Colliders {
    pub fn from_vec(colliders: Vec<Collider>) -> Colliders {
        Colliders { polygons: colliders.iter().map(|collider| collider.to_polygon()).collect(), colliders }
    }

    pub fn polygons(&self) -> &Vec<Polygon<f32>> {
        &self.polygons
    }
    pub fn colliders(&self) -> &Vec<Collider> { &self.colliders }
    pub fn to_owned_polygons(&self) -> Vec<Polygon<f32>> {
        self.polygons.clone()
    }
}

impl Component for Colliders {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Collider {
    a: Point2D,
    b: Point2D,
    c: Point2D,
    d: Point2D,
}

impl Collider {
    pub fn new(starting_point: Point2D, width: f32, height: f32) -> Self {
        Collider {
            b: Point2D { x: starting_point.x + width, y: starting_point.y },
            d: Point2D { x: starting_point.x, y: starting_point.y + height },
            c: Point2D { x: starting_point.x + width, y: starting_point.y + height },
            a: starting_point,
        }
    }

    pub fn to_polygon(&self) -> Polygon<f32> {
        Polygon::new(
            LineString::from(
                vec![(self.a.x, self.a.y), (self.b.x, self.b.y), (self.c.x, self.c.y), (self.d.x, self.d.y), (self.a.x, self.a.y)]
            ),
            vec![],
        )
    }

    pub fn top_left_point(&self) -> &Point2D {
        &self.a
    }
    pub fn top_right_point(&self) -> &Point2D {
        &self.b
    }
}

pub fn are_colliding(ship_polygon: &Vec<Polygon<f32>>, struct_polygons: &Vec<Polygon<f32>>) -> bool {
    for polygon in ship_polygon.iter() {
        for struct_polygon in struct_polygons.iter() {
            if polygon.intersects(struct_polygon) {
                return true;
            }
        }
    }
    false
}