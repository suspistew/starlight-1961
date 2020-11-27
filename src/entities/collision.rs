use crate::utils::{Point2D, min_of_f32_vec, max_of_f32_vec};
use amethyst::core::ecs::{Component, DenseVecStorage};
use geo::{Polygon, LineString};
use geo::intersects::Intersects;
use std::cmp::Ordering;

pub struct ButtonPlatform;

impl Component for ButtonPlatform {
    type Storage = DenseVecStorage<Self>;
}

pub struct LandingPlatform;

impl Component for LandingPlatform {
    type Storage = DenseVecStorage<Self>;
}

pub struct Arrival;

impl Component for Arrival {
    type Storage = DenseVecStorage<Self>;
}

pub struct Transparent;

impl Component for Transparent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Colliders {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    colliders: Vec<Collider>,
    polygons: Vec<Polygon<f32>>,
}

impl Colliders {
    pub fn from_vec(colliders: Vec<Collider>) -> Colliders {
        let (min_x, min_y, max_x, max_y) = {
            (
                colliders.iter().map(|col| min_of_f32_vec([col.a.x, col.b.x, col.c.x, col.d.x])).min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap(),
                colliders.iter().map(|col| min_of_f32_vec([col.a.y, col.b.y, col.c.y, col.d.y])).min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap(),
                colliders.iter().map(|col| max_of_f32_vec([col.a.x, col.b.x, col.c.x, col.d.x])).max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap(),
                colliders.iter().map(|col| max_of_f32_vec([col.a.y, col.b.y, col.c.y, col.d.y])).max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap()
            )
        };
        Colliders { polygons: colliders.iter().map(|collider| collider.to_polygon()).collect(), colliders, min_x, min_y, max_x, max_y }
    }
    pub fn from_points(a: Point2D, b: Point2D, c: Point2D, d: Point2D) -> Colliders {
        Colliders {
            polygons: vec![
                Polygon::new(
                    LineString::from(
                        vec![(a.x, a.y), (b.x, b.y), (c.x, c.y), (d.x, d.y), (a.x, a.y)]
                    ),
                    vec![],
                )
            ],
            colliders: Vec::new(),
            min_x: min_of_f32_vec([a.x, b.x, c.x, d.x]),
            min_y: min_of_f32_vec([a.y, b.y, c.y, d.y]),
            max_x: max_of_f32_vec([a.x, b.x, c.x, d.x]),
            max_y: max_of_f32_vec([a.y, b.y, c.y, d.y]),
        }
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
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
    pub d: Point2D,
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

pub fn compute_is_eligible_for_collision(col1: &Colliders, col2: &Colliders) -> bool {
    !(col1.min_x < col2.min_x && col1.max_x < col2.min_x && col1.min_x < col2.max_x && col1.max_x < col2.max_x)
        && !(col1.min_x > col2.min_x && col1.max_x > col2.min_x && col1.min_x > col2.max_x && col1.max_x > col2.max_x)
        && !(col1.min_y < col2.min_y && col1.max_y < col2.min_y && col1.min_y < col2.max_y && col1.max_y < col2.max_y)
        && !(col1.min_y > col2.min_y && col1.max_y > col2.min_y && col1.min_y > col2.max_y && col1.max_y > col2.max_y)
}