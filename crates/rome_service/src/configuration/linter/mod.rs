#[rustfmt::skip]
mod rules;

pub use crate::configuration::linter::rules::Rules;
use crate::settings::LinterSettings;
use crate::{ConfigurationError, MatchOptions, Matcher, WorkspaceError};
use indexmap::IndexSet;
use rome_diagnostics::Severity;
pub use rules::*;
#[cfg(feature = "schemars")]
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    pub enabled: bool,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub ignore: Option<IndexSet<String>>,
}

impl LinterConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["enabled", "rules", "ignore"];
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Some(Rules::default()),
            ignore: None,
        }
    }
}

impl TryFrom<LinterConfiguration> for LinterSettings {
    type Error = WorkspaceError;

    fn try_from(conf: LinterConfiguration) -> Result<Self, Self::Error> {
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        if let Some(ignore) = conf.ignore {
            for pattern in ignore {
                matcher.add_pattern(&pattern).map_err(|err| {
                    WorkspaceError::Configuration(ConfigurationError::new_invalid_ignore_pattern(
                        pattern.to_string(),
                        err.msg.to_string(),
                    ))
                })?;
            }
        }
        Ok(Self {
            enabled: conf.enabled,
            rules: conf.rules,
            ignored_files: matcher,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RulePlainConfiguration {
    Warn,
    Error,
    Off,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOptions {
    level: RulePlainConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(schema_with = "schema_any"))]
    pub options: Option<Value>,
}

#[cfg(feature = "schemars")]
fn schema_any(_gen: &mut SchemaGenerator) -> Schema {
    Schema::Bool(true)
}
