use crate::configuration::linter::{RulePlainConfiguration, RuleWithOptions};
use crate::configuration::LinterConfiguration;
use crate::{RuleConfiguration, Rules};
use rome_deserialize::json::{
    has_only_known_keys, with_only_known_variants, VisitConfigurationAsJson,
};
use rome_deserialize::{DeserializationDiagnostic, VisitConfigurationNode};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxNode};

impl VisitConfigurationAsJson for LinterConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for LinterConfiguration {
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
                self.rules = Some(rules);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitConfigurationAsJson for RuleConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for RuleConfiguration {
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
}

impl VisitConfigurationAsJson for RulePlainConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for RulePlainConfiguration {
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

impl VisitConfigurationAsJson for RuleWithOptions {}

impl VisitConfigurationNode<JsonLanguage> for RuleWithOptions {
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
