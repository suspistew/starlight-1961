use crate::entities::explosion::Explosion;
use crate::resources::main_resource::MainResource;
use amethyst::core::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::core::timing::Time;
use amethyst::renderer::SpriteRender;

pub struct ExplosionSystem {
    sprite_update_timer: Option<f32>,
}

impl ExplosionSystem {
    pub fn new() -> ExplosionSystem {
        ExplosionSystem {
            sprite_update_timer: None,
        }
    }
}

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (
        ReadStorage<'s, Explosion>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Write<'s, MainResource>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (explosions, mut sprites, time, mut ship_resource, entities): Self::SystemData,
    ) {
        if ship_resource.should_be_reset {
            return;
        }
        for (_, sprite, entity) in (&explosions, &mut sprites, &*entities).join() {
            if self.sprite_update_timer.is_none() {
                self.sprite_update_timer.replace(0.08);
            } else {
                self.sprite_update_timer
                    .replace(self.sprite_update_timer.unwrap() - time.delta_seconds());
                if self.sprite_update_timer.unwrap() <= 0.0 {
                    if sprite.sprite_number < 3 {
                        sprite.sprite_number += 1;
                        self.sprite_update_timer.replace(0.08);
                    } else {
                        let _deletion = entities.delete(entity);
                        self.sprite_update_timer = None;
                        ship_resource.should_be_reset = true;
                    }
                }
            }
        }
    }
}
