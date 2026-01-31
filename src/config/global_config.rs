#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::util;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::types::config::GlobalConfig;

/// Read GlobalConfig from config file
pub fn get_config() -> anyhow::Result<GlobalConfig> {
    let path = match util::get_application_config_path() {
        Some(data) => data,
        _ => anyhow::bail!("fuck"),
    };

    util::parse_config(path.as_str())
}

/// set the global config ( overwrite or edit variables )
pub fn set(config: &GlobalConfig) {}
