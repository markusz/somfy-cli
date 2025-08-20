use std::path::PathBuf;
const CONFIG_FOLDER: &str = ".somfy_cli";

pub fn get_config_folder() -> PathBuf {
    let mut path = PathBuf::new();
    let home = dirs::home_dir().unwrap_or(PathBuf::from("."));
    path.push(home);
    path.push(CONFIG_FOLDER);

    path
}