use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Entities, Join, Read};
use amethyst::core::{Transform, Time};
use crate::entities::canons::{Canon, Bullet, canon_to_shooting_timer, CanonKind, canon_kind_to_bullet_life_duration};
use amethyst::renderer::SpriteRender;
use std::collections::HashMap;
use rand::Rng;
use crate::resources::main_resource::MainResource;

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

    fn run(&mut self, (canons, mut bullets, mut transforms, mut sprite_renders, time, resource, entities): Self::SystemData) {
        for (canon, entity) in (&canons, &entities).join() {
            *self.shooting_timers.entry(entity.id())
                .or_insert(rand::thread_rng().gen_range(0.1, 2.5)) -= time.delta_seconds();
            let val = self.shooting_timers.get(&entity.id()).unwrap();
            if val <= &0. {
                match canon.kind {
                    CanonKind::Air => {
                        for bullet_index in 0..1 {
                            let mut bullet_transform = Transform::default();
                            bullet_transform.set_translation_xyz(canon.bullet_x_start - bullet_index as f32 * 15.,  canon.bullet_y_start, 0.9);
                            entities
                                .build_entity()
                                .with(Bullet { direction: canon.direction.clone(), kind: canon.kind.clone(), life_duration: canon_kind_to_bullet_life_duration(&canon.kind) }, &mut bullets)
                                .with(SpriteRender {
                                    sprite_sheet: resource.sprites.as_ref().unwrap().bullet_sprite_render.clone(),
                                    sprite_number: 2,
                                }, &mut sprite_renders)
                                .with(bullet_transform, &mut transforms)
                                .build();
                        }
                    }
                    _ => {
                        let mut bullet_transform = Transform::default();
                        bullet_transform.set_translation_xyz(canon.bullet_x_start, canon.bullet_y_start , 0.9);
                        entities
                            .build_entity()
                            .with(Bullet { direction: canon.direction.clone(), kind: canon.kind.clone(), life_duration: canon_kind_to_bullet_life_duration(&canon.kind) }, &mut bullets)
                            .with(SpriteRender {
                                sprite_sheet: resource.sprites.as_ref().unwrap().bullet_sprite_render.clone(),
                                sprite_number: 0,
                            }, &mut sprite_renders)
                            .with(bullet_transform, &mut transforms)
                            .build();

                    }
                }

                self.shooting_timers.remove(&entity.id());
                self.shooting_timers.insert(entity.id(), canon_to_shooting_timer(&canon.kind));
            }
        }
    }
}
