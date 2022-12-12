//! This module is responsible to parse the configuration from a JSON format
//!

mod configuration;
mod formatter;
mod javascript;
mod linter;

use crate::configuration::visitor::VisitConfigurationNode;
use crate::ConfigurationDiagnostic;
use indexmap::IndexSet;
use rome_console::markup;
use rome_json_syntax::{
    AnyJsonValue, JsonArrayValue, JsonBooleanValue, JsonLanguage, JsonMemberName, JsonNumberValue,
    JsonObjectValue, JsonRoot, JsonStringValue, JsonSyntaxNode,
};
use rome_rowan::{AstNode, AstSeparatedList, SyntaxNodeCast, SyntaxTokenText, TextRange};
use std::num::ParseIntError;

pub fn parse_configuration_from_json(
    root: JsonRoot,
    visitor: &mut impl VisitConfigurationNode<JsonLanguage>,
) -> Result<(), ConfigurationDiagnostic> {
    let value = root.value()?;
    match value {
        AnyJsonValue::JsonObjectValue(node) => {
            for element in node.json_member_list() {
                let element = element?;
                let member_name = element.name()?;
                let member_value = element.value()?;
                visitor.visit_map(member_name.syntax(), member_value.syntax())?;
            }
            Ok(())
        }
        _ => Err(ConfigurationDiagnostic::new_deserialization_error(
            "The configuration should be an object",
        )
        .with_span(root.range())),
    }
}

/// Convenient function to check if the current [JsonMemberName] belongs to a sub set of
/// `allowed_keys`
fn has_only_known_keys(
    node: &JsonSyntaxNode,
    allowed_keys: &[&str],
) -> Result<(), ConfigurationDiagnostic> {
    node.clone()
        .cast::<JsonMemberName>()
        .map(|node| {
            let key_name = node.inner_string_text()?;
            if allowed_keys.contains(&key_name.text()) {
                Ok(())
            } else {
                Err(ConfigurationDiagnostic::new_unknown_member(key_name.text())
                    .with_span(node.range())
                    .with_known_keys("Accepted keys", allowed_keys))
            }
        })
        .unwrap_or_else(|| Err(ConfigurationDiagnostic::new_syntax_error()))
}

/// Convenient function that returns a [JsonStringValue] from a generic node, and checks
/// if it's content matches the `allowed_keys`.
///
/// Useful when when you're parsing an `enum` and you still need to verify the value of the node, but
/// still need it.
fn with_only_known_variants(
    node: &JsonSyntaxNode,
    allowed_keys: &[&str],
) -> Result<JsonStringValue, ConfigurationDiagnostic> {
    node.clone()
        .cast::<JsonStringValue>()
        .map(|node| {
            let key_name = node.value_token()?;
            if allowed_keys.contains(&key_name.text_trimmed()) {
                Ok(node)
            } else {
                Err(
                    ConfigurationDiagnostic::new_unknown_variant(key_name.text_trimmed())
                        .with_span(node.range())
                        .with_known_keys("Accepted variants", allowed_keys),
                )
            }
        })
        .unwrap_or_else(|| Err(ConfigurationDiagnostic::new_syntax_error()))
}

/// Convenient trait that contains utility functions to work with [JsonLanguage]
pub(crate) trait VisitConfigurationAsJson: VisitConfigurationNode<JsonLanguage> {
    /// Convenient function to use inside [visit_map].
    ///
    /// It casts key to [JsonMemberName] and verifies that key name is correct by calling
    /// [visit_member_name].
    ///
    /// It casts the value to [AnyJsonValue].
    ///
    /// ## Errors
    ///
    /// The function will emit a generic diagnostic if [visit_member_name] is not implemented by
    /// the visitor that calls this function.
    fn get_key_and_value(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
    ) -> Result<(SyntaxTokenText, AnyJsonValue), ConfigurationDiagnostic> {
        let member = key
            .clone()
            .cast::<JsonMemberName>()
            .ok_or_else(ConfigurationDiagnostic::new_syntax_error)?;
        self.visit_member_name(member.syntax())?;
        let name = member.inner_string_text()?;
        let value = value
            .clone()
            .cast::<AnyJsonValue>()
            .ok_or_else(ConfigurationDiagnostic::new_syntax_error)?;

        Ok((name, value))
    }

    /// It attempts to map a [AnyJsonValue] to a string.
    ///
    /// Use this function when you want to map a string to an enum type.
    ///
    /// ## Errors
    ///
    /// The function will emit a generic diagnostic if the `visitor` doesn't implement [visit_member_value]
    fn map_to_known_string<T>(
        &self,
        value: &AnyJsonValue,
        name: &str,
        visitor: &mut T,
    ) -> Result<(), ConfigurationDiagnostic>
    where
        T: VisitConfigurationNode<JsonLanguage>,
    {
        let value = JsonStringValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "string")
                .with_span(value.range())
        })?;

        visitor.visit_member_value(value.syntax())?;
        Ok(())
    }

    /// It attempts to map a [AnyJsonValue] to a [String].
    ///
    /// ## Errors
    ///
    /// It emits an error if `value` can't be cast to a [JsonStringValue]
    fn map_to_string(
        &self,
        value: &AnyJsonValue,
        name: &str,
    ) -> Result<String, ConfigurationDiagnostic> {
        let value = JsonStringValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "string")
                .with_span(value.range())
        })?;
        Ok(value.text())
    }

    /// It attempts to map a [AnyJsonValue] to a [u8].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u8]
    fn map_to_u8(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u8,
    ) -> Result<u8, ConfigurationDiagnostic> {
        let value = JsonNumberValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "number")
                .with_span(value.range())
        })?;
        let value = value.value_token()?;
        value.text_trimmed().parse::<u8>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        })
    }

    /// It attempts to map a [AnyJsonValue] to a [u16].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u16]
    fn map_to_u16(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u16,
    ) -> Result<u16, ConfigurationDiagnostic> {
        let value = JsonNumberValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "number")
                .with_span(value.range())
        })?;
        let value = value.value_token()?;

        value.text_trimmed().parse::<u16>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        })
    }

    /// It attempts to map a [AnyJsonValue] to a [u64].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u64]
    fn map_to_u64(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u64,
    ) -> Result<u64, ConfigurationDiagnostic> {
        let value = JsonNumberValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "number")
                .with_span(value.range())
        })?;
        let value = value.value_token()?;

        value.text_trimmed().parse::<u64>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        })
    }

    /// It attempts to cast [AnyJsonValue] to a [bool]
    ///
    /// ## Errors
    ///
    /// The function emits a diagnostic if `value` can't be cast to [JsonBooleanValue]
    fn map_to_boolean(
        &self,
        value: &AnyJsonValue,
        name: &str,
    ) -> Result<bool, ConfigurationDiagnostic> {
        let value = JsonBooleanValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "boolean")
                .with_span(value.range())
        })?;
        Ok(value.value_token()?.text() == "true")
    }

    /// It attempts to map a [AnyJsonValue] to a [IndexSet] of [String].
    ///
    /// ## Errors
    ///
    /// The function emit diagnostics if:
    /// - `value` can't be cast to [JsonArrayValue]
    /// - any element of the of the array can't be cast to [JsonStringValue]
    fn map_to_index_set_string(
        &self,
        value: &AnyJsonValue,
        name: &str,
    ) -> Result<Option<IndexSet<String>>, ConfigurationDiagnostic> {
        let array = JsonArrayValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "array")
                .with_span(value.range())
        })?;
        let mut elements = IndexSet::new();
        if array.elements().is_empty() {
            return Ok(None);
        }
        for element in array.elements() {
            let element = element?;
            match element {
                AnyJsonValue::JsonStringValue(value) => {
                    elements.insert(value.value_token()?.to_string());
                }
                _ => {
                    return Err(ConfigurationDiagnostic::new_incorrect_type("string")
                        .with_span(element.range()))
                }
            }
        }

        Ok(Some(elements))
    }

    /// It attempts to map [AnyJsonValue] to a generic map.
    ///
    /// Use this function when the value of your member is another object, and this object
    /// needs to be mapped to another type.
    ///
    /// This function will loop though the list of elements and call [visit_map] on each pair
    /// of `name` and `value`.
    ///
    /// ## Errors
    /// This function will emit diagnostics if:
    /// - the `value` can't be cast to [JsonObjectValue]
    fn map_to_object<T>(
        &mut self,
        value: &AnyJsonValue,
        name: &str,
        visitor: &mut T,
    ) -> Result<(), ConfigurationDiagnostic>
    where
        T: VisitConfigurationNode<JsonLanguage>,
    {
        let value = JsonObjectValue::cast_ref(value.syntax()).ok_or_else(|| {
            ConfigurationDiagnostic::new_incorrect_type_for_value(name, "object")
                .with_span(value.range())
        })?;
        for element in value.json_member_list() {
            let element = element?;
            visitor.visit_map(element.name()?.syntax(), element.value()?.syntax())?;
        }
        Ok(())
    }
}

fn emit_diagnostic_form_number(
    parse_error: ParseIntError,
    value_text: &str,
    value_range: TextRange,
    maximum: impl rome_console::fmt::Display,
) -> ConfigurationDiagnostic {
    if value_text.starts_with("-") {
        ConfigurationDiagnostic::new_deserialization_error(parse_error.to_string())
            .with_span(value_range)
            .with_hint(markup! {"Value can't be negative"})
    } else {
        ConfigurationDiagnostic::new_deserialization_error(parse_error.to_string())
            .with_span(value_range)
            .with_hint(markup! {"Maximum value accepted is "{{maximum}}})
    }
}
