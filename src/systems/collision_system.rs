use amethyst::core::ecs::{System, ReadStorage, Write, Join};
use crate::entities::collision::{Colliders, LandingPlatform, are_colliding};
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::main_resource::MainResource;
use crate::entities::canons::Bullet;
use crate::entities::doors::{PlasmaDoor, DoorState};
use crate::entities::bonus::Bonus;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, PlasmaDoor>,
        ReadStorage<'s, Bonus>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        Write<'s, MainResource>,
        ReadStorage<'s, Bullet>
    );

    fn run(&mut self, (colliders, landing_plateforms,plasma_doors, bonus,  ships, transforms, mut ship_resource, bullets): Self::SystemData) {
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons_for_collision(transform.translation().x, transform.translation().y);
            for (collider, _, _, _, _) in (&colliders, !&landing_plateforms, !&bullets, !&plasma_doors, !&bonus).join() {
                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding && are_colliding(&ship_polygon, struct_polygons) {
                    ship_resource.ship_life  -= ship_resource.ship_life;
                }
            }
            for (collider, door) in (&colliders, &plasma_doors).join() {
                match door.state {
                    DoorState::Closed=> {
                        let struct_polygons = collider.polygons();
                        if !ship_resource.is_exploding && are_colliding(&ship_polygon, struct_polygons) {
                            ship_resource.ship_life -= ship_resource.ship_life;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

