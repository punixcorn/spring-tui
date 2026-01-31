mod api;
#[allow(dead_code)]
#[allow(unused_imports)]
mod types;
mod cli;
mod config;
mod tui;
mod generator;
#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::parse() {
        None => {
            // Run TUI
            tui::run().await?;
        }
        Some(result) => {
            if let Some(dir) = result.dir {
                config::modify_global_config(dir.as_str());
            }
            if let Some(file) = result.file {
                config::create_project_from_config(file.as_str());
            }
        }
    }
    Ok(())
}
