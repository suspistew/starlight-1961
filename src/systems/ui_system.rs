use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use crate::resources::main_resource::MainResource;
use crate::entities::ship::{ShipPower, ShipLife, ShipFuel};
use amethyst::ui::{UiText, UiImage};
use core::cmp;

pub struct UISystem;

impl<'s> System<'s> for UISystem {
    type SystemData = (
        Read<'s, MainResource>,
        ReadStorage<'s, ShipPower>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiImage>,
        ReadStorage<'s, ShipLife>,
        ReadStorage<'s, ShipFuel>,
    );

    fn run(&mut self, (main_resource, powers, mut ui_texts, mut ui_images, lifes, fuels): Self::SystemData) {
        for (ui_text, _) in (&mut ui_texts, &powers).join() {
            ui_text.text = format_force(main_resource.x_force, main_resource.y_force).to_string();
        }

        for (fuel, image) in (&fuels, &mut ui_images).join() {
            match image {
                UiImage::Sprite(sprite) => {
                    if main_resource.ship_fuel / 48. < fuel.fuel_point as f32 {
                        sprite.sprite_number = 1;
                    }else{
                        sprite.sprite_number = 0;
                    }
                },
                _ => {}
            }

        }

        for (life, image) in (&lifes, &mut ui_images).join() {
            match image {
                UiImage::Sprite(sprite) => {
                    if main_resource.ship_life < life.life_point  {
                        sprite.sprite_number = 1;
                    }else{
                        sprite.sprite_number = 0;
                    }
                },
                _ => {}
            }

        }

    }
}

fn format_force(xf: f32, yf: f32) -> u32 {
    cmp::max(((yf.abs()) * 10.) as u32, (xf.abs() * 10.) as u32)
}