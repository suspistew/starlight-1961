use amethyst::core::ecs::{System, ReadStorage, Write, Join, Entities, WriteStorage};
use crate::entities::collision::{Colliders, LandingPlatform, are_colliding};
use crate::entities::ship::ShipParent;
use amethyst::core::Transform;
use crate::resources::main_resource::MainResource;
use geo::algorithm::intersects::Intersects;
use geo::Polygon;
use crate::entities::explosion::Explosion;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::assets::Handle;
use amethyst::core::ecs::storage::MaskedStorage;
use amethyst_tiles::{TileMap, MortonEncoder2D};
use crate::utils::starlight_tile::StartLightTile;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, ShipParent>,
        WriteStorage<'s, Transform>,
        Write<'s, MainResource>,
        WriteStorage<'s, Explosion>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        ReadStorage<'s, TileMap<StartLightTile, MortonEncoder2D>>
    );

    fn run(&mut self, (colliders, landing_plateforms, ships, mut transforms, mut ship_resource, mut explosions, mut sprite_renders, entities, tilemap): Self::SystemData) {
        let mut explosion_information = (false, 0., 0.);
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
            for (collider, _) in (&colliders, !&landing_plateforms).join() {
                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding && are_colliding(&ship_polygon, struct_polygons) {
                    ship_resource.is_exploding = true;
                    explosion_information = (true, transform.translation().x, transform.translation().y);
                }
            }
        }

        if explosion_information.0 {
            let mut explosion_transform = Transform::default();
            explosion_transform.set_translation_xyz(explosion_information.1, explosion_information.2, 0.9);
            entities
                .build_entity()
                .with(Explosion, &mut explosions)
                .with(init_sprite_render(ship_resource.sprites.as_ref().unwrap().explosion_sprite_render.clone()), &mut sprite_renders)
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

