use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Read, Join, Write};
use amethyst::core::Transform;
use crate::entities::ship::{Ship, ShipParent, Thrusters};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;
use crate::resources::ship_resource::ShipResource;
use amethyst::core::num::FloatConst;


pub struct ShipSystem ;

fn ANGLE_ROTATION_MODIFIER_BIND_TO_PI() -> f32 { f32::PI() / 96.}
const ANGLE_ROTATION_DEGREE_MODIFIER: f32 = 1.875;


impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, ShipParent>,
        ReadStorage<'s, Thrusters>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, ShipResource>,
    );

    fn run(&mut self, (mut transforms, ships, ships_parent, thrusters, input, mut sprites, mut ship_resource): Self::SystemData) {

        for (_ship, sprite_render) in (&ships, &mut sprites).join() {
            sprite_render.sprite_number = ship_resource.sprite_nb();
        }
        for (_thruster, sprite_render) in (&thrusters, &mut sprites).join() {
            sprite_render.sprite_number = 5 + ship_resource.sprite_nb();
        }
        for (_ship_parent, transform) in (&ships_parent, &mut transforms).join() {
            if ship_resource.y_force != 0. || ship_resource.x_force != 0. {
                transform.append_translation_xyz(ship_resource.x_force, ship_resource.y_force, 0.);
            }
        }
        for (_ship, transform) in (&ships, &mut transforms).join() {
            if let Some(true) = input.action_is_down("power") {
                ship_resource.y_force += calculate_y_force(transform.rotation().quaternion().k);
                ship_resource.x_force += calculate_x_force(transform.rotation().quaternion().k);
                ship_resource.power += 1;
            }else{
                 if !ship_resource.is_landed {
                 }
                ship_resource.y_force -= 0.02;
                // TODO : Add x force lose
                ship_resource.power = 0;
            }
            if let Some(true) = input.action_is_down("rotate_left") {
                if transform.rotation().quaternion().k < 0.50 {
                    transform.prepend_rotation_z_axis(-1. * ANGLE_ROTATION_MODIFIER_BIND_TO_PI());
                    ship_resource.current_rotation_angle -= ANGLE_ROTATION_DEGREE_MODIFIER;
                }
            }
            if let Some(true) = input.action_is_down("rotate_right") {
                if transform.rotation().quaternion().k > -0.50 {
                    transform.prepend_rotation_z_axis(ANGLE_ROTATION_MODIFIER_BIND_TO_PI());
                    ship_resource.current_rotation_angle += ANGLE_ROTATION_DEGREE_MODIFIER;
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