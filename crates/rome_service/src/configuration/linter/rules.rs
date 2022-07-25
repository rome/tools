//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{ConfigurationError, RomeError, RuleConfiguration};
use indexmap::{IndexMap, IndexSet};
use rome_analyze::RuleFilter;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[doc = r" It enables a preset of rules of any group recommended by Rome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<Js>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx: Option<Jsx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<Regex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<Ts>,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            recommended: Some(true),
            js: None,
            jsx: None,
            regex: None,
            ts: None,
        }
    }
}
impl Rules {
    pub(crate) fn is_recommended(&self) -> bool { matches!(self.recommended, Some(true)) }
    #[doc = r" It returns a tuple of filters. The first element of the tuple are the enabled rules,"]
    #[doc = r" while the second element are the disabled rules."]
    #[doc = r""]
    #[doc = r" Only one element of the tuple is [Some] at the time."]
    #[doc = r""]
    #[doc = r" The enabled rules are calculated from the difference with the disabled rules."]
    pub fn as_analysis_filters(
        &self,
    ) -> (Option<IndexSet<RuleFilter>>, Option<IndexSet<RuleFilter>>) {
        let mut enabled_rules = IndexSet::new();
        let mut disabled_rules = IndexSet::new();
        if self.is_recommended() {
            enabled_rules.extend(Js::RECOMMENDED_RULES);
            enabled_rules.extend(Jsx::RECOMMENDED_RULES);
            enabled_rules.extend(Regex::RECOMMENDED_RULES);
            enabled_rules.extend(Ts::RECOMMENDED_RULES);
        }
        if let Some(group) = self.js.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.jsx.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.regex.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.ts.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.js.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.jsx.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.regex.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if let Some(group) = self.ts.as_ref() {
            if group.is_recommended() {
                enabled_rules.extend(&Js::RECOMMENDED_RULES);
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        }
        if enabled_rules.len() > disabled_rules.len() {
            (None, Some(disabled_rules))
        } else {
            (
                Some(enabled_rules.difference(&disabled_rules).cloned().collect()),
                None,
            )
        }
    }
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Js {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_js_rules"
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Js {
    const GROUP_NAME: &'static str = "js";
    pub(crate) const GROUP_RULES: [&'static str; 24] = [
        "noArguments",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noCompareNegZero",
        "noDeadCode",
        "noDebugger",
        "noDelete",
        "noDoubleEquals",
        "noEmptyPattern",
        "noFunctionAssign",
        "noLabelVar",
        "noNegationElse",
        "noShoutyConstants",
        "noSparseArray",
        "noUnnecessaryContinue",
        "noUnsafeNegation",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useSimplifiedLogicExpression",
        "useSingleCaseStatement",
        "useSingleVarDeclarator",
        "useTemplate",
        "useValidTypeof",
        "useWhile",
    ];
    const RECOMMENDED_RULES: [RuleFilter<'static>; 23] = [
        RuleFilter::Rule("js", Self::GROUP_RULES[0]),
        RuleFilter::Rule("js", Self::GROUP_RULES[1]),
        RuleFilter::Rule("js", Self::GROUP_RULES[2]),
        RuleFilter::Rule("js", Self::GROUP_RULES[3]),
        RuleFilter::Rule("js", Self::GROUP_RULES[5]),
        RuleFilter::Rule("js", Self::GROUP_RULES[6]),
        RuleFilter::Rule("js", Self::GROUP_RULES[7]),
        RuleFilter::Rule("js", Self::GROUP_RULES[8]),
        RuleFilter::Rule("js", Self::GROUP_RULES[9]),
        RuleFilter::Rule("js", Self::GROUP_RULES[10]),
        RuleFilter::Rule("js", Self::GROUP_RULES[11]),
        RuleFilter::Rule("js", Self::GROUP_RULES[12]),
        RuleFilter::Rule("js", Self::GROUP_RULES[13]),
        RuleFilter::Rule("js", Self::GROUP_RULES[14]),
        RuleFilter::Rule("js", Self::GROUP_RULES[15]),
        RuleFilter::Rule("js", Self::GROUP_RULES[16]),
        RuleFilter::Rule("js", Self::GROUP_RULES[17]),
        RuleFilter::Rule("js", Self::GROUP_RULES[18]),
        RuleFilter::Rule("js", Self::GROUP_RULES[19]),
        RuleFilter::Rule("js", Self::GROUP_RULES[20]),
        RuleFilter::Rule("js", Self::GROUP_RULES[21]),
        RuleFilter::Rule("js", Self::GROUP_RULES[22]),
        RuleFilter::Rule("js", Self::GROUP_RULES[23]),
    ];
    pub(crate) fn is_recommended(&self) -> bool { matches!(self.recommended, Some(true)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
}
fn deserialize_js_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Js::GROUP_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::DeserializationError(format!(
                    "Invalid rule name `{rule_name}`"
                )),
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Jsx {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_jsx_rules"
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Jsx {
    const GROUP_NAME: &'static str = "jsx";
    pub(crate) const GROUP_RULES: [&'static str; 3] = [
        "noCommentText",
        "noImplicitBoolean",
        "useSelfClosingElements",
    ];
    const RECOMMENDED_RULES: [RuleFilter<'static>; 3] = [
        RuleFilter::Rule("jsx", Self::GROUP_RULES[0]),
        RuleFilter::Rule("jsx", Self::GROUP_RULES[1]),
        RuleFilter::Rule("jsx", Self::GROUP_RULES[2]),
    ];
    pub(crate) fn is_recommended(&self) -> bool { matches!(self.recommended, Some(true)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
}
fn deserialize_jsx_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Jsx::GROUP_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::DeserializationError(format!(
                    "Invalid rule name `{rule_name}`"
                )),
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Regex {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_regex_rules"
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Regex {
    const GROUP_NAME: &'static str = "regex";
    pub(crate) const GROUP_RULES: [&'static str; 1] =
        ["noMultipleSpacesInRegularExpressionLiterals"];
    const RECOMMENDED_RULES: [RuleFilter<'static>; 1] =
        [RuleFilter::Rule("regex", Self::GROUP_RULES[0])];
    pub(crate) fn is_recommended(&self) -> bool { matches!(self.recommended, Some(true)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
}
fn deserialize_regex_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Regex::GROUP_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::DeserializationError(format!(
                    "Invalid rule name `{rule_name}`"
                )),
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Ts {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_ts_rules"
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Ts {
    const GROUP_NAME: &'static str = "ts";
    pub(crate) const GROUP_RULES: [&'static str; 1] = ["useShorthandArrayType"];
    const RECOMMENDED_RULES: [RuleFilter<'static>; 1] =
        [RuleFilter::Rule("ts", Self::GROUP_RULES[0])];
    pub(crate) fn is_recommended(&self) -> bool { matches!(self.recommended, Some(true)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::GROUP_NAME, key))
            } else {
                None
            }
        }))
    }
}
fn deserialize_ts_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Ts::GROUP_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::DeserializationError(format!(
                    "Invalid rule name `{rule_name}`"
                )),
            )));
        }
    }
    Ok(value)
}
