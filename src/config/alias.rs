use std::path::PathBuf;
use crate::config::config::get_config_folder;

const CONFIG_LOCATION_FILENAME: &str = "alias.json";

fn get_file_location() -> PathBuf {
    let mut path = get_config_folder();
    path.push(CONFIG_LOCATION_FILENAME);
    path
}

pub(crate) fn ensure_alias_file() {
    
}