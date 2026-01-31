use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializrDependencies {
    pub boot_version: String,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    group_id: String,
    artifact_id: String,
    scope: String,
    bom: Option<String>,
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
pub struct Link {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CapabilityGroup {
    #[serde(rename = "type")]
    pub capability_type: String,
    pub default: Option<String>,
    pub values: Vec<CapabilityValue>,
}

#[derive(Debug, Deserialize)]
pub struct CapabilityValue {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub action: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct TextCapability {
    #[serde(rename = "type")]
    pub capability_type: String,
    pub default: String,
}