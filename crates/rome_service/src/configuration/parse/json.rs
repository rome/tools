//! This module is responsible to parse the configuration from a JSON format
//!

mod configuration;
mod formatter;
mod javascript;
mod linter;
mod rules;

use crate::Configuration;
use rome_deserialize::json::{JsonDeserialize, VisitConfigurationAsJson};
use rome_deserialize::DeserializationDiagnostic;
use rome_json_syntax::{AnyJsonValue, JsonRoot};
use rome_rowan::AstNode;

impl JsonDeserialize for Configuration {
    fn parse_from_json(
        root: JsonRoot,
        visitor: &mut impl VisitConfigurationAsJson,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = root.value().ok()?;
        match value {
            AnyJsonValue::JsonObjectValue(node) => {
                for element in node.json_member_list() {
                    let element = element.ok()?;
                    let member_name = element.name().ok()?;
                    let member_value = element.value().ok()?;
                    visitor.visit_map(member_name.syntax(), member_value.syntax(), diagnostics)?;
                }
                Some(())
            }
            _ => {
                diagnostics.push(
                    DeserializationDiagnostic::new("The configuration should be an object")
                        .with_range(root.range()),
                );
                None
            }
        }
    }
}
