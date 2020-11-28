use amethyst::core::ecs::{System, ReadStorage, Write, Join};
use crate::entities::collision::{Colliders, Arrival, are_colliding};
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::main_resource::MainResource;

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Arrival>,
        Write<'s, MainResource>
    );

    fn run(&mut self, (colliders, ships, transforms, arrivals, mut main_resource): Self::SystemData) {
        if main_resource.is_landed {
            for (_ship, transform) in (&ships, &transforms).join() {
                let ship_polygon = main_resource.get_colliders_polygons_for_landing(transform.translation().x, transform.translation().y);
                for (collider, _) in (&colliders, &arrivals).join() {
                    let struct_polygons = collider.polygons();
                    if are_colliding(&ship_polygon, struct_polygons)
                        && main_resource.collected_coin == main_resource.level_config().coin_nb{
                        main_resource.victory = true;
                    }
                }
            }
        }
    }
}