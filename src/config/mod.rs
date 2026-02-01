mod config_parser;
mod global_config;
mod util;
use crate::generator;

pub async fn create_project_from_config(path: &str, extract: bool) -> anyhow::Result<()> {
    let config = config_parser::parse_config(path.to_string())?;
    generator::generate_project(&config, extract)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
