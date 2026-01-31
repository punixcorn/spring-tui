mod config_parser;
mod global_config;
mod util;
use crate::generator;

pub async fn create_project_from_config(path: &str) -> anyhow::Result<()> {
    let config = config_parser::parse_config(path.to_string())?;
    match generator::generate_project(&config).await {
        Ok(_) => println!("Project generated successfully."),
        Err(e) => println!("Failed to generate project: {}", e),
    };
    Ok(())
}

pub fn modify_global_config(path: &str) {
    unimplemented!( "Function to modify global config at path: {}", path);
}
