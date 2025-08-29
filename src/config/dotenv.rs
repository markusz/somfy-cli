use crate::config::common::get_config_folder;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "env.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpProtocol {
    Http,
    Https,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CliApiClientConfig {
    pub protocol: Option<HttpProtocol>,
    pub hostname: Option<String>,
    pub port: Option<usize>,
    pub api_key: Option<String>,
    pub cert_path: Option<String>,
}

fn get_file_location() -> PathBuf {
    let mut path = get_config_folder();
    path.push(CONFIG_FILENAME);
    path
}

pub(crate) fn load_config_file() -> anyhow::Result<Option<CliApiClientConfig>> {
    let path = get_file_location();
    let config = std::fs::read(&path);
    match config {
        Ok(val) => {
            let config: CliApiClientConfig = serde_json::from_slice(val.as_slice())
                .context("Failed to parse config file. Check that ~/.config/somfy-cli/env.json contains valid JSON")?;
            Ok(Some(config))
        }
        Err(_) => Ok(None),
    }
}
