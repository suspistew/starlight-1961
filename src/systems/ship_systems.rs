use amethyst::core::ecs::{System, WriteStorage, ReadStorage, Read, Join, Write, Entities};
use amethyst::core::{Transform, Time};
use crate::entities::ship::{Ship, ShipParent, Thrusters};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use crate::resources::main_resource::MainResource;
use amethyst::core::num::FloatConst;
use std::panic::resume_unwind;
use crate::states::level::TILE_SIZE;
use crate::entities::explosion::Explosion;
use amethyst::assets::Handle;

pub struct ShipSystem;

fn angle_rotation_modifier_bind_to_pi() -> f32 { f32::PI() / 96. }

pub const ANGLE_ROTATION_DEGREE_MODIFIER: f32 = 1.875;


impl<'s> System<'s> for ShipSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, ShipParent>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, MainResource>,
        Read<'s, Time>,
        WriteStorage<'s, Explosion>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, ships, ships_parent, input, mut sprites, mut main_resource, time,mut explosions,  entities): Self::SystemData) {
        for (_, transform, sprite_render) in (&ships, &mut transforms, &mut sprites).join() {
            if main_resource.should_be_reset{ transform.set_rotation_z_axis(0.); break; }
            if main_resource.is_exploding { if true { sprite_render.sprite_number = 5 } return; }
            sprite_render.sprite_number = main_resource.sprite_nb();
            if let Some(true) = input.action_is_down("power") {
                main_resource.power(time.delta_seconds(), transform.rotation());
            } else {
                main_resource.apply_gravity(time.delta_seconds());
            }
            if !main_resource.is_landed {
                if let Some(true) = input.action_is_down("rotate_left") {
                    if transform.rotation().quaternion().k < 0.50 {
                        transform.prepend_rotation_z_axis(-1. * angle_rotation_modifier_bind_to_pi());
                        main_resource.current_rotation_angle -= ANGLE_ROTATION_DEGREE_MODIFIER;
                    }
                }
                if let Some(true) = input.action_is_down("rotate_right") {
                    if transform.rotation().quaternion().k > -0.50 {
                        transform.prepend_rotation_z_axis(angle_rotation_modifier_bind_to_pi());
                        main_resource.current_rotation_angle += ANGLE_ROTATION_DEGREE_MODIFIER;
                    }
                }
            }else{
                if main_resource.current_rotation_angle > 0. {
                    transform.prepend_rotation_z_axis(-1. * angle_rotation_modifier_bind_to_pi());
                    main_resource.current_rotation_angle -= ANGLE_ROTATION_DEGREE_MODIFIER;
                } else if main_resource.current_rotation_angle < 0. {
                    transform.prepend_rotation_z_axis(angle_rotation_modifier_bind_to_pi());
                    main_resource.current_rotation_angle += ANGLE_ROTATION_DEGREE_MODIFIER;
                }
            }
        }

        let mut ship_transform = (0., 0.);

        for (_, transform) in (&ships_parent, &mut transforms).join() {
            ship_transform = (transform.translation().x, transform.translation().y);
            if main_resource.should_be_reset {
                let config = main_resource.level_config();
                transform.set_translation_xyz(
                    config.start_x as f32 * TILE_SIZE - 16.,
                    ((config.height - config.start_y) as f32 * TILE_SIZE),
                    0.04,
                );
                main_resource.reset();
                return;
            }
            if main_resource.y_force != 0. || main_resource.x_force != 0. {
                transform.append_translation_xyz(main_resource.x_force, main_resource.y_force, 0.);
            }
        }

        if main_resource.ship_life <= 0 {
            main_resource.is_exploding = true;
            let mut explosion_transform = Transform::default();
            explosion_transform.set_translation_xyz(ship_transform.0, ship_transform.1, 0.9);
            entities
                .build_entity()
                .with(Explosion, &mut explosions)
                .with(init_sprite_render(main_resource.sprites.as_ref().unwrap().explosion_sprite_render.clone()), &mut sprites)
                .with(explosion_transform, &mut transforms)
                .build();
        }
    }
}

fn init_sprite_render(sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    }
}