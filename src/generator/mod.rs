// make does the logic of coverting the Types into the desired url to be downloaded
// creates a struct SprintInitConfig and parses it into the url from InitializrCapabilities.links based on the build
/* InitializrCapabilities {
    links: {
        "gradle-project-kotlin": Link {
            href: "https://start.spring.io/starter.zip?type=gradle-project-kotlin{&dependencies,packaging,javaVersion,language,bootVersion,groupId,artifactId,version,name,description,packageName,configurationFileFormat}",
            templated: Some(
                true,
            ),
        },
        "gradle-project": Link {
            href: "https://start.spring.io/starter.zip?type=gradle-project{&dependencies,packaging,javaVersion,language,bootVersion,groupId,artifactId,version,name,description,packageName,configurationFileFormat}",
            templated: Some(
                true,
            ),
        },
        "gradle-build": Link {
            href: "https://start.spring.io/build.gradle?type=gradle-build{&dependencies,packaging,javaVersion,language,bootVersion,groupId,artifactId,version,name,description,packageName,configurationFileFormat}",
            templated: Some(
                true,
            ),
        },
        "maven-project": Link {
            href: "https://start.spring.io/starter.zip?type=maven-project{&dependencies,packaging,javaVersion,language,bootVersion,groupId,artifactId,version,name,description,packageName,configurationFileFormat}",
            templated: Some(
                true,
            ),
        },
        "maven-build": Link {
            href: "https://start.spring.io/pom.xml?type=maven-build{&dependencies,packaging,javaVersion,language,bootVersion,groupId,artifactId,version,name,description,packageName,configurationFileFormat}",
            templated: Some(
                true,
            ),
        },
*/


use crate::types::generic::SprintInitConfig;
use crate::{api, types};

pub fn download_url(config: &SprintInitConfig, base_url: &str) -> String {
    let mut url = base_url.to_string();

    // Append query parameters based on the config
    url.push_str(&format!("&dependencies={}", config.dependencies));
    url.push_str(&format!("&packaging={}", config.packaging));
    url.push_str(&format!("&javaVersion={}", config.java_version));
    url.push_str(&format!("&language={}", config.language));
    url.push_str(&format!("&bootVersion={}", config.boot_version));
    url.push_str(&format!("&groupId={}", config.group_id));
    url.push_str(&format!("&artifactId={}", config.artifact_id));
    url.push_str(&format!("&version={}", config.version));
    url.push_str(&format!("&name={}", config.name));
    url.push_str(&format!("&description={}", config.description));
    url.push_str(&format!("&packageName={}", config.package_name));
    url.push_str(&format!("&configurationFileFormat={}", config.configuration_file_format));

    url
}

pub async fn generate_project(config: &SprintInitConfig, extract_project: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Get capabilities from API
    let capabilities = api::get_capabilities().await?;
    
    // Get the base URL from the capabilities based on project type
    let base_url = capabilities
        .links
        .get(&config.project_type)
        .ok_or("Unsupported project type")?
        .href
        .split('{')
        .next()
        .ok_or("Invalid URL format")?;

    let download_link = download_url(config, base_url);
    // println!("Download URL: {}", download_link);

    // Download the project
    let client = reqwest::Client::new();
    let response = client.get(&download_link).send().await?;
    
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        
        // Save to file
        let file_path = format!("{}.zip", config.artifact_id);
        std::fs::write(&file_path, bytes)?;
        // println!("Project downloaded to: {}", file_path);
        
        // Extract the zip
        if extract_project {
            let file = std::fs::File::open(&file_path)?;
            let mut archive = zip::ZipArchive::new(file)?;
            archive.extract(".")?;
            println!("Project extracted successfully!");
        }
        
        Ok(())
    } else {
        let error_text = response.text().await.unwrap_or_default();
        let error_response: Result<types::generic::ErrorResponse, _> = serde_json::from_str(&error_text);
        if let Ok(err) = error_response {
            Err(format!("Download failed: {}",err.message).into())
        } else {  
            Err(format!("Download failed: {}", error_text).into())
        }
    }
}

pub fn generate_project_config_file(
    config: &SprintInitConfig,
    extension: types::config::FileType,
    custom_filename: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ext_str = match extension {
        types::config::FileType::Yaml => "yaml",
        types::config::FileType::Json => "json",
        types::config::FileType::Toml => "toml",
    };

    let filename = if let Some(name) = custom_filename {
        if name.ends_with(&format!(".{}", ext_str)) {
            name
        } else {
            format!("{}.{}", name, ext_str)
        }
    } else {
        format!("config.{}", ext_str)
    };

    let content = match extension {
        types::config::FileType::Yaml => serde_yaml::to_string(config)?,
        types::config::FileType::Json => serde_json::to_string_pretty(config)?,
        types::config::FileType::Toml => toml::to_string_pretty(config)?,
    };

    std::fs::write(filename, content)?;
    Ok(())
}