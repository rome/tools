//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{ConfigurationError, RomeError, RuleConfiguration};
use indexmap::{IndexMap, IndexSet};
use rome_analyze::RuleFilter;
use rome_diagnostics::v2::{Category, Severity};
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
    pub a11y: Option<A11y>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Complexity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correctness: Option<Correctness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nursery: Option<Nursery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            recommended: Some(true),
            a11y: None,
            complexity: None,
            correctness: None,
            nursery: None,
            security: None,
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
                "a11y" => A11y::has_rule(rule_name).then_some((category, rule_name)),
                "complexity" => Complexity::has_rule(rule_name).then_some((category, rule_name)),
                "correctness" => Correctness::has_rule(rule_name).then_some((category, rule_name)),
                "nursery" => Nursery::has_rule(rule_name).then_some((category, rule_name)),
                "security" => Security::has_rule(rule_name).then_some((category, rule_name)),
                "style" => Style::has_rule(rule_name).then_some((category, rule_name)),
                _ => None,
            },
            _ => None,
        }
    }
    #[doc = r" Given a category coming from [Diagnostic](rome_diagnostics::Diagnostic), this function returns"]
    #[doc = r" the [Severity](rome_diagnostics::Severity) associated to the rule, if the configuration changed it."]
    #[doc = r""]
    #[doc = r" If not, the function returns [None]."]
    pub fn get_severity_from_code(&self, category: &Category) -> Option<Severity> {
        let mut split_code = category.name().split('/');
        let _lint = split_code.next();
        debug_assert_eq!(_lint, Some("lint"));
        let group = split_code.next();
        let rule_name = split_code.next();
        if let Some((group, rule_name)) = self.matches_diagnostic_code(group, rule_name) {
            let severity = match group {
                "a11y" => self
                    .a11y
                    .as_ref()
                    .and_then(|a11y| a11y.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if A11y::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    }),
                "complexity" => self
                    .complexity
                    .as_ref()
                    .and_then(|complexity| complexity.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Complexity::is_recommended_rule(rule_name) {
                            Severity::Error
                        } else {
                            Severity::Warning
                        }
                    }),
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
                "security" => self
                    .security
                    .as_ref()
                    .and_then(|security| security.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Security::is_recommended_rule(rule_name) {
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
        if let Some(group) = self.a11y.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(A11y::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Complexity::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
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
        if let Some(group) = self.security.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Security::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
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
        if let Some(group) = self.a11y.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(A11y::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Complexity::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
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
        if let Some(group) = self.security.as_ref() {
            if self.is_recommended() && group.is_recommended() {
                enabled_rules.extend(Security::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
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
pub struct A11y {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_a11y_rules",
        flatten
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "A11ySchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct A11ySchema {
    no_autofocus: Option<RuleConfiguration>,
    no_positive_tabindex: Option<RuleConfiguration>,
    use_alt_text: Option<RuleConfiguration>,
    use_anchor_content: Option<RuleConfiguration>,
    use_blank_target: Option<RuleConfiguration>,
    use_button_type: Option<RuleConfiguration>,
    use_key_with_click_events: Option<RuleConfiguration>,
    use_key_with_mouse_events: Option<RuleConfiguration>,
    use_valid_anchor: Option<RuleConfiguration>,
}
impl A11y {
    const CATEGORY_NAME: &'static str = "a11y";
    pub(crate) const CATEGORY_RULES: [&'static str; 9] = [
        "noAutofocus",
        "noPositiveTabindex",
        "useAltText",
        "useAnchorContent",
        "useBlankTarget",
        "useButtonType",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useValidAnchor",
    ];
    const RECOMMENDED_RULES: [&'static str; 9] = [
        "noAutofocus",
        "noPositiveTabindex",
        "useAltText",
        "useAnchorContent",
        "useBlankTarget",
        "useButtonType",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useValidAnchor",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 9] = [
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[5]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[8]),
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 9] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_a11y_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !A11y::CATEGORY_RULES.contains(&rule_name.as_str()) {
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
pub struct Complexity {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_complexity_rules",
        flatten
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "ComplexitySchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct ComplexitySchema {
    no_extra_boolean_cast: Option<RuleConfiguration>,
    use_simplified_logic_expression: Option<RuleConfiguration>,
}
impl Complexity {
    const CATEGORY_NAME: &'static str = "complexity";
    pub(crate) const CATEGORY_RULES: [&'static str; 2] =
        ["noExtraBooleanCast", "useSimplifiedLogicExpression"];
    const RECOMMENDED_RULES: [&'static str; 2] =
        ["noExtraBooleanCast", "useSimplifiedLogicExpression"];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 2] = [
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[1]),
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 2] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_complexity_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Complexity::CATEGORY_RULES.contains(&rule_name.as_str()) {
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
    #[cfg_attr(feature = "schemars", schemars(with = "CorrectnessSchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct CorrectnessSchema {
    no_arguments: Option<RuleConfiguration>,
    no_array_index_key: Option<RuleConfiguration>,
    no_async_promise_executor: Option<RuleConfiguration>,
    no_catch_assign: Option<RuleConfiguration>,
    no_children_prop: Option<RuleConfiguration>,
    no_comment_text: Option<RuleConfiguration>,
    no_compare_neg_zero: Option<RuleConfiguration>,
    no_debugger: Option<RuleConfiguration>,
    no_delete: Option<RuleConfiguration>,
    no_double_equals: Option<RuleConfiguration>,
    no_dupe_args: Option<RuleConfiguration>,
    no_empty_pattern: Option<RuleConfiguration>,
    no_function_assign: Option<RuleConfiguration>,
    no_import_assign: Option<RuleConfiguration>,
    no_label_var: Option<RuleConfiguration>,
    no_multiple_spaces_in_regular_expression_literals: Option<RuleConfiguration>,
    no_new_symbol: Option<RuleConfiguration>,
    no_render_return_value: Option<RuleConfiguration>,
    no_restricted_globals: Option<RuleConfiguration>,
    no_shadow_restricted_names: Option<RuleConfiguration>,
    no_sparse_array: Option<RuleConfiguration>,
    no_undeclared_variables: Option<RuleConfiguration>,
    no_unnecessary_continue: Option<RuleConfiguration>,
    no_unreachable: Option<RuleConfiguration>,
    no_unsafe_negation: Option<RuleConfiguration>,
    no_unused_variables: Option<RuleConfiguration>,
    no_useless_fragments: Option<RuleConfiguration>,
    no_void_elements_with_children: Option<RuleConfiguration>,
    use_single_case_statement: Option<RuleConfiguration>,
    use_valid_typeof: Option<RuleConfiguration>,
    use_while: Option<RuleConfiguration>,
}
impl Correctness {
    const CATEGORY_NAME: &'static str = "correctness";
    pub(crate) const CATEGORY_RULES: [&'static str; 31] = [
        "noArguments",
        "noArrayIndexKey",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noChildrenProp",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDelete",
        "noDoubleEquals",
        "noDupeArgs",
        "noEmptyPattern",
        "noFunctionAssign",
        "noImportAssign",
        "noLabelVar",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noNewSymbol",
        "noRenderReturnValue",
        "noRestrictedGlobals",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUndeclaredVariables",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnsafeNegation",
        "noUnusedVariables",
        "noUselessFragments",
        "noVoidElementsWithChildren",
        "useSingleCaseStatement",
        "useValidTypeof",
        "useWhile",
    ];
    const RECOMMENDED_RULES: [&'static str; 27] = [
        "noArguments",
        "noArrayIndexKey",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noChildrenProp",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDelete",
        "noDoubleEquals",
        "noDupeArgs",
        "noEmptyPattern",
        "noFunctionAssign",
        "noImportAssign",
        "noLabelVar",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noNewSymbol",
        "noRenderReturnValue",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnsafeNegation",
        "noVoidElementsWithChildren",
        "useSingleCaseStatement",
        "useValidTypeof",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 27] = [
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
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[19]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[20]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[22]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[23]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[24]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[27]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[28]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[29]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[30]),
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 27] {
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
    #[cfg_attr(feature = "schemars", schemars(with = "NurserySchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct NurserySchema {
    no_banned_types: Option<RuleConfiguration>,
    no_const_assign: Option<RuleConfiguration>,
    no_explicit_any: Option<RuleConfiguration>,
    no_invalid_constructor_super: Option<RuleConfiguration>,
    use_camel_case: Option<RuleConfiguration>,
    use_const: Option<RuleConfiguration>,
    use_exhaustive_dependencies: Option<RuleConfiguration>,
    use_flat_map: Option<RuleConfiguration>,
    use_valid_for_direction: Option<RuleConfiguration>,
}
impl Nursery {
    const CATEGORY_NAME: &'static str = "nursery";
    pub(crate) const CATEGORY_RULES: [&'static str; 9] = [
        "noBannedTypes",
        "noConstAssign",
        "noExplicitAny",
        "noInvalidConstructorSuper",
        "useCamelCase",
        "useConst",
        "useExhaustiveDependencies",
        "useFlatMap",
        "useValidForDirection",
    ];
    const RECOMMENDED_RULES: [&'static str; 0] = [];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 0] = [];
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 0] {
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
pub struct Security {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_security_rules",
        flatten
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "SecuritySchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct SecuritySchema {
    no_dangerously_set_inner_html: Option<RuleConfiguration>,
    no_dangerously_set_inner_html_with_children: Option<RuleConfiguration>,
}
impl Security {
    const CATEGORY_NAME: &'static str = "security";
    pub(crate) const CATEGORY_RULES: [&'static str; 2] = [
        "noDangerouslySetInnerHtml",
        "noDangerouslySetInnerHtmlWithChildren",
    ];
    const RECOMMENDED_RULES: [&'static str; 2] = [
        "noDangerouslySetInnerHtml",
        "noDangerouslySetInnerHtmlWithChildren",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 2] = [
        RuleFilter::Rule("security", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("security", Self::CATEGORY_RULES[1]),
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 2] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_security_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Security::CATEGORY_RULES.contains(&rule_name.as_str()) {
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
    #[cfg_attr(feature = "schemars", schemars(with = "StyleSchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct StyleSchema {
    no_implicit_boolean: Option<RuleConfiguration>,
    no_negation_else: Option<RuleConfiguration>,
    no_shouty_constants: Option<RuleConfiguration>,
    no_unused_template_literal: Option<RuleConfiguration>,
    use_block_statements: Option<RuleConfiguration>,
    use_fragment_syntax: Option<RuleConfiguration>,
    use_optional_chain: Option<RuleConfiguration>,
    use_self_closing_elements: Option<RuleConfiguration>,
    use_shorthand_array_type: Option<RuleConfiguration>,
    use_single_var_declarator: Option<RuleConfiguration>,
    use_template: Option<RuleConfiguration>,
}
impl Style {
    const CATEGORY_NAME: &'static str = "style";
    pub(crate) const CATEGORY_RULES: [&'static str; 11] = [
        "noImplicitBoolean",
        "noNegationElse",
        "noShoutyConstants",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useFragmentSyntax",
        "useOptionalChain",
        "useSelfClosingElements",
        "useShorthandArrayType",
        "useSingleVarDeclarator",
        "useTemplate",
    ];
    const RECOMMENDED_RULES: [&'static str; 10] = [
        "noImplicitBoolean",
        "noNegationElse",
        "noShoutyConstants",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useOptionalChain",
        "useSelfClosingElements",
        "useShorthandArrayType",
        "useSingleVarDeclarator",
        "useTemplate",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 10] = [
        RuleFilter::Rule("style", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[8]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[9]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[10]),
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
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 10] {
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
