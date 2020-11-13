use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Read, Join, Write};
use amethyst::core::Transform;
use crate::entities::ship::{Ship, ShipParent, Thrusters};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;
use crate::resources::ship_resource::ShipResource;
use amethyst::core::num::FloatConst;
use std::panic::resume_unwind;

pub struct ShipSystem;

fn ANGLE_ROTATION_MODIFIER_BIND_TO_PI() -> f32 { f32::PI() / 96. }

const ANGLE_ROTATION_DEGREE_MODIFIER: f32 = 1.875;


impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, ShipParent>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, ShipResource>,
    );

    fn run(&mut self, (mut transforms, ships, ships_parent, input, mut sprites, mut ship_resource): Self::SystemData) {
        for (_, transform, sprite_render) in (&ships, &mut transforms, &mut sprites).join() {
            if ship_resource.is_exploding { if true { sprite_render.sprite_number = 5 } return; }
            sprite_render.sprite_number = ship_resource.sprite_nb();
            if let Some(true) = input.action_is_down("power") {
                ship_resource.power(transform.rotation());
            } else {
                ship_resource.apply_gravity();
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

        for (_, transform) in (&ships_parent, &mut transforms).join() {
            if ship_resource.y_force != 0. || ship_resource.x_force != 0. {
                transform.append_translation_xyz(ship_resource.x_force, ship_resource.y_force, 0.);
            }
        }
    }
}