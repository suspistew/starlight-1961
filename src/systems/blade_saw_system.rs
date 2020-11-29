use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Join, Read, Write};
use crate::entities::blade_saw::{BladeSaw, BladeSawSprite};
use amethyst::core::{Transform, Time};
use amethyst::core::num::FloatConst;
use crate::utils::sprites::TILE_SIZE;
use crate::resources::main_resource::MainResource;
use crate::utils::sprites::sprite_to_entities::init_blade_saw_collider;
use crate::entities::ship::ShipParent;
use crate::entities::collision::are_colliding;

pub struct BladeSawSystem;

impl<'s> System<'s> for BladeSawSystem {
    type SystemData = (
        WriteStorage<'s, BladeSaw>,
        ReadStorage<'s, BladeSawSprite>,
        WriteStorage<'s, Transform>,
        Write<'s, MainResource>,
        ReadStorage<'s, ShipParent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut blade_parents, blade_sprite, mut transforms, mut main_resource, ships, time): Self::SystemData) {
        for (_, transform) in (&blade_sprite, &mut transforms).join() {
            transform.prepend_rotation_z_axis(3. * time.delta_seconds());
        }
        let mut ship_polygon = Vec::new();
        for (_ship, transform) in (&ships, &transforms).join() {
            ship_polygon = main_resource.get_colliders_for_collision(transform.translation().x, transform.translation().y);
        }
        for (blade, transform) in (&mut blade_parents, &mut transforms).join() {
            transform.append_translation_xyz(50. * blade.direction_x * time.delta_seconds(), 50. * blade.direction_y * time.delta_seconds(), 0.);
            let (x, y) = (transform.translation().x, transform.translation().y);
            if x <= (blade.min_x * TILE_SIZE - 16.)
                ||  x >= (blade.max_x * TILE_SIZE + 16.) {
                blade.direction_x *= -1.;
            }
            if  y >= (main_resource.level_config().height as f32 - blade.min_y - 1.) * TILE_SIZE + 16.
                ||  y <= (main_resource.level_config().height as f32 - blade.max_y - 1.) * TILE_SIZE - 16. {
                blade.direction_y *= -1.;
            }

            let collider = init_blade_saw_collider(x, y);
            if are_colliding( &ship_polygon, collider.polygons()){
                main_resource.ship_life -= main_resource.ship_life;
            }
        }
    }
}
