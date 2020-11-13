use amethyst::core::ecs::{System, ReadStorage, Write, Join};
use crate::entities::collision::Colliders;
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::ship_resource::ShipResource;
use geo::algorithm::intersects::Intersects;
use geo::Polygon;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        Write<'s, ShipResource>,
    );

    fn run(&mut self, (colliders, ships, transforms, ship_resource): Self::SystemData) {
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
            for (collider) in (&colliders).join() {
                let struct_polygons = collider.polygons();
                are_colliding(&ship_polygon, struct_polygons);
            }
        }
    }
}


fn are_colliding(ship_polygon: &Vec<Polygon<f32>>, struct_polygons: &Vec<Polygon<f32>>) -> bool {
    for polygon in ship_polygon.iter() {
        for struct_polygon in struct_polygons.iter() {
            if polygon.intersects(struct_polygon) {
                return true;
            }
        }
    }
    false
}

