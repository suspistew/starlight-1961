use amethyst::core::ecs::{System, WriteStorage, Write, ReadStorage, Read, Join, ReadExpect};
use crate::entities::bonus::{Bonus, BonusKind};
use amethyst::core::{Transform, Time};
use crate::resources::main_resource::MainResource;
use crate::entities::ship::ShipParent;
use crate::utils::sprites::sprite_to_entities::init_bonus_collider;
use crate::entities::collision::are_colliding;
use amethyst::renderer::SpriteRender;
use crate::utils::sprites::plasma_doors::EMPTY;
use amethyst::assets::AssetStorage;
use crate::utils::sound::{Sounds, play_bonus};
use amethyst::audio::output::Output;
use amethyst::audio::Source;

const DEFAULT_CHANGE_DIRECTION_TIMER :f32 = 0.6;

pub struct BonusSystem{
    direction_y: f32,
    change_direction_timer: f32
}

impl Default for BonusSystem{
    fn default() -> Self {
        BonusSystem{
            direction_y: 15.,
            change_direction_timer: DEFAULT_CHANGE_DIRECTION_TIMER
        }
    }
}

impl<'s> System<'s> for BonusSystem {
    type SystemData = (
        WriteStorage<'s, Bonus>,
        WriteStorage<'s, Transform>,
        Write<'s, MainResource>,
        ReadStorage<'s, ShipParent>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (mut bonuses, mut transforms, mut main_resource, ships, mut sprites, time, storage, sounds, audio_output): Self::SystemData) {
        self.change_direction_timer -= time.delta_seconds();
        if self.change_direction_timer <= 0. {
            self.change_direction_timer = DEFAULT_CHANGE_DIRECTION_TIMER;
            self.direction_y *= -1.;
        }
        let mut ship_polygon = Vec::new();
        for (_ship, transform) in (&ships, &transforms).join() {
            ship_polygon = main_resource.get_colliders_polygons_for_collision(transform.translation().x, transform.translation().y);
        }
        for (bonus, transform, sprite) in (&mut bonuses, &mut transforms, &mut sprites ).join(){
            transform.append_translation_xyz(0., self.direction_y * time.delta_seconds(), 0.);
            let (x, y) = (transform.translation().x, transform.translation().y);
            let collider = init_bonus_collider(&bonus.kind, x, y);
            if main_resource.should_reset_bonuses{
                sprite.sprite_number = bonus.initial_sprite;
                bonus.taken = false;
            }else if !bonus.taken &&  are_colliding( &ship_polygon, collider.polygons()){
                match bonus.kind {
                    BonusKind::Fuel => {
                        main_resource.bonus_fuel();
                    },
                    BonusKind::Wrench => {
                        main_resource.bonus_heal();
                        main_resource.bonus_coin();
                    },
                    BonusKind::Coin => {
                        main_resource.bonus_coin();
                    }
                }
                play_bonus(&*sounds, &storage, audio_output.as_deref());
                bonus.taken = true;
                sprite.sprite_number= EMPTY;
            }
        }
        main_resource.should_reset_bonuses = false;
    }
}
