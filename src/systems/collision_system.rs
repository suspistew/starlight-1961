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
use crate::entities::canons::Bullet;
use crate::entities::doors::{PlasmaDoor, DoorState};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Colliders>,
        ReadStorage<'s, LandingPlatform>,
        ReadStorage<'s, PlasmaDoor>,
        ReadStorage<'s, ShipParent>,
        WriteStorage<'s, Transform>,
        Write<'s, MainResource>,
        WriteStorage<'s, Explosion>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Bullet>,
        Entities<'s>
    );

    fn run(&mut self, (colliders, landing_plateforms,plasma_doors, ships, mut transforms, mut ship_resource, mut explosions, mut sprite_renders, bullets, entities): Self::SystemData) {
        for (_ship, transform) in (&ships, &transforms).join() {
            let ship_polygon = ship_resource.get_colliders_polygons(transform.translation().x, transform.translation().y);
            for (collider, _, _, _) in (&colliders, !&landing_plateforms, !&bullets, !&plasma_doors).join() {
                let struct_polygons = collider.polygons();
                if !ship_resource.is_exploding && are_colliding(&ship_polygon, struct_polygons) {
                    ship_resource.ship_life  -= ship_resource.ship_life;
                }
            }
            for (collider, door) in (&colliders, &plasma_doors).join() {
                match door.state {
                    DoorState::Closed=> {
                        let struct_polygons = collider.polygons();
                        if !ship_resource.is_exploding && are_colliding(&ship_polygon, struct_polygons) {
                            ship_resource.ship_life -= ship_resource.ship_life;
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn init_sprite_render(sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    }
}

