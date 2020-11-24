use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use crate::resources::main_resource::MainResource;
use crate::entities::ship::Thrusters;
use amethyst::renderer::SpriteRender;

pub struct ThrustersSystem ;

impl<'s> System<'s> for ThrustersSystem {
    type SystemData = (
        ReadStorage<'s, Thrusters>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, MainResource>,
    );

    fn run(&mut self, (thrusters, mut sprite_renders, main_resource): Self::SystemData) {
        for (_, sprite_render) in (&thrusters, &mut sprite_renders).join() {
            if main_resource.is_exploding {
                sprite_render.sprite_number = 5;
            }else {
                sprite_render.sprite_number = {
                    if main_resource.bullet_hit_timer > 0. {
                        if main_resource.power == 0 {
                            5
                        }else{
                            6
                        }
                    }else {
                        5 + main_resource.sprite_nb()
                    }
                };
            }
        }
    }
}