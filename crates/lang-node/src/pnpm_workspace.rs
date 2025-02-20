use crate::PNPM;
use cached::proc_macro::cached;
use moon_error::MoonError;
use moon_lang::config_cache;
use moon_utils::fs::sync::read_yaml;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

config_cache!(PnpmWorkspace, PNPM.config_filenames[2], read_yaml);

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PnpmWorkspace {
    pub packages: Vec<String>,

    #[serde(flatten)]
    pub unknown: HashMap<String, Value>,

    #[serde(skip)]
    pub path: PathBuf,
}
