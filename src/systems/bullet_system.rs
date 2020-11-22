use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Join, Read, Entities, Write};
use crate::entities::canons::{Bullet, Canon, CanonKind};
use amethyst::core::{Transform, Time};
use crate::utils::Direction;
use crate::entities::collision::{Colliders, are_colliding};
use geo::Polygon;
use crate::utils::sprites::sprite_to_entities::init_bullet_collider;
use std::ops::Deref;
use crate::resources::main_resource::MainResource;
use crate::entities::ship::ShipParent;

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Canon>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, ShipParent>,
        Write<'s, MainResource>,
        Read<'s, Time>,
        Entities<'s>,);

    fn run(&mut self, (bullets, canons, mut transforms, colliders, ships,mut main_resource, time, entities): Self::SystemData) {
        let mut ship_polygon = Vec::new();
        for (_ship, transform) in (&ships, &transforms).join() {
            ship_polygon = main_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
        }
        let mut bullet_vec: Vec<(u32, Vec<Polygon<f32>>)> = Vec::new();
        for (bullet, transform, entity) in (&bullets, &mut transforms, &entities).join() {
            let colliders = init_bullet_collider(CanonKind::Bullet, transform.translation().x, transform.translation().y);
            if are_colliding(colliders.polygons(), &ship_polygon) {
                main_resource.ship_life -= 1;
                entities.delete(entity);
            }else{
                bullet_vec.push((entity.id(), colliders.polygons().clone()));
            }
            match bullet.direction {
                Direction::Left => transform.append_translation_xyz(-200.0 * time.delta_seconds(), 0., 0.),
                Direction::Right => transform.append_translation_xyz(200.0 * time.delta_seconds(), 0., 0.),
                _ => {transform.append_translation_xyz(0.,0.,0.)}
            };
        }
        for (platform_collider, _, _) in (&colliders, !&bullets, !&canons).join() {
            for (id, polygons) in bullet_vec.iter(){
                if are_colliding(&polygons, platform_collider.polygons()) {
                    let e = entities.entity(*id);
                    entities.delete(e);
                }
            }
        }
    }
}