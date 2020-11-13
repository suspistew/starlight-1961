use crate::utils::Point2D;
use amethyst::core::ecs::{Component, DenseVecStorage};
use geo::{Polygon, CoordinateType, LineString};

pub struct Transparent;

impl Component for Transparent {
    type Storage = DenseVecStorage<Self>;
}

pub struct Colliders {
    polygons: Vec<Polygon<f32>>
}

impl Colliders {
    pub fn from_vec(colliders: Vec<Collider>) -> Colliders {
        Colliders { polygons: colliders.iter().map(|collider| collider.to_polygon()).collect() }
    }

    fn new() -> Colliders {
        Colliders {
            polygons: Vec::new()
        }
    }

    pub fn polygons(&self) -> &Vec<Polygon<f32>> {
        &self.polygons
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::prelude::Intersects;


    #[test]
    fn test() {
       let pol_1 = Polygon::new(
           LineString::from(
               vec![(144., 96.), (176., 96.), (176., 64.), (144., 64.), (144., 96.)]
           ),
           vec![],
       );
        let pol_2 = Polygon::new(
            LineString::from(
                vec![(248., 128.), (256., 128.), (256., 96.), (248., 96.), (248., 128.)]
            ),
            vec![],
        );
        assert_eq!(false, pol_1.intersects(&pol_2));
    }
}