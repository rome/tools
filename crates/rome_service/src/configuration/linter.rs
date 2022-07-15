use crate::settings::LinterSettings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct LinterConfiguration {
    // if `false`, it disables the feature. `true` by default
    pub enabled: bool,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl From<&LinterConfiguration> for LinterSettings {
    fn from(conf: &LinterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
        }
    }
}
