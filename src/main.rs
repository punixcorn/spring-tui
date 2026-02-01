mod api;
mod types;
mod cli;
mod config;
mod tui;
mod generator;
use std::io::Write;

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::parse() {
        None => {
            // Run TUI
            tui::run().await?;
        }
        Some(result) => {
            // if let Some(dir) = result.dir {
            //     config::modify_global_config(dir.as_str());
            // }
            if let Some(file) = result.file {
                let file_clone = file.clone();
                let spinner_handle = tokio::spawn(async move {
                    let chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                    let mut i = 0;
                    loop {
                        print!("\r\x1b[32m{}\x1b[0m Generating project from {}...", chars[i], file_clone);
                        std::io::stdout().flush().unwrap();
                        i = (i + 1) % chars.len();
                        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
                    }
                });

                let res = config::create_project_from_config(file.as_str(), result.extract.unwrap_or(false)).await;
                
                spinner_handle.abort();
                // Clear the spinner line
                print!("\r\x1b[2K");
                std::io::stdout().flush().unwrap();

                match res {
                     Ok(_) => { println!("\x1b[32m✓ Project generated successfully!\x1b[0m"); },
                     Err(e) => {
                         println!("\x1b[31m✗ Error generating project from config: {}\x1b[0m", e);
                     }
                }
            }
        }
    }
    Ok(())
}
