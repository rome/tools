use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::{RuleConfiguration, Rules};
use rome_console::markup;
use rome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{AnyJsonValue, JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxNode};

impl VisitJsonNode for LinterConfiguration {}

impl VisitNode<JsonLanguage> for LinterConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, LinterConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text, diagnostics);
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics)?;
            }
            "rules" => {
                let mut rules = Rules::default();
                self.map_to_object(&value, name_text, &mut rules, diagnostics)?;
                if !are_recommended_and_all_correct(
                    &value,
                    rules.recommended.as_ref(),
                    rules.all.as_ref(),
                    diagnostics,
                ) {
                    rules = Rules::default();
                }
                self.rules = Some(rules);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for RuleConfiguration {}

impl VisitNode<JsonLanguage> for RuleConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["level", "options"], diagnostics)
    }

    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, RulePlainConfiguration::KNOWN_KEYS, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "error" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Error);
            }
            "warn" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Warn);
            }
            "off" => {
                *self = RuleConfiguration::Plain(RulePlainConfiguration::Off);
            }
            _ => {}
        }
        Some(())
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "level" => {
                if let RuleConfiguration::WithOptions(options) = self {
                    let mut level = RulePlainConfiguration::default();
                    level.visit_member_value(value.syntax(), diagnostics)?;
                    options.level = level;
                } else {
                    let mut level = RulePlainConfiguration::default();
                    level.visit_member_value(value.syntax(), diagnostics)?;
                    *self = RuleConfiguration::WithOptions(RuleWithOptions {
                        level,
                        ..RuleWithOptions::default()
                    })
                }
            }
            "options" => {
                if let RuleConfiguration::WithOptions(options) = self {
                    options.options = Some(format!("{value}"))
                } else {
                    *self = RuleConfiguration::WithOptions(RuleWithOptions {
                        options: Some(format!("{value}")),
                        ..RuleWithOptions::default()
                    })
                }
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for RulePlainConfiguration {}

impl VisitNode<JsonLanguage> for RulePlainConfiguration {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, RulePlainConfiguration::KNOWN_KEYS, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "error" => {
                *self = RulePlainConfiguration::Error;
            }
            "warn" => {
                *self = RulePlainConfiguration::Warn;
            }
            "off" => {
                *self = RulePlainConfiguration::Off;
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitJsonNode for RuleWithOptions {}

impl VisitNode<JsonLanguage> for RuleWithOptions {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["level", "options"], diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;

        let name_text = name.text();

        match name_text {
            "level" => {
                let mut rule_options = RulePlainConfiguration::default();
                rule_options.visit_member_value(value.syntax(), diagnostics)?;
                self.level = rule_options;
            }
            "options" => {
                self.options = Some(format!("{}", value));
            }
            _ => {}
        }
        Some(())
    }
}

pub(crate) fn are_recommended_and_all_correct(
    current_node: &AnyJsonValue,
    recommended: Option<&bool>,
    all: Option<&bool>,
    diagnostics: &mut Vec<DeserializationDiagnostic>,
) -> bool {
    if let (Some(recommended), Some(all)) = (recommended, all) {
        if *recommended == true && *all == true {
            diagnostics
                .push(DeserializationDiagnostic::new(markup!(
                    <Emphasis>"'recommended'"</Emphasis>" and "<Emphasis>"'all'"</Emphasis>" can't be both "<Emphasis>"'true'"</Emphasis>". You should choose only one of them."
                ))
                    .with_range(current_node.range())
                    .with_note(markup!("Rome will fallback to its defaults for this section.")));
            return false;
        }
    }
    true
}
