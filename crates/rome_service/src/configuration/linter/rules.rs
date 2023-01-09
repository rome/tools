//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::RuleConfiguration;
use indexmap::{IndexMap, IndexSet};
use rome_analyze::RuleFilter;
use rome_diagnostics::{Category, Severity};
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
    pub performance: Option<Performance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<Suspicious>,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            recommended: Some(true),
            a11y: None,
            complexity: None,
            correctness: None,
            nursery: None,
            performance: None,
            security: None,
            style: None,
            suspicious: None,
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
                "performance" => Performance::has_rule(rule_name).then_some((category, rule_name)),
                "security" => Security::has_rule(rule_name).then_some((category, rule_name)),
                "style" => Style::has_rule(rule_name).then_some((category, rule_name)),
                "suspicious" => Suspicious::has_rule(rule_name).then_some((category, rule_name)),
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
                "performance" => self
                    .performance
                    .as_ref()
                    .and_then(|performance| performance.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Performance::is_recommended_rule(rule_name) {
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
                "suspicious" => self
                    .suspicious
                    .as_ref()
                    .and_then(|suspicious| suspicious.rules.get(rule_name))
                    .map(|rule_setting| rule_setting.into())
                    .unwrap_or_else(|| {
                        if Suspicious::is_recommended_rule(rule_name) {
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
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(A11y::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Complexity::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Correctness::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            if self.is_recommended() && rome_flags::is_unstable() || group.is_recommended() {
                enabled_rules.extend(Nursery::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() && rome_flags::is_unstable() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.performance.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Performance::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Performance::recommended_rules_as_filters());
        }
        if let Some(group) = self.security.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Security::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Style::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.suspicious.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Suspicious::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Suspicious::recommended_rules_as_filters());
        }
        if let Some(group) = self.a11y.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(A11y::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(A11y::recommended_rules_as_filters());
        }
        if let Some(group) = self.complexity.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Complexity::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Complexity::recommended_rules_as_filters());
        }
        if let Some(group) = self.correctness.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Correctness::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Correctness::recommended_rules_as_filters());
        }
        if let Some(group) = self.nursery.as_ref() {
            if self.is_recommended() && rome_flags::is_unstable() || group.is_recommended() {
                enabled_rules.extend(Nursery::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() && rome_flags::is_unstable() {
            enabled_rules.extend(Nursery::recommended_rules_as_filters());
        }
        if let Some(group) = self.performance.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Performance::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Performance::recommended_rules_as_filters());
        }
        if let Some(group) = self.security.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Security::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Security::recommended_rules_as_filters());
        }
        if let Some(group) = self.style.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Style::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Style::recommended_rules_as_filters());
        }
        if let Some(group) = self.suspicious.as_ref() {
            if self.is_recommended() || group.is_recommended() {
                enabled_rules.extend(Suspicious::recommended_rules_as_filters());
            }
            enabled_rules.extend(&group.get_enabled_rules());
            disabled_rules.extend(&group.get_disabled_rules());
        } else if self.is_recommended() {
            enabled_rules.extend(Suspicious::recommended_rules_as_filters());
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
    #[doc = "Avoid the autoFocus attribute"]
    no_autofocus: Option<RuleConfiguration>,
    #[doc = "Disallow target=\"_blank\" attribute without rel=\"noreferrer\""]
    no_blank_target: Option<RuleConfiguration>,
    #[doc = "Prevent the usage of positive integers on tabIndex property"]
    no_positive_tabindex: Option<RuleConfiguration>,
    #[doc = "It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image."]
    use_alt_text: Option<RuleConfiguration>,
    #[doc = "Enforce that anchor elements have content and that the content is accessible to screen readers."]
    use_anchor_content: Option<RuleConfiguration>,
    #[doc = "Enforces the usage of the attribute type for the element button"]
    use_button_type: Option<RuleConfiguration>,
    #[doc = "Enforce that html element has lang attribute. This allows users to choose a language other than the default."]
    use_html_lang: Option<RuleConfiguration>,
    #[doc = "Enforce to have the onClick mouse event with the onKeyUp, the onKeyDown, or the onKeyPress keyboard event."]
    use_key_with_click_events: Option<RuleConfiguration>,
    #[doc = "Enforce that onMouseOver/onMouseOut are accompanied by onFocus/onBlur for keyboard-only users. It is important to take into account users with physical disabilities who cannot use a mouse, who use assistive technology or screenreader."]
    use_key_with_mouse_events: Option<RuleConfiguration>,
    #[doc = "Enforce that all anchors are valid, and they are navigable elements."]
    use_valid_anchor: Option<RuleConfiguration>,
}
impl A11y {
    const CATEGORY_NAME: &'static str = "a11y";
    pub(crate) const CATEGORY_RULES: [&'static str; 10] = [
        "noAutofocus",
        "noBlankTarget",
        "noPositiveTabindex",
        "useAltText",
        "useAnchorContent",
        "useButtonType",
        "useHtmlLang",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useValidAnchor",
    ];
    const RECOMMENDED_RULES: [&'static str; 10] = [
        "noAutofocus",
        "noBlankTarget",
        "noPositiveTabindex",
        "useAltText",
        "useAnchorContent",
        "useButtonType",
        "useHtmlLang",
        "useKeyWithClickEvents",
        "useKeyWithMouseEvents",
        "useValidAnchor",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 10] = [
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[5]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[8]),
        RuleFilter::Rule("a11y", Self::CATEGORY_RULES[9]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 10] {
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
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
    #[doc = "Disallow unnecessary boolean casts"]
    no_extra_boolean_cast: Option<RuleConfiguration>,
    #[doc = "Disallow unclear usage of multiple space characters in regular expression literals"]
    no_multiple_spaces_in_regular_expression_literals: Option<RuleConfiguration>,
    #[doc = "Disallow unnecessary fragments"]
    no_useless_fragments: Option<RuleConfiguration>,
    #[doc = "Promotes the use of .flatMap() when map().flat() are used together."]
    use_flat_map: Option<RuleConfiguration>,
    #[doc = "Enforce using concise optional chain instead of chained logical expressions."]
    use_optional_chain: Option<RuleConfiguration>,
    #[doc = "Discard redundant terms from logical expressions."]
    use_simplified_logic_expression: Option<RuleConfiguration>,
}
impl Complexity {
    const CATEGORY_NAME: &'static str = "complexity";
    pub(crate) const CATEGORY_RULES: [&'static str; 6] = [
        "noExtraBooleanCast",
        "noMultipleSpacesInRegularExpressionLiterals",
        "noUselessFragments",
        "useFlatMap",
        "useOptionalChain",
        "useSimplifiedLogicExpression",
    ];
    const RECOMMENDED_RULES: [&'static str; 5] = [
        "noExtraBooleanCast",
        "noMultipleSpacesInRegularExpressionLiterals",
        "useFlatMap",
        "useOptionalChain",
        "useSimplifiedLogicExpression",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 5] = [
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("complexity", Self::CATEGORY_RULES[5]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 5] {
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
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
    #[doc = "Prevent passing of children as props."]
    no_children_prop: Option<RuleConfiguration>,
    #[doc = "Prevents from having const variables being re-assigned."]
    no_const_assign: Option<RuleConfiguration>,
    #[doc = "Disallows empty destructuring patterns."]
    no_empty_pattern: Option<RuleConfiguration>,
    #[doc = "Disallow new operators with the Symbol object"]
    no_new_symbol: Option<RuleConfiguration>,
    #[doc = "Prevent the usage of the return value of React.render."]
    no_render_return_value: Option<RuleConfiguration>,
    #[doc = "Prevents the usage of variables that haven't been declared inside the document"]
    no_undeclared_variables: Option<RuleConfiguration>,
    #[doc = "Avoid using unnecessary continue."]
    no_unnecessary_continue: Option<RuleConfiguration>,
    #[doc = "Disallow unreachable code"]
    no_unreachable: Option<RuleConfiguration>,
    #[doc = "Disallow unused variables."]
    no_unused_variables: Option<RuleConfiguration>,
    #[doc = "This rules prevents void elements (AKA self-closing elements) from having children."]
    no_void_elements_with_children: Option<RuleConfiguration>,
    #[doc = "Enforce \"for\" loop update clause moving the counter in the right direction."]
    use_valid_for_direction: Option<RuleConfiguration>,
}
impl Correctness {
    const CATEGORY_NAME: &'static str = "correctness";
    pub(crate) const CATEGORY_RULES: [&'static str; 11] = [
        "noChildrenProp",
        "noConstAssign",
        "noEmptyPattern",
        "noNewSymbol",
        "noRenderReturnValue",
        "noUndeclaredVariables",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noUnusedVariables",
        "noVoidElementsWithChildren",
        "useValidForDirection",
    ];
    const RECOMMENDED_RULES: [&'static str; 9] = [
        "noChildrenProp",
        "noConstAssign",
        "noEmptyPattern",
        "noNewSymbol",
        "noRenderReturnValue",
        "noUnnecessaryContinue",
        "noUnreachable",
        "noVoidElementsWithChildren",
        "useValidForDirection",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 9] = [
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[9]),
        RuleFilter::Rule("correctness", Self::CATEGORY_RULES[10]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 9] {
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
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
    #[doc = "Enforce that the accessKey attribute is not used on any HTML element."]
    no_access_key: Option<RuleConfiguration>,
    #[doc = "Disallow assignments in expressions."]
    no_assign_in_expressions: Option<RuleConfiguration>,
    #[doc = "Disallow certain types."]
    no_banned_types: Option<RuleConfiguration>,
    #[doc = "Disallow reassigning class members."]
    no_class_assign: Option<RuleConfiguration>,
    #[doc = "Disallow comma operator."]
    no_comma_operator: Option<RuleConfiguration>,
    #[doc = "Disallow TypeScript const enum"]
    no_const_enum: Option<RuleConfiguration>,
    #[doc = "Disallow returning a value from a constructor."]
    no_constructor_return: Option<RuleConfiguration>,
    #[doc = "Enforces that no distracting elements are used."]
    no_distracting_elements: Option<RuleConfiguration>,
    #[doc = "Disallow duplicate case labels. If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression."]
    no_duplicate_case: Option<RuleConfiguration>,
    #[doc = "Disallow duplicate class members."]
    no_duplicate_class_members: Option<RuleConfiguration>,
    #[doc = "Prevents JSX properties to be assigned multiple times."]
    no_duplicate_jsx_props: Option<RuleConfiguration>,
    #[doc = "Prevents object literals having more than one property declaration for the same name. If an object property with the same name is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored, which is likely a mistake."]
    no_duplicate_object_keys: Option<RuleConfiguration>,
    #[doc = "Disallow the declaration of empty interfaces."]
    no_empty_interface: Option<RuleConfiguration>,
    #[doc = "Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files."]
    no_extra_non_null_assertion: Option<RuleConfiguration>,
    #[doc = "Typing mistakes and misunderstandings about where semicolons are required can lead to semicolons that are unnecessary. While not technically an error, extra semicolons can cause confusion when reading code."]
    no_extra_semicolons: Option<RuleConfiguration>,
    #[doc = "Disallow calling global object properties as functions"]
    no_global_object_calls: Option<RuleConfiguration>,
    #[doc = "Check that the scope attribute is only used on th elements."]
    no_header_scope: Option<RuleConfiguration>,
    #[doc = "Disallow function and var declarations in nested blocks."]
    no_inner_declarations: Option<RuleConfiguration>,
    #[doc = "Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors."]
    no_invalid_constructor_super: Option<RuleConfiguration>,
    #[doc = "Disallow non-null assertions using the ! postfix operator."]
    no_non_null_assertion: Option<RuleConfiguration>,
    #[doc = "Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements."]
    no_noninteractive_element_to_interactive_role: Option<RuleConfiguration>,
    #[doc = "Disallow literal numbers that lose precision"]
    no_precision_loss: Option<RuleConfiguration>,
    #[doc = "Disallow direct use of Object.prototype builtins."]
    no_prototype_builtins: Option<RuleConfiguration>,
    #[doc = "Enforce img alt prop does not contain the word \"image\", \"picture\", or \"photo\"."]
    no_redundant_alt: Option<RuleConfiguration>,
    #[doc = "Prevents from having redundant \"use strict\"."]
    no_redundant_use_strict: Option<RuleConfiguration>,
    #[doc = "This rule allows you to specify global variable names that you don’t want to use in your application."]
    no_restricted_globals: Option<RuleConfiguration>,
    #[doc = "Disallow comparisons where both sides are exactly the same."]
    no_self_compare: Option<RuleConfiguration>,
    #[doc = "Disallow returning a value from a setter"]
    no_setter_return: Option<RuleConfiguration>,
    #[doc = "Disallow comparison of expressions modifying the string case with non-compliant value."]
    no_string_case_mismatch: Option<RuleConfiguration>,
    #[doc = "Ensures the super() constructor is called exactly once on every code path in a class constructor before this is accessed if the class has a superclass"]
    no_unreachable_super: Option<RuleConfiguration>,
    #[doc = "Disallow control flow statements in finally blocks."]
    no_unsafe_finally: Option<RuleConfiguration>,
    #[doc = "Disallow unused labels."]
    no_unused_labels: Option<RuleConfiguration>,
    #[doc = "Disallow useless case in switch statements."]
    no_useless_switch_case: Option<RuleConfiguration>,
    #[doc = "Disallow the use of var"]
    no_var: Option<RuleConfiguration>,
    #[doc = "Disallow returning a value from a function with the return type 'void'"]
    no_void_type_return: Option<RuleConfiguration>,
    #[doc = "Disallow with statements in non-strict contexts."]
    no_with: Option<RuleConfiguration>,
    #[doc = "Enforce that ARIA state and property values are valid."]
    use_aria_prop_types: Option<RuleConfiguration>,
    #[doc = "Enforce that elements with ARIA roles must have all required ARIA attributes for that role."]
    use_aria_props_for_role: Option<RuleConfiguration>,
    #[doc = "Enforce camel case naming convention."]
    use_camel_case: Option<RuleConfiguration>,
    #[doc = "Require const declarations for variables that are never reassigned after declared."]
    use_const: Option<RuleConfiguration>,
    #[doc = "Enforce default function parameters and optional parameters to be last."]
    use_default_parameter_last: Option<RuleConfiguration>,
    #[doc = "Enforce default clauses in switch statements to be last"]
    use_default_switch_clause_last: Option<RuleConfiguration>,
    #[doc = "Require that each enum member value be explicitly initialized."]
    use_enum_initializers: Option<RuleConfiguration>,
    #[doc = "Enforce all dependencies are correctly specified."]
    use_exhaustive_dependencies: Option<RuleConfiguration>,
    #[doc = "Disallow the use of Math.pow in favor of the ** operator."]
    use_exponentiation_operator: Option<RuleConfiguration>,
    #[doc = "Enforce that all React hooks are being called from the Top Level component functions."]
    use_hook_at_top_level: Option<RuleConfiguration>,
    #[doc = "Enforces the usage of the attribute title for the element iframe"]
    use_iframe_title: Option<RuleConfiguration>,
    #[doc = "Require calls to isNaN() when checking for NaN."]
    use_is_nan: Option<RuleConfiguration>,
    #[doc = "Enforces that audio and video elements must have a track for captions."]
    use_media_caption: Option<RuleConfiguration>,
    #[doc = "Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals"]
    use_numeric_literals: Option<RuleConfiguration>,
    #[doc = "Ensures that ARIA properties aria-* are all valid."]
    use_valid_aria_props: Option<RuleConfiguration>,
    #[doc = "Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country."]
    use_valid_lang: Option<RuleConfiguration>,
    #[doc = "Require generator functions to contain yield."]
    use_yield: Option<RuleConfiguration>,
}
impl Nursery {
    const CATEGORY_NAME: &'static str = "nursery";
    pub(crate) const CATEGORY_RULES: [&'static str; 53] = [
        "noAccessKey",
        "noAssignInExpressions",
        "noBannedTypes",
        "noClassAssign",
        "noCommaOperator",
        "noConstEnum",
        "noConstructorReturn",
        "noDistractingElements",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noEmptyInterface",
        "noExtraNonNullAssertion",
        "noExtraSemicolons",
        "noGlobalObjectCalls",
        "noHeaderScope",
        "noInnerDeclarations",
        "noInvalidConstructorSuper",
        "noNonNullAssertion",
        "noNoninteractiveElementToInteractiveRole",
        "noPrecisionLoss",
        "noPrototypeBuiltins",
        "noRedundantAlt",
        "noRedundantUseStrict",
        "noRestrictedGlobals",
        "noSelfCompare",
        "noSetterReturn",
        "noStringCaseMismatch",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnusedLabels",
        "noUselessSwitchCase",
        "noVar",
        "noVoidTypeReturn",
        "noWith",
        "useAriaPropTypes",
        "useAriaPropsForRole",
        "useCamelCase",
        "useConst",
        "useDefaultParameterLast",
        "useDefaultSwitchClauseLast",
        "useEnumInitializers",
        "useExhaustiveDependencies",
        "useExponentiationOperator",
        "useHookAtTopLevel",
        "useIframeTitle",
        "useIsNan",
        "useMediaCaption",
        "useNumericLiterals",
        "useValidAriaProps",
        "useValidLang",
        "useYield",
    ];
    const RECOMMENDED_RULES: [&'static str; 43] = [
        "noAssignInExpressions",
        "noBannedTypes",
        "noClassAssign",
        "noCommaOperator",
        "noConstEnum",
        "noConstructorReturn",
        "noDistractingElements",
        "noDuplicateCase",
        "noDuplicateClassMembers",
        "noDuplicateJsxProps",
        "noDuplicateObjectKeys",
        "noEmptyInterface",
        "noExtraNonNullAssertion",
        "noExtraSemicolons",
        "noGlobalObjectCalls",
        "noHeaderScope",
        "noInnerDeclarations",
        "noInvalidConstructorSuper",
        "noNoninteractiveElementToInteractiveRole",
        "noRedundantAlt",
        "noSelfCompare",
        "noSetterReturn",
        "noStringCaseMismatch",
        "noUnreachableSuper",
        "noUnsafeFinally",
        "noUnusedLabels",
        "noUselessSwitchCase",
        "noVar",
        "noVoidTypeReturn",
        "noWith",
        "useAriaPropsForRole",
        "useConst",
        "useDefaultParameterLast",
        "useDefaultSwitchClauseLast",
        "useEnumInitializers",
        "useExhaustiveDependencies",
        "useIframeTitle",
        "useIsNan",
        "useMediaCaption",
        "useNumericLiterals",
        "useValidAriaProps",
        "useValidLang",
        "useYield",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 43] = [
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[5]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[8]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[9]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[10]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[11]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[12]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[13]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[14]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[15]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[16]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[17]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[18]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[20]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[23]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[26]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[27]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[28]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[29]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[30]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[31]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[32]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[33]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[34]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[35]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[37]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[39]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[40]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[41]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[42]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[43]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[46]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[47]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[48]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[49]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[50]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[51]),
        RuleFilter::Rule("nursery", Self::CATEGORY_RULES[52]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 43] {
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct Performance {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_performance_rules",
        flatten
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "PerformanceSchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct PerformanceSchema {
    #[doc = "Disallow the use of the delete operator"]
    no_delete: Option<RuleConfiguration>,
}
impl Performance {
    const CATEGORY_NAME: &'static str = "performance";
    pub(crate) const CATEGORY_RULES: [&'static str; 1] = ["noDelete"];
    const RECOMMENDED_RULES: [&'static str; 1] = ["noDelete"];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 1] =
        [RuleFilter::Rule("performance", Self::CATEGORY_RULES[0])];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 1] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_performance_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Performance::CATEGORY_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
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
    #[doc = "Prevent the usage of dangerous JSX props"]
    no_dangerously_set_inner_html: Option<RuleConfiguration>,
    #[doc = "Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop."]
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
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
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
    #[doc = "Disallow the use of arguments"]
    no_arguments: Option<RuleConfiguration>,
    #[doc = "Disallow implicit true values on JSX boolean attributes"]
    no_implicit_boolean: Option<RuleConfiguration>,
    #[doc = "Disallow negation in the condition of an if statement if it has an else clause"]
    no_negation_else: Option<RuleConfiguration>,
    #[doc = "Disallow the use of constants which its value is the upper-case version of its name."]
    no_shouty_constants: Option<RuleConfiguration>,
    #[doc = "Disallow template literals if interpolation and special-character handling are not needed"]
    no_unused_template_literal: Option<RuleConfiguration>,
    #[doc = "Requires following curly brace conventions. JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity."]
    use_block_statements: Option<RuleConfiguration>,
    #[doc = "This rule enforces the use of <>...</> over <Fragment>...</Fragment>."]
    use_fragment_syntax: Option<RuleConfiguration>,
    #[doc = "Prevent extra closing tags for components without children"]
    use_self_closing_elements: Option<RuleConfiguration>,
    #[doc = "When expressing array types, this rule promotes the usage of T[] shorthand instead of Array<T>."]
    use_shorthand_array_type: Option<RuleConfiguration>,
    #[doc = "Enforces case clauses have a single statement, emits a quick fix wrapping the statements in a block"]
    use_single_case_statement: Option<RuleConfiguration>,
    #[doc = "Disallow multiple variable declarations in the same variable statement"]
    use_single_var_declarator: Option<RuleConfiguration>,
    #[doc = "Template literals are preferred over string concatenation."]
    use_template: Option<RuleConfiguration>,
    #[doc = "Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed"]
    use_while: Option<RuleConfiguration>,
}
impl Style {
    const CATEGORY_NAME: &'static str = "style";
    pub(crate) const CATEGORY_RULES: [&'static str; 13] = [
        "noArguments",
        "noImplicitBoolean",
        "noNegationElse",
        "noShoutyConstants",
        "noUnusedTemplateLiteral",
        "useBlockStatements",
        "useFragmentSyntax",
        "useSelfClosingElements",
        "useShorthandArrayType",
        "useSingleCaseStatement",
        "useSingleVarDeclarator",
        "useTemplate",
        "useWhile",
    ];
    const RECOMMENDED_RULES: [&'static str; 6] = [
        "noArguments",
        "noUnusedTemplateLiteral",
        "useSelfClosingElements",
        "useSingleVarDeclarator",
        "useTemplate",
        "useWhile",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 6] = [
        RuleFilter::Rule("style", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[10]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[11]),
        RuleFilter::Rule("style", Self::CATEGORY_RULES[12]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 6] {
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
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
            )));
        }
    }
    Ok(value)
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct Suspicious {
    #[doc = r" It enables the recommended rules for this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    #[doc = r" List of rules for the current group"]
    #[serde(
        skip_serializing_if = "IndexMap::is_empty",
        deserialize_with = "deserialize_suspicious_rules",
        flatten
    )]
    #[cfg_attr(feature = "schemars", schemars(with = "SuspiciousSchema"))]
    pub rules: IndexMap<String, RuleConfiguration>,
}
#[cfg_attr(
    feature = "schemars",
    derive(JsonSchema),
    serde(rename_all = "camelCase")
)]
#[allow(dead_code)]
#[doc = r" A list of rules that belong to this group"]
struct SuspiciousSchema {
    #[doc = "Discourage the usage of Array index in keys."]
    no_array_index_key: Option<RuleConfiguration>,
    #[doc = "Disallows using an async function as a Promise executor."]
    no_async_promise_executor: Option<RuleConfiguration>,
    #[doc = "Disallow reassigning exceptions in catch clauses"]
    no_catch_assign: Option<RuleConfiguration>,
    #[doc = "Prevent comments from being inserted as text nodes"]
    no_comment_text: Option<RuleConfiguration>,
    #[doc = "Disallow comparing against -0"]
    no_compare_neg_zero: Option<RuleConfiguration>,
    #[doc = "Disallow the use of debugger"]
    no_debugger: Option<RuleConfiguration>,
    #[doc = "Require the use of === and !=="]
    no_double_equals: Option<RuleConfiguration>,
    #[doc = "Disallow duplicate function arguments name."]
    no_duplicate_parameters: Option<RuleConfiguration>,
    #[doc = "Disallow the any type usage."]
    no_explicit_any: Option<RuleConfiguration>,
    #[doc = "Disallow reassigning function declarations."]
    no_function_assign: Option<RuleConfiguration>,
    #[doc = "Disallow assigning to imported bindings"]
    no_import_assign: Option<RuleConfiguration>,
    #[doc = "Disallow labels that share a name with a variable"]
    no_label_var: Option<RuleConfiguration>,
    #[doc = "Disallow identifiers from shadowing restricted names."]
    no_shadow_restricted_names: Option<RuleConfiguration>,
    #[doc = "Disallow sparse arrays"]
    no_sparse_array: Option<RuleConfiguration>,
    #[doc = "Disallow using unsafe negation."]
    no_unsafe_negation: Option<RuleConfiguration>,
    #[doc = "This rule verifies the result of typeof $expr unary expressions is being compared to valid values, either string literals containing valid type names or other typeof expressions"]
    use_valid_typeof: Option<RuleConfiguration>,
}
impl Suspicious {
    const CATEGORY_NAME: &'static str = "suspicious";
    pub(crate) const CATEGORY_RULES: [&'static str; 16] = [
        "noArrayIndexKey",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDoubleEquals",
        "noDuplicateParameters",
        "noExplicitAny",
        "noFunctionAssign",
        "noImportAssign",
        "noLabelVar",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUnsafeNegation",
        "useValidTypeof",
    ];
    const RECOMMENDED_RULES: [&'static str; 16] = [
        "noArrayIndexKey",
        "noAsyncPromiseExecutor",
        "noCatchAssign",
        "noCommentText",
        "noCompareNegZero",
        "noDebugger",
        "noDoubleEquals",
        "noDuplicateParameters",
        "noExplicitAny",
        "noFunctionAssign",
        "noImportAssign",
        "noLabelVar",
        "noShadowRestrictedNames",
        "noSparseArray",
        "noUnsafeNegation",
        "useValidTypeof",
    ];
    const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; 16] = [
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[0]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[1]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[2]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[3]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[4]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[5]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[6]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[7]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[8]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[9]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[10]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[11]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[12]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[13]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[14]),
        RuleFilter::Rule("suspicious", Self::CATEGORY_RULES[15]),
    ];
    pub(crate) fn is_recommended(&self) -> bool {
        !matches!(self.recommended, Some(false))
    }
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
    pub(crate) fn has_rule(rule_name: &str) -> bool {
        Self::CATEGORY_RULES.contains(&rule_name)
    }
    #[doc = r" Checks if, given a rule name, it is marked as recommended"]
    pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
        Self::RECOMMENDED_RULES.contains(&rule_name)
    }
    pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; 16] {
        Self::RECOMMENDED_RULES_AS_FILTERS
    }
}
fn deserialize_suspicious_rules<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
    for rule_name in value.keys() {
        if !Suspicious::CATEGORY_RULES.contains(&rule_name.as_str()) {
            return Err(serde::de::Error::custom(format!(
                "invalid rule name {}",
                rule_name
            )));
        }
    }
    Ok(value)
}
