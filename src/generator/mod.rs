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
use crate::api;

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

pub async fn generate_project(config: &SprintInitConfig) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("Download URL: {}", download_link);

    // Download the project
    let client = reqwest::Client::new();
    let response = client.get(&download_link).send().await?;
    
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        
        // Save to file
        let file_path = format!("{}.zip", config.artifact_id);
        std::fs::write(&file_path, bytes)?;
        println!("Project downloaded to: {}", file_path);
        
        // Extract the zip
        let file = std::fs::File::open(&file_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(".")?;
        println!("Project extracted successfully!");
        
        Ok(())
    } else {
        Err(format!("Download failed with status: {}", response.status()).into())
    }
}