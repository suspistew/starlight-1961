use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Read, Join};
use amethyst::core::Transform;
use crate::entities::ship::Ship;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut transforms, mut ships, input, mut sprites): Self::SystemData) {
        for (ship, transform, sprite_render) in (&mut ships, &mut transforms, &mut sprites).join() {
            if ship.y_force != 0. {
                transform.append_translation_xyz(0., ship.y_force, 0.);
            }
            sprite_render.sprite_number = ship.sprite_nb();
        }
        for (ship, transform) in (&mut ships, &mut transforms).join() {
            if let Some(true) = input.action_is_down("power") {
                ship.y_force += 0.01;
                ship.power += 1;
            }else{
                ship.y_force -= 0.02;
                ship.power = 0;
            }
            if let Some(true) = input.action_is_down("rotate_right") {
                transform.prepend_rotation_z_axis(0.01);
            }
        }
    }
}