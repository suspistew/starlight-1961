use amethyst::core::ecs::{System, Entities, WriteStorage};
use amethyst::renderer::SpriteRender;

pub struct ExplosionSystem{
    sprite_update_timer: Option<f32>
}

impl ExplosionSystem {
    pub fn new () -> ExplosionSystem {
        ExplosionSystem {
            sprite_update_timer: None
        }
    }
}

impl <'s> System<'s> for ExplosionSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(&mut self, (sprites, entities): Self::SystemData) {

    }

}

