use amethyst::core::math::Point3;
use amethyst::core::ecs::{World, WorldExt};
use crate::resources::main_resource::MainResource;
use amethyst_tiles::Tile;

#[derive(Default, Clone)]
pub struct StartLightTile;

impl Tile for StartLightTile{

    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        let level = world.read_resource::<MainResource>();
        match level.level_config().tiles.get(&point) {
            Some(&value) => return Some(value),
            _ => {}
        }

        None
    }
}