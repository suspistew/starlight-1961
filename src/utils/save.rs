use amethyst::utils::application_root_dir;
use serde_json::{from_reader, to_string_pretty};
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader};
use std::path::Path;
use serde::Deserialize;
use serde::Serialize;

const SAVE_PATH: &str = "assets/.starlight";
#[derive(Deserialize, Serialize, Debug)]
pub struct StarlightSave{
    pub save: usize
}

pub fn read_saved_level() -> Option<StarlightSave>{
    let app_root = application_root_dir().unwrap();
    let input_path = app_root.join(SAVE_PATH);
    let path = Path::new(&input_path);
    let file_already_exist = path.exists();
    if file_already_exist{
        let file = File::open(path);
        if file.is_ok() {
            let reader = BufReader::new(file.unwrap());
            return match from_reader(reader) {
                Ok(save) => save,
                Err(e) => None,
            };
        }
    }
    None
}

pub fn save_progress(level_number: usize){
    let app_root = application_root_dir().unwrap();
    let input_path = app_root.join(SAVE_PATH);
    let path = Path::new(&input_path);
    let file_already_exist = path.exists();
    if !file_already_exist || fs::remove_file(path).is_ok() {
        if let Ok(mut target_file) = File::create(path) {
            let to_save = to_string_pretty(&StarlightSave{ save: level_number }).unwrap();
            target_file
                .write_all(to_save.to_string().as_bytes());
        }
    }

}