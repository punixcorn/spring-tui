mod util;
use crate::types;
use crate::types::api::{InitializrCapabilities, InitializrDependencies};
use crate::api::util::{get_base_url, get_headers};
use std::error::Error;

pub async fn get_dependencies() -> Result<InitializrDependencies, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(get_base_url() + "dependencies")
        .headers(get_headers())
        .send()
        .await?;

    if response.status().is_success() {
        let deps = response.json::<InitializrDependencies>().await?;
        Ok(deps)
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

pub async fn get_capabilities() -> Result<InitializrCapabilities, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(get_base_url())
        .headers(get_headers())
        .send()
        .await?;

    if response.status().is_success() {
        let capabilities = response.json::<InitializrCapabilities>().await?;
        Ok(capabilities)
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
