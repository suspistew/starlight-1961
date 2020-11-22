use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Entities, Join, Read};
use crate::entities::collision::Colliders;
use amethyst::core::{Transform, Time};
use crate::entities::canons::{Canon, Bullet, CanonKind};
use amethyst::renderer::SpriteRender;
use std::collections::HashMap;
use amethyst::core::ecs::world::Index;
use rand::{random, Rng};
use crate::utils::Direction;
use crate::resources::main_resource::MainResource;
use crate::utils::sprites::sprite_to_entities::init_bullet_collider;

pub struct CanonSystem {
    shooting_timers: HashMap<u32, f32>
}

impl Default for CanonSystem {
    fn default() -> Self {
        CanonSystem {
            shooting_timers: HashMap::new()
        }
    }
}

impl<'s> System<'s> for CanonSystem {
    type SystemData = (
        ReadStorage<'s, Canon>,
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Read<'s, MainResource>,
        Entities<'s>,
    );

    fn run(&mut self, (canons, mut bullets, mut transforms, mut sprite_renders, time,resource, entities): Self::SystemData) {
        for (canon, entity) in (&canons, &entities).join() {
            *self.shooting_timers.entry(entity.id())
                .or_insert(rand::thread_rng().gen_range(0.1, 2.5)) -= time.delta_seconds();
            let mut val = self.shooting_timers.get(&entity.id()).unwrap();
            if val <= &0. {
                let mut bullet_transform = Transform::default();
                bullet_transform.set_translation_xyz(canon.bullet_x_start, canon.bullet_y_start, 0.9);
                entities
                    .build_entity()
                    .with(Bullet{ direction: canon.direction.clone() }, &mut bullets)
                    .with(SpriteRender {
                        sprite_sheet: resource.sprites.as_ref().unwrap().bullet_sprite_render.clone(),
                        sprite_number: 0,
                    }, &mut sprite_renders)
                    .with(bullet_transform, &mut transforms)
                    .build();

                self.shooting_timers.remove(&entity.id());
                self.shooting_timers.insert(entity.id(), 1.5);
            }
        }
    }
}