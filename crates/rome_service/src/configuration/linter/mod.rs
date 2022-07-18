#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::settings::LinterSettings;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature. `true` by default
    pub enabled: bool,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globals: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            globals: None,
            rules: None,
        }
    }
}

impl From<&LinterConfiguration> for LinterSettings {
    fn from(conf: &LinterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            globals: conf.globals.clone(),
            rules: conf.rules.clone(),
        }
    }
}
