//! This module contains the rules that have options

use crate::analyzers::nursery::no_excessive_complexity::{complexity_options, ComplexityOptions};
use crate::semantic_analyzers::nursery::use_exhaustive_dependencies::{
    hooks_options, HooksOptions,
};
use crate::semantic_analyzers::nursery::use_naming_convention::{
    naming_convention_options, NamingConventionOptions,
};
use bpaf::Bpaf;
use rome_analyze::options::RuleOptions;
use rome_analyze::RuleKey;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum PossibleOptions {
    /// Options for `noExcessiveComplexity` rule
    Complexity(#[bpaf(external(complexity_options), hide)] ComplexityOptions),
    /// Options for `useExhaustiveDependencies` and `useHookAtTopLevel` rule
    Hooks(#[bpaf(external(hooks_options), hide)] HooksOptions),
    /// Options for `useNamingConvention` rule
    NamingConvention(#[bpaf(external(naming_convention_options), hide)] NamingConventionOptions),
    /// No options available
    #[default]
    NoOptions,
}

impl FromStr for PossibleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::NoOptions)
    }
}

impl PossibleOptions {
    const KNOWN_KEYS: &'static [&'static str] = &[
        "enumMemberCase",
        "hooks",
        "maxAllowedComplexity",
        "strictCase",
    ];

    pub fn extract_option(&self, rule_key: &RuleKey) -> RuleOptions {
        match rule_key.rule_name() {
            "noExcessiveComplexity" => {
                let options = match self {
                    PossibleOptions::Complexity(options) => options.clone(),
                    _ => ComplexityOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                let options = match self {
                    PossibleOptions::Hooks(options) => options.clone(),
                    _ => HooksOptions::default(),
                };
                RuleOptions::new(options)
            }
            "useNamingConvention" => {
                let options = match self {
                    PossibleOptions::NamingConvention(options) => *options,
                    _ => NamingConventionOptions::default(),
                };
                RuleOptions::new(options)
            }
            // TODO: review error
            _ => panic!("This rule {:?} doesn't have options", rule_key),
        }
    }
}

impl VisitJsonNode for PossibleOptions {}
impl VisitNode<JsonLanguage> for PossibleOptions {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, PossibleOptions::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, val) = self.get_key_and_value(key, value, diagnostics)?;
        match name.text() {
            "hooks" => {
                let mut options = HooksOptions::default();
                self.map_to_array(&val, &name, &mut options, diagnostics)?;
                *self = PossibleOptions::Hooks(options);
            }
            "maxAllowedComplexity" => {
                let mut options = ComplexityOptions::default();
                options.visit_map(key, value, diagnostics)?;
                *self = PossibleOptions::Complexity(options);
            }
            "strictCase" | "enumMemberCase" => {
                let mut options = match self {
                    PossibleOptions::NamingConvention(options) => *options,
                    _ => NamingConventionOptions::default(),
                };
                options.visit_map(key, value, diagnostics)?;
                *self = PossibleOptions::NamingConvention(options);
            }
            _ => (),
        }
        Some(())
    }
}
