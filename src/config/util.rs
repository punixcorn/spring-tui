use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use crate::types::config::{ConfigMetadata, FileType};

pub fn parse_config<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    let metadata = ConfigMetadata::from_path(path.to_string())?;

    match metadata.file_type {
        FileType::Yaml => Ok(serde_yaml::from_str(&content)?),
        FileType::Json => Ok(serde_json::from_str(&content)?),
        FileType::Toml => Ok(toml::from_str(&content)?),
    }
}

pub fn read_file(path: String) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn get_application_config_path() -> Option<String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let config_dir = PathBuf::from(home).join(".config/spring-tui");

    for ext in ["json", "toml", "yml", "yaml"] {
        let path = config_dir.join(format!("config.{}", ext));
        if path.exists() {
            return path.to_str().map(String::from);
        }
    }

    None
}
