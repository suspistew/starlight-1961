use crate::entities::collision::{are_colliding, Colliders, LandingPlatform};
use crate::entities::ship::ShipParent;
use crate::resources::main_resource::MainResource;
use crate::systems::ship_systems::ANGLE_ROTATION_DEGREE_MODIFIER;
use crate::utils::sound::{play_land, Sounds};
use crate::utils::sprites::TILE_SIZE;
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::{Join, Read, ReadExpect, ReadStorage, System, Write};
use amethyst::core::Transform;

pub struct LandingSystem;

impl<'s> System<'s> for LandingSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Transform>,
        Write<'s, MainResource>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            colliders,
            landing_plateforms,
            ships,
            transforms,
            mut ship_resource,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons_for_landing(
                transform.translation().x,
                transform.translation().y,
            );
            for (collider, _) in (&colliders, &landing_plateforms).join() {
                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding
                    && !ship_resource.is_landed
                    && are_colliding(&ship_polygon, struct_polygons)
                {
                    if correct_landing_position(&ship_resource, transform, collider) {
                        play_land(&*sounds, &storage, audio_output.as_deref());
                        ship_resource.is_landed = true;
                        ship_resource.y_force = 0.;
                        ship_resource.x_force = 0.;
                    } else {
                        if ship_resource.y_force < 0. {
                            ship_resource.ship_life -= ship_resource.ship_life;
                        }
                    }
                }
            }
        }
    }
}

const OVERFLOW_TOLERANCE: f32 = 5.;

fn correct_landing_position(
    ship_resource: &MainResource,
    transform: &Transform,
    colliders: &Colliders,
) -> bool {
    let ship_x = transform.translation().x;
    let plateform_x_start = colliders.colliders().get(0).unwrap().top_left_point().x;
    let plateform_x_end = colliders.colliders().get(0).unwrap().top_right_point().x;

    ship_resource.power == 0
        && ship_resource.y_force > -1.2
        && ship_resource.current_rotation_angle > -9. * ANGLE_ROTATION_DEGREE_MODIFIER
        && ship_resource.current_rotation_angle < 9. * ANGLE_ROTATION_DEGREE_MODIFIER
        && ship_x + OVERFLOW_TOLERANCE >= plateform_x_start
        && (ship_x + TILE_SIZE - OVERFLOW_TOLERANCE) <= plateform_x_end
}
