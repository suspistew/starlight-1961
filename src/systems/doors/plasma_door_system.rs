use amethyst::core::ecs::{System, WriteStorage, Entities, Read, Join, Write};
use crate::entities::doors::{PlasmaDoor, DoorState};
use amethyst::renderer::SpriteRender;
use std::collections::HashMap;
use amethyst::core::Time;
use crate::utils::sprites::plasma_doors::{plasma_door_next_sprite, plasma_door_close_sprite};
use crate::resources::main_resource::MainResource;

const TIMING_CHANGE_SPRITE:f32 = 0.1;

pub struct PlasmaDoorSystem {
    sprite_changing_timer: f32,
    door_timers: HashMap<u32, f32>
}


impl Default for PlasmaDoorSystem {
    fn default() -> Self {
        PlasmaDoorSystem {
            sprite_changing_timer: TIMING_CHANGE_SPRITE,
            door_timers: HashMap::new()
        }
    }
}


impl<'s> System<'s> for PlasmaDoorSystem {
    type SystemData = (
        WriteStorage<'s, PlasmaDoor>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Entities<'s>,
        Write<'s, MainResource>);

    fn run(&mut self, (mut doors, mut sprites, time, entities, mut main_resource): Self::SystemData) {
        if main_resource.should_reset_plasma_timers {
            self.sprite_changing_timer= TIMING_CHANGE_SPRITE;
            self.door_timers= HashMap::new();
            main_resource.should_reset_plasma_timers = false;
        }
        self.sprite_changing_timer -= time.delta_seconds();
        for (door, entity, sprite) in (&mut doors, &entities, &mut sprites).join(){
            *self.door_timers.entry(entity.id())
                .or_insert(3.5) -= time.delta_seconds();
            match door.state {
                DoorState::Closed => {

                    if self.sprite_changing_timer <= 0. {
                        sprite.sprite_number = plasma_door_next_sprite(sprite.sprite_number);
                    }
                },
                _ => {}
            };

            let val = self.door_timers.get(&entity.id()).unwrap();
            if val <= &0. {
                self.door_timers.get(&entity.id()).unwrap();
                match door.state {
                    DoorState::Closed => {
                        door.state = DoorState::Open;
                        sprite.sprite_number = plasma_door_close_sprite(sprite.sprite_number);
                        self.door_timers.insert(entity.id(), 1.5);
                    },
                    DoorState::Open => {
                        door.state = DoorState::Closed;
                        sprite.sprite_number = door.initial_sprite;
                        self.door_timers.insert(entity.id(), 3.5);
                    }
                };


            }
        }
        if self.sprite_changing_timer <= 0. {
            self.sprite_changing_timer = TIMING_CHANGE_SPRITE;
        }
    }

}