#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::util;
use serde::{Deserialize, Serialize};
use crate::types::generic::SprintInitConfig;

pub fn parse_config(path: String) -> anyhow::Result<SprintInitConfig> {
    util::parse_config(path.as_str())
}
