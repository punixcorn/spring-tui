
#[cfg(test)]
mod integration_tests {
    use spring_tui::config::create_project_from_config;
    use std::path::Path;
    use std::fs;

    #[tokio::test]
    async fn test_example_files_generation() {
        // Map of config file to expected output file (zip)
        // Based on the content of the example files (artifactId)
        let examples = vec![
            ("example/rake-service-config.json", "snake.zip"),
            ("example/rake-service-config.yaml", "rake.zip"),
            ("example/security-config.toml", "hopper.zip"),
        ];

        for (config_path, output_file) in examples {
            assert!(Path::new(config_path).exists(), "Example file {} not found in current directory", config_path);
            
            println!("Testing generation from {}", config_path);

            // Run the generation (extract = false -> creates zip)
            let result = create_project_from_config(config_path, false).await;
            
            assert!(result.is_ok(), "Failed to generate project from {}: {:?}", config_path, result.err());

            // Check if output file exists
            let zip_path = Path::new(output_file);
            assert!(zip_path.exists(), "Expected output file {} was not created", output_file);

            // Cleanup
            let _ = fs::remove_file(zip_path);
        }
    }
}