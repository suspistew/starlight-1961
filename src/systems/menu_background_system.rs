use crate::entities::main_menu::{MenuBackground, PushEnter};
use crate::entities::ship::Ship;
use crate::entities::sound::MenuSound;
use crate::utils::sound::{play_bonus, Sounds};
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::core::{Time, Transform};
use amethyst::renderer::SpriteRender;
use amethyst::ui::UiImage;
use rand::Rng;

pub struct MenuBackgroundSystem {
    blink_delay: f32,
}

impl Default for MenuBackgroundSystem {
    fn default() -> Self {
        MenuBackgroundSystem { blink_delay: 0.5 }
    }
}

impl<'s> System<'s> for MenuBackgroundSystem {
    type SystemData = (
        ReadStorage<'s, MenuBackground>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, PushEnter>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, UiImage>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadStorage<'s, MenuSound>,
    );

    fn run(
        &mut self,
        (
            backgrounds,
            mut transforms,
            ships,
            pushs,
            mut sprites,
            mut images,
            time,
            mut entities,
            storage,
            sounds,
            audio_output,
            menu_sounds,
        ): Self::SystemData,
    ) {
        for (background, transform) in (&backgrounds, &mut transforms).join() {
            transform.append_translation_xyz(
                0.,
                (-7. * background.parallax_index as f32) * time.delta_seconds(),
                0.,
            );
            if transform.translation().y <= -800. {
                transform.append_translation_xyz(0., 1600., 0.);
            }
        }
        for (_, sprite) in (&ships, &mut sprites).join() {
            sprite.sprite_number = rand::thread_rng().gen_range(0, 3) as usize
        }

        self.blink_delay -= time.delta_seconds();

        if self.blink_delay <= 0. {
            for (image, _) in (&mut images, &pushs).join() {
                match image {
                    UiImage::Sprite(sprite) => {
                        if sprite.sprite_number == 1 {
                            sprite.sprite_number = 0;
                        } else {
                            sprite.sprite_number = 1;
                        }
                    }
                    _ => {}
                }
            }
            self.blink_delay = 0.5;
        }

        for (entity, sound) in (&*entities, &menu_sounds).join() {
            entities.delete(entity);
            play_bonus(&*sounds, &storage, audio_output.as_deref());
        }
    }
}
