use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use crate::resources::ship_resource::ShipResource;
use crate::entities::ship::Thrusters;
use amethyst::renderer::SpriteRender;

pub struct ThrustersSystem ;

impl<'s> System<'s> for ThrustersSystem {
    type SystemData = (
        ReadStorage<'s, Thrusters>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, ShipResource>,
    );

    fn run(&mut self, (thrusters, mut sprite_renders, ship_resource): Self::SystemData) {
        for (_, sprite_render) in (&thrusters, &mut sprite_renders).join() {
            if ship_resource.is_exploding {
                sprite_render.sprite_number = 5;
            }else {
                sprite_render.sprite_number = 5 + ship_resource.sprite_nb();
            }
        }
    }
}