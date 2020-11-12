use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Read, Join};
use amethyst::core::Transform;
use crate::entities::ship::{Ship, ShipParent};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, ShipParent>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut transforms, mut ships, mut ships_parent, input, mut sprites): Self::SystemData) {
        let mut y_force = 0.;
        let mut x_force = 0.;

        for (ship, sprite_render) in (&mut ships, &mut sprites).join() {
            if ship.y_force != 0. {
                y_force = ship.y_force;
            }
            if ship.x_force != 0. {
                x_force = ship.x_force;
            }
            sprite_render.sprite_number = ship.sprite_nb();
        }
        for (_ship_parent, transform) in (&mut ships_parent, &mut transforms).join() {
            if y_force != 0. || x_force != 0. {
                transform.append_translation_xyz(x_force, y_force, 0.);
            }
        }
        for (ship, transform) in (&mut ships, &mut transforms).join() {
            if let Some(true) = input.action_is_down("power") {
                ship.y_force += calculate_y_force(transform.rotation().quaternion().k);
                ship.x_force += calculate_x_force(transform.rotation().quaternion().k);
                ship.power += 1;
            }else{
                ship.y_force -= 0.02;
                // TODO : Add x force lose
                ship.power = 0;

            }
            if let Some(true) = input.action_is_down("rotate_left") {
                if transform.rotation().quaternion().k < 0.50 {
                    transform.prepend_rotation_z_axis(-0.03);
                }
            }
            if let Some(true) = input.action_is_down("rotate_right") {
                if transform.rotation().quaternion().k > -0.50 {
                    transform.prepend_rotation_z_axis(0.03);
                }
            }

        }
    }
}

fn calculate_y_force(z_rotation: f32) -> f32 {
    0.02 * ((0.75 - (z_rotation.abs())) / 0.75)
}

fn calculate_x_force(z_rotation: f32) -> f32 {
    -0.05 * ((z_rotation) / 0.50 )
}