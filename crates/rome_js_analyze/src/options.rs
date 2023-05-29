//! This module contains the rules that have options

use crate::semantic_analyzers::nursery::use_exhaustive_dependencies::{
    hooks_options, HooksOptions,
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
    /// Options for `useExhaustiveDependencies` and `useHookAtTopLevel` rule
    Hooks(#[bpaf(external(hooks_options), hide)] HooksOptions),
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
    const KNOWN_KEYS: &'static [&'static str] = &["hooks"];

    pub fn extract_option(&self, rule_key: &RuleKey) -> RuleOptions {
        match rule_key.rule_name() {
            "useExhaustiveDependencies" | "useHookAtTopLevel" => {
                let options = match self {
                    PossibleOptions::Hooks(hooks) => hooks.clone(),
                    _ => HooksOptions::default(),
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
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();

        if name_text == "hooks" {
            let mut options = HooksOptions::default();
            self.map_to_array(&value, &name, &mut options, diagnostics);
            *self = PossibleOptions::Hooks(options);
        }

        Some(())
    }
}
