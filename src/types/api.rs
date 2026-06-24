use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct InitializrDependencies {
    pub boot_version: String,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub scope: String,
    pub bom: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializrCapabilities {
    #[serde(rename = "_links")]
    pub links: HashMap<String, Link>,
    pub packaging: Option<CapabilityGroup>,
    pub java_version: Option<CapabilityGroup>,
    pub language: Option<CapabilityGroup>,
    pub boot_version: Option<CapabilityGroup>,
    #[serde(rename = "type")]
    pub project_type: Option<CapabilityGroup>,
    pub group_id: Option<TextCapability>,
    pub artifact_id: Option<TextCapability>,
    pub version: Option<TextCapability>,
    pub name: Option<TextCapability>,
    pub description: Option<TextCapability>,
    pub package_name: Option<TextCapability>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Link {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CapabilityGroup {
    #[serde(rename = "type")]
    pub capability_type: String,
    pub default: Option<String>,
    pub values: Vec<CapabilityValue>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CapabilityValue {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub action: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TextCapability {
    #[serde(rename = "type")]
    pub capability_type: String,
    pub default: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::InitializrCapabilities;

    #[test]
    fn capabilities_accept_text_fields_without_defaults() {
        let json = r#"{
            "_links": {
                "maven-project": {
                    "href": "https://start.spring.io/starter.zip?type=maven-project",
                    "templated": true
                }
            },
            "name": { "type": "text" },
            "description": { "type": "text" }
        }"#;

        let capabilities: InitializrCapabilities = serde_json::from_str(json).unwrap();

        assert_eq!(capabilities.name.unwrap().default, None);
        assert_eq!(capabilities.description.unwrap().default, None);
    }
}
