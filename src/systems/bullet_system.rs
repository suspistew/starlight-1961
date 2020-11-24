use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Join, Read, Entities, Write};
use crate::entities::canons::{Bullet, Canon, CanonKind, canon_kind_to_bullet_speed};
use amethyst::core::{Transform, Time};
use crate::utils::Direction;
use crate::entities::collision::{Colliders, are_colliding};
use geo::Polygon;
use crate::utils::sprites::sprite_to_entities::init_bullet_collider;
use crate::resources::main_resource::MainResource;
use crate::entities::ship::ShipParent;

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        WriteStorage<'s, Bullet>,
        ReadStorage<'s, Canon>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, ShipParent>,
        Write<'s, MainResource>,
        Read<'s, Time>,
        Entities<'s>,);

    fn run(&mut self, (mut bullets, canons, mut transforms, colliders, ships,mut main_resource, time, entities): Self::SystemData) {
        let mut ship_polygon = Vec::new();
        for (_ship, transform) in (&ships, &transforms).join() {
            ship_polygon = main_resource.get_colliders_polygons_for_collision(transform.translation().x, transform.translation().y);
        }
        let mut bullet_vec: Vec<(u32, Vec<Polygon<f32>>)> = Vec::new();
        for (bullet, transform, entity) in (&mut bullets, &mut transforms, &entities).join() {
            let colliders = init_bullet_collider(&bullet.kind, transform.translation().x, transform.translation().y);
            if are_colliding(colliders.polygons(), &ship_polygon) {
                match bullet.kind {
                    CanonKind::Air => {
                        main_resource.x_force -= 2. * time.delta_seconds();
                    },
                    _=> {
                        main_resource.bullet_hit();

                        let _res = entities.delete(entity);
                    }
                }

            }else{
                bullet_vec.push((entity.id(), colliders.polygons().clone()));
            }
            let bullet_speed = canon_kind_to_bullet_speed(&bullet.kind);
            match bullet.direction {
                Direction::Left => transform.append_translation_xyz(-1. * bullet_speed  * time.delta_seconds(), 0., 0.),
                Direction::Right => transform.append_translation_xyz(bullet_speed * time.delta_seconds(), 0., 0.),
                _ => {transform.append_translation_xyz(0.,0.,0.)}
            };
            bullet.life_duration -= time.delta_seconds();
            if bullet.life_duration <= 0. { let _res = entities.delete(entity); }
        }
        for (platform_collider, _, _) in (&colliders, !&bullets, !&canons).join() {
            for (id, polygons) in bullet_vec.iter(){
                if are_colliding(&polygons, platform_collider.polygons()) {
                    let e = entities.entity(*id);
                    let _res = entities.delete(e);
                }
            }
        }
    }
}