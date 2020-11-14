use amethyst::core::ecs::{System, ReadStorage, Write, Join, Entities};
use crate::entities::collision::{Colliders, LandingPlatform};
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::ship_resource::ShipResource;
use geo::algorithm::intersects::Intersects;
use geo::Polygon;


pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        Write<'s, ShipResource>,
        Entities<'s>,
    );

    fn run(&mut self, (colliders, landing_plateforms, ships, transforms, mut ship_resource, _entities): Self::SystemData) {
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
            for (collider, _) in (&colliders, !&landing_plateforms).join() {
                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding &&  are_colliding(&ship_polygon, struct_polygons) {
                    ship_resource.is_exploding = true;
                    println!("colliding");
/*
                    entities
                        .build_entity()
                        .with(Explosion)
                        .with(t, &mut transforms)
                        .with(Bloc::new(BlocKind::Moving), &mut allblocs)
                        .build();
                        */
                }
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

