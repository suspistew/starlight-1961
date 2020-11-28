use amethyst::core::ecs::{System, Read, ReadStorage, WriteStorage, Join};
use crate::resources::main_resource::MainResource;
use crate::entities::ship::{ShipPowerLeftNumber, ShipLife, ShipFuel, ShipPowerRightNumber, Coin};
use amethyst::ui::UiImage;
use core::cmp;

pub struct UISystem;

impl<'s> System<'s> for UISystem {
    type SystemData = (
        Read<'s, MainResource>,
        ReadStorage<'s, ShipPowerLeftNumber>,
        ReadStorage<'s, ShipPowerRightNumber>,
        WriteStorage<'s, UiImage>,
        ReadStorage<'s, ShipLife>,
        ReadStorage<'s, Coin>,
        ReadStorage<'s, ShipFuel>,
    );

    fn run(&mut self, (main_resource, left_powers, right_powers, mut ui_images, lifes, coins, fuels): Self::SystemData) {
        let (left, right) = format_force(main_resource.x_force, main_resource.y_force);
        for (image, _) in (&mut ui_images, &left_powers).join() {
            match image {
                UiImage::Sprite(sprite) => { sprite.sprite_number = left; },
                _ => {}
            }
        }
        for (image, _) in (&mut ui_images, &right_powers).join() {
            match image {
                UiImage::Sprite(sprite) => { sprite.sprite_number = right; },
                _ => {}
            }
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

        for (coin, image) in (&coins, &mut ui_images).join() {
            match image {
                UiImage::Sprite(sprite) => {
                    if main_resource.collected_coin < coin.coin_id  {
                        sprite.sprite_number = 87;
                    }else{
                        sprite.sprite_number = 97;
                    }
                },
                _ => {}
            }

        }

    }
}

fn format_force(xf: f32, yf: f32) -> (usize, usize) {
    let speed = cmp::max(((yf.abs()) * 10.) as u32, (xf.abs() * 10.) as u32).to_string();
    if speed.len() > 1 {
        return (speed[..1].parse().unwrap(), speed[1..2].parse().unwrap());
    }
    (0, speed.parse().unwrap())
}