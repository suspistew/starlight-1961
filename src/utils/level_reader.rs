use amethyst::utils::application_root_dir;
use std::fs::File;
use serde_json::from_reader;
use serde::Deserialize;
use amethyst::core::math::Point3;
use std::collections::HashMap;
use crate::utils::sprites::NO_TILE_ID;
use crate::entities::blade_saw::BladeSaw;

pub fn read_level(lvl_number: usize) -> LevelConfig {
    let input_path = format!(
        "assets/levels/level_{}.json",
        lvl_number
    );
    let app_root = application_root_dir().unwrap();
    let input_path = app_root.join(input_path.as_str());
    let f = File::open(&input_path.as_path()).expect("Failed opening file");
    let res: TiledLevel = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load level {}: {}", lvl_number, e);
            std::process::exit(1);
        }
    };

    LevelConfig::new(res)
}


#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    pub height: u32,
    pub width: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub tiles: HashMap<Point3<u32>,usize>,
    pub blade_saws: Vec<BladeSaw>,
    pub text: String
}

impl LevelConfig {
    fn new(level: TiledLevel) -> Self {
        let mut tiles: HashMap<Point3<u32>,usize> = HashMap::new();
        let mut blade_saws: Vec<BladeSaw> = Vec::new();
        for layer in level.layers {
            let z = get_z_from_layer_name(layer.name.as_str());
            if layer.data.is_some() {
                let layer_data = layer.data.unwrap();
                for y in 0..level.height {
                    let line = &layer_data[((y * level.width) as usize)..((y * level.width + level.width) as usize)];
                    for (x, tile) in line.iter().enumerate() {
                        let (tile_x, tile_y, tile_z) = (x as u32, (level.height - y - 1), z as u32);
                        let tile_number: i32 = match *tile {
                            0 => NO_TILE_ID,
                            any => (any as i32),
                        };
                        if tile_number != NO_TILE_ID && tile_number > 0 {
                            tiles.insert(Point3::new(tile_x, tile_y, tile_z), (tile_number - 1) as usize);
                        }
                    }
                }
            }else if layer.objects.is_some(){
                let objects = layer.objects.unwrap();
                for entity in objects{
                    match entity.data_type{
                        DataType::BladeSaw => blade_saws.push(BladeSaw{
                            direction_x:  entity.properties.iter().filter(|e| e.name == "direction_x".to_string()).next().unwrap().value.parse().unwrap(),
                            direction_y: entity.properties.iter().filter(|e| e.name == "direction_y".to_string()).next().unwrap().value.parse().unwrap(),
                            start_x: entity.properties.iter().filter(|e| e.name == "start_x".to_string()).next().unwrap().value.parse().unwrap(),
                            start_y: entity.properties.iter().filter(|e| e.name == "start_y".to_string()).next().unwrap().value.parse().unwrap(),
                            min_x: entity.properties.iter().filter(|e| e.name == "min_x".to_string()).next().unwrap().value.parse().unwrap(),
                            min_y: entity.properties.iter().filter(|e| e.name == "min_y".to_string()).next().unwrap().value.parse().unwrap(),
                            max_y: entity.properties.iter().filter(|e| e.name == "max_y".to_string()).next().unwrap().value.parse().unwrap(),
                            max_x: entity.properties.iter().filter(|e| e.name == "max_x".to_string()).next().unwrap().value.parse().unwrap()
                        })
                    }
                }
            }
        }

        LevelConfig{
            height: level.height,
            width: level.width,
            start_x: level.properties.iter().filter(|e| e.name == "start_x".to_string()).next().unwrap().value.parse().unwrap(),
            start_y: level.properties.iter().filter(|e| e.name == "start_y".to_string()).next().unwrap().value.parse().unwrap(),
            tiles,
            blade_saws,
            text: level.properties.iter().filter(|e| e.name == "text".to_string()).next().unwrap().value.to_string(),
        }
    }
}

fn get_z_from_layer_name(name: &str) -> usize{
    match name {
        "Structures" => 0,
        "Entities" => 1,
        "Interactives" => 2,
        "Background" => 3,
        _ => 99
    }
}

#[derive(Debug, Deserialize)]
pub struct TiledLevel {
    pub height: u32,
    pub width: u32,
    pub layers: Vec<TiledLayer>,
    pub properties: Vec<TiledPropery>
}

#[derive(Debug, Deserialize)]
pub struct TiledPropery{
    pub name: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct TiledLayer{
    data: Option<Vec<usize>>,
    name: String,
    objects: Option<Vec<TiledEntity>>
}

#[derive(Debug, Deserialize)]
pub struct TiledEntity{
    #[serde(rename(deserialize = "type"))]
    pub data_type: DataType,
    pub properties: Vec<TiledPropery>
}

#[derive(Debug, Deserialize)]
pub enum DataType {
    BladeSaw
}