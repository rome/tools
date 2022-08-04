#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::settings::LinterSettings;
use rome_console::codespan::Severity;
pub use rules::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    pub enabled: bool,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
        }
    }
}

impl From<LinterConfiguration> for LinterSettings {
    fn from(conf: LinterConfiguration) -> Self {
        Self {
            enabled: conf.enabled,
            rules: conf.rules,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

    pub fn is_disabled(&self) -> bool {
        if let Self::WithOptions(rule) = self {
            rule.level == RulePlainConfiguration::Off
        } else {
            matches!(self, Self::Plain(RulePlainConfiguration::Off))
        }
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }
}
impl Default for RuleConfiguration {
    fn default() -> Self {
        Self::Plain(RulePlainConfiguration::Error)
    }
}

impl From<&RuleConfiguration> for Severity {
    fn from(conf: &RuleConfiguration) -> Self {
        match conf {
            RuleConfiguration::Plain(p) => p.into(),
            RuleConfiguration::WithOptions(conf) => {
                let level = &conf.level;
                level.into()
            }
        }
    }
}

impl From<&RulePlainConfiguration> for Severity {
    fn from(conf: &RulePlainConfiguration) -> Self {
        match conf {
            RulePlainConfiguration::Warn => Severity::Warning,
            RulePlainConfiguration::Error => Severity::Error,
            _ => unreachable!("the rule is turned off, it should not step in here"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    Warn,
    Error,
    Off,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions {
    level: RulePlainConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Box<serde_json::value::RawValue>>,
}
