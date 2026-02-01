use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GlobalConfig {
    pub dir: String,
}

impl GlobalConfig {}

pub enum FileType {
    Yaml,
    Json,
    Toml,
}

// will add file path later
pub struct ConfigMetadata {
    pub file_type: FileType,
}

impl ConfigMetadata {
    pub fn from_path(file_path: String) -> anyhow::Result<Self> {
        let extension = Path::new(&file_path)
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("No file extension found"))?;

        let current_file_type = match extension {
            "yaml" | "yml" => FileType::Yaml,
            "json" => FileType::Json,
            "toml" => FileType::Toml,
            _ => anyhow::bail!("Unsuppored file type"),
        };

        Ok(Self {
            file_type: current_file_type,
        })
    }
}
