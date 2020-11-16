use amethyst::core::ecs::{System, ReadStorage, Write, Entities, Join, WriteStorage};
use crate::entities::collision::{Colliders, LandingPlatform, are_colliding};
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::main_resource::MainResource;
use crate::systems::ship_systems::ANGLE_ROTATION_DEGREE_MODIFIER;
use crate::states::level::TILE_SIZE;
use crate::entities::explosion::Explosion;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::assets::Handle;

pub struct LandingSystem;

impl <'s> System<'s> for LandingSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, ShipParent>,
        WriteStorage<'s, Transform>,
        Write<'s, MainResource>,
        WriteStorage<'s, Explosion>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(&mut self, (colliders, landing_plateforms, ships, mut transforms, mut ship_resource, mut explosions, mut sprites, entities): Self::SystemData) {
        let mut explosion_information = (false, 0., 0.);
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
            for (collider, _) in (&colliders, &landing_plateforms).join() {

                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding && !ship_resource.is_landed && are_colliding(&ship_polygon, struct_polygons) {
                    if correct_landing_position(&ship_resource, transform, collider) {
                        ship_resource.is_landed = true;
                        ship_resource.y_force = 0.;
                        ship_resource.x_force = 0.;
                    }else{
                        if ship_resource.y_force < 0.{ ship_resource.is_exploding = true; explosion_information = (true, transform.translation().x, transform.translation().y); }
                    }
                }
            }
        }
        if explosion_information.0 {
            let mut explosion_transform = Transform::default();
            explosion_transform.set_translation_xyz(explosion_information.1, explosion_information.2, 0.9);
            entities
                .build_entity()
                .with(Explosion, &mut explosions)
                .with(init_sprite_render(ship_resource.sprites.as_ref().unwrap().explosion_sprite_render.clone()), &mut sprites)
                .with(explosion_transform, &mut transforms)
                .build();
        }
    }
}

const OVERFLOW_TOLERANCE: f32 = 5.;

fn correct_landing_position(ship_resource: &MainResource, transform: &Transform, colliders: &Colliders) -> bool {
    let ship_x = transform.translation().x;
    let plateform_x_start = colliders.colliders().get(0).unwrap().top_left_point().x;
    let plateform_x_end = colliders.colliders().get(0).unwrap().top_right_point().x;

    ship_resource.power == 0
        && ship_resource.y_force > -1.0
        && ship_resource.current_rotation_angle > -8. * ANGLE_ROTATION_DEGREE_MODIFIER
        && ship_resource.current_rotation_angle < 8. * ANGLE_ROTATION_DEGREE_MODIFIER
    && ship_x + OVERFLOW_TOLERANCE >= plateform_x_start
    && (ship_x + TILE_SIZE - OVERFLOW_TOLERANCE) <= plateform_x_end

}

fn init_sprite_render(sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    }
}