use crate::entities::explosion::Explosion;
use crate::entities::ship::{Ship, ShipParent};
use crate::resources::main_resource::MainResource;
use crate::utils::sound::{play_explosion, play_fire, Sounds};
use crate::utils::sprites::TILE_SIZE;
use amethyst::assets::{AssetStorage, Handle};
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage,
};
use amethyst::core::num::FloatConst;
use amethyst::core::{Time, Transform};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender, SpriteSheet};

const DEFAULT_FIRE_TIMER: f32 = 0.2;

pub struct ShipSystem {
    pub play_fire_timer: f32,
}

impl Default for ShipSystem {
    fn default() -> Self {
        ShipSystem {
            play_fire_timer: DEFAULT_FIRE_TIMER,
        }
    }
}

fn angle_rotation_modifier_bind_to_pi() -> f32 {
    f32::PI() / 96.
}

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
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            ships,
            ships_parent,
            input,
            mut sprites,
            mut main_resource,
            time,
            mut explosions,
            entities,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        if main_resource.victory {
            return;
        }
        self.play_fire_timer -= time.delta_seconds();
        if main_resource.bullet_hit_timer > 0. {
            main_resource.bullet_hit_timer -= time.delta_seconds();
        }
        for (_, transform, sprite_render) in (&ships, &mut transforms, &mut sprites).join() {
            if main_resource.should_be_reset {
                transform.set_rotation_z_axis(0.);
                break;
            }
            if main_resource.is_exploding {
                if true {
                    sprite_render.sprite_number = 5
                }
                return;
            }
            sprite_render.sprite_number = main_resource.sprite_nb();
            let power = input.action_is_down("power");
            if main_resource.ship_fuel > 0. && power.is_some() && power.unwrap() {
                if self.play_fire_timer <= 0. {
                    play_fire(&*sounds, &storage, audio_output.as_deref());
                    self.play_fire_timer = DEFAULT_FIRE_TIMER;
                }
                main_resource.power(time.delta_seconds(), transform.rotation());
            } else {
                main_resource.apply_gravity(time.delta_seconds());
            }
            if !main_resource.is_landed {
                if let Some(true) = input.action_is_down("rotate_left") {
                    if transform.rotation().quaternion().k < 0.50 {
                        transform
                            .prepend_rotation_z_axis(-1. * angle_rotation_modifier_bind_to_pi());
                        main_resource.current_rotation_angle -= ANGLE_ROTATION_DEGREE_MODIFIER;
                    }
                }
                if let Some(true) = input.action_is_down("rotate_right") {
                    if transform.rotation().quaternion().k > -0.50 {
                        transform.prepend_rotation_z_axis(angle_rotation_modifier_bind_to_pi());
                        main_resource.current_rotation_angle += ANGLE_ROTATION_DEGREE_MODIFIER;
                    }
                }
            } else {
                main_resource.fuel_up(time.delta_seconds());
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
                    (config.height - config.start_y) as f32 * TILE_SIZE,
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
            play_explosion(&*sounds, &storage, audio_output.as_deref());
            main_resource.is_exploding = true;
            let mut explosion_transform = Transform::default();
            explosion_transform.set_translation_xyz(ship_transform.0, ship_transform.1, 0.9);
            entities
                .build_entity()
                .with(Explosion, &mut explosions)
                .with(
                    init_sprite_render(
                        main_resource
                            .sprites
                            .as_ref()
                            .unwrap()
                            .explosion_sprite_render
                            .clone(),
                    ),
                    &mut sprites,
                )
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
