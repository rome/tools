#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::settings::LinterSettings;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature. `true` by default
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: None,
        }
    }
}

impl From<&LinterConfiguration> for LinterSettings {
    fn from(conf: &LinterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            rules: conf.rules.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RuleConfiguration {
    Plain(RulePlainConfiguration),
    WithOptions(RuleWithOptions),
}
impl RuleConfiguration {
    pub fn is_err(&self) -> bool {
        if let Self::WithOptions(rule) = self {
            rule.level == RulePlainConfiguration::Error
        } else {
            matches!(self, Self::Plain(RulePlainConfiguration::Error))
        }
    }
}
impl Default for RuleConfiguration {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    Warn,
    Error,
    Off,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions {
    level: RulePlainConfiguration,
    options: Value,
}
