
use serde::{Serialize,Deserialize}; 

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SprintInitConfig {
    /// gradle-kotline, gradle-groovy , gradle-mavn
    pub project_type: String,
    /// java, kotlin, grovvyy
    pub language: String,
    /// 5.0.5
    pub platform_version: String,
    /// packageing : jar / war
    pub packaging: String,
    ///configurationFileFormat: properties/yaml
    pub configuration_file_format: String,
    /// 25,21,17
    pub java_version: i32,
    /// com.example
    pub group_id: String,
    /// demo
    pub artifact_id: String,
    /// demo
    pub name: String,
    ///=Demo%20project%20for%20Spring%20Boot
    pub description: String,
    /// com.exmaple.demo = groupId + name
    pub package_name: String,
    ///lombok,devtools
    pub dependencies: String,
    /// boot version : 2.4.5 SNAPSHOT
    pub boot_version: String,
    /// version : 0.0.1-SNAPSHOT
    pub version: String,
}
