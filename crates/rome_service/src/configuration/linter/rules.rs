//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{ConfigurationError, RomeError, RuleConfiguration};
use indexmap::{IndexMap, IndexSet};
use rome_analyze::RuleFilter;
use rome_console::codespan::Severity;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[doc = r" It enables the lint rules recommended by Rome. `true` by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correctness: Option<Correctness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nursery: Option<Nursery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            recommended: Some(true),
            correctness: None,
            nursery: None,
            style: None,
        }
    }
}
impl Rules {
    #[doc = r" Checks if the code coming from [rome_diagnostics::Diagnostic] corresponds to a rule."]
    #[doc = r" Usually the code is built like {category}/{rule_name}"]
    pub fn matches_diagnostic_code<'a>(
        &self,
        category: Option<&'a str>,
        rule_name: Option<&'a str>,
    ) -> Option<(&'a str, &'a str)> {
        match (category, rule_name) {
            (Some(category), Some(rule_name)) => match category {
                "correctness" => Correctness::has_rule(rule_name).then(|| (category, rule_name)),
                "nursery" => Nursery::has_rule(rule_name).then(|| (category, rule_name)),
                "style" => Style::has_rule(rule_name).then(|| (category, rule_name)),
                _ => None,
            },
            _ => None,
        }
    }
    #[doc = r" Given a code coming from [Diagnostic](rome_diagnostics::Diagnostic), this function returns"]
    #[doc = r" the [Severity](rome_diagnostics::Severity) associated to the rule, if the configuration changed it."]
    #[doc = r""]
    #[doc = r" If not, the function returns [None]."]
    pub fn get_severity_from_code(&self, code: &str) -> Option<Severity> {
        let mut split_code = code.split('/');
        let group = split_code.next();
        let rule_name = split_code.next();
        if let Some((group, rule_name)) = self.matches_diagnostic_code(group, rule_name) {
            let severity = match group {
                "correctness" => self
                    .correctness
                    .as_ref()
                    .and_then(|correctness| correctness.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Correctness::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    }),
                "nursery" => self
                    .nursery
                    .as_ref()
                    .and_then(|nursery| nursery.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Nursery::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    }),
                "style" => self
                    .style
                    .as_ref()
                    .and_then(|style| style.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Style::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    }),
                _ => unreachable!("this group should not exist, found {}", group),
            };
            Some(severity)
        } else {
            None
        }
    }
    pub(crate) fn is_recommended(&self) -> bool { !matches!(self.recommended, Some(false)) }
    #[doc = r" It returns a tuple of filters. The first element of the tuple are the enabled rules,"]
    #[doc = r" while the second element are the disabled rules."]
    #[doc = r""]
    #[doc = r" Only one element of the tuple is [Some] at the time."]
    #[doc = r""]
    #[doc = r" The enabled rules are calculated from the difference with the disabled rules."]
    pub fn as_enabled_rules(&self) -> IndexSet<RuleFilter> {
        let mut enabled_rules = IndexSet::new();
        let mut disabled_rules = IndexSet::new();
        if let Some(group) = self.correctness.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Correctness::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Nursery::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Style::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Correctness::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Nursery::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Style::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        enabled_rules.difference(&disabled_rules).cloned().collect()
    }
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct Correctness {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_correctness_rules",
        flatten
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Correctness {
    const CATEGORY_NAME: &'static str = "correctness";
    pub(crate) const CATEGORY_RULES: [&'static str; 29] = [
        "noArguments",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDelete",
        "noDoubleEquals",
        "noDupeArgs",
        "noEmptyPattern",
        "noExtraBooleanCast",
        "noFunctionAssign",
        "noImplicitBoolean",
        "noImportAssign",
        "noLabelVar",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUnnecessaryContinue",
        "noUnsafeNegation",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useOptionalChain",
        "useSimplifiedLogicExpression",
        "useSingleCaseStatement",
        "useSingleVarDeclarator",
        "useTemplate",
        "useValidTypeof",
        "useWhile",
    ];
    const RECOMMENDED_RULES: [&'static str; 29] = [
        "noArguments",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDelete",
        "noDoubleEquals",
        "noDupeArgs",
        "noEmptyPattern",
        "noExtraBooleanCast",
        "noFunctionAssign",
        "noImplicitBoolean",
        "noImportAssign",
        "noLabelVar",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUnnecessaryContinue",
        "noUnsafeNegation",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useOptionalChain",
        "useSimplifiedLogicExpression",
        "useSingleCaseStatement",
        "useSingleVarDeclarator",
        "useTemplate",
        "useValidTypeof",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 29] = [
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[5]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[8]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[9]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[10]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[11]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[12]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[13]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[14]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[15]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[16]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[17]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[18]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[19]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[20]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[21]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[22]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[23]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[24]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[25]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[26]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[27]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[28]),
    ];
    pub(crate) fn is_recommended(&self) -> bool { !matches!(self.recommended, Some(false)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    pub(crate) fn has_rule(rule_name: &str) -> bool { Self::CATEGORY_RULES.contains(&rule_name) }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 29] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_correctness_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Correctness::CATEGORY_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::UnknownRule(rule_name.to_string()),
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct Nursery {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_nursery_rules",
        flatten
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Nursery {
    const CATEGORY_NAME: &'static str = "nursery";
    pub(crate) const CATEGORY_RULES: [&'static str; 3] =
        ["noUnreachable", "noUnusedVariables", "useCamelCase"];
    const RECOMMENDED_RULES: [&'static str; 1] = ["noUnusedVariables"];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 1] =
        [RuleFilter::Rule("nursery", Self::CATEGORY_RULES[1])];
    pub(crate) fn is_recommended(&self) -> bool { !matches!(self.recommended, Some(false)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    pub(crate) fn has_rule(rule_name: &str) -> bool { Self::CATEGORY_RULES.contains(&rule_name) }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 1] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_nursery_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Nursery::CATEGORY_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::UnknownRule(rule_name.to_string()),
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct Style {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_style_rules",
        flatten
    )]
    pub rules: IndexMap<String, RuleConfiguration>,
}
impl Style {
    const CATEGORY_NAME: &'static str = "style";
    pub(crate) const CATEGORY_RULES: [&'static str; 4] = [
        "noNegationElse",
        "noShoutyConstants",
        "useSelfClosingElements",
        "useShorthandArrayType",
    ];
    const RECOMMENDED_RULES: [&'static str; 4] = [
        "noNegationElse",
        "noShoutyConstants",
        "useSelfClosingElements",
        "useShorthandArrayType",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 4] = [
        RuleFilter::Rule("style", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[3]),
    ];
    pub(crate) fn is_recommended(&self) -> bool { !matches!(self.recommended, Some(false)) }
    pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_enabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
        IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
            if conf.is_disabled() {
                Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
            } else {
                None
            }
        }))
    }
    #[doc = r" Checks if, given a rule name, matches one of the rules contained in this category"]
    pub(crate) fn has_rule(rule_name: &str) -> bool { Self::CATEGORY_RULES.contains(&rule_name) }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 4] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_style_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Style::CATEGORY_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(RomeError::Configuration(
                ConfigurationError::UnknownRule(rule_name.to_string()),
            )));
        }
    }
    Ok(value)
}
