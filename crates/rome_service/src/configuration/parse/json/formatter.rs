use crate::configuration::diagnostics::{Deserialization, DeserializationAdvice};
use crate::configuration::parse::json::{
    has_only_known_keys, with_only_known_variants, VisitConfigurationAsJson,
};
use crate::configuration::visitor::VisitConfigurationNode;
use crate::configuration::{FormatterConfiguration, PlainIndentStyle};
use crate::ConfigurationDiagnostic;
use rome_console::markup;
use rome_diagnostics::MessageAndDescription;
use rome_formatter::LineWidth;
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxNode};

impl VisitConfigurationAsJson for FormatterConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for FormatterConfiguration {
    fn visit_member_name(&mut self, node: &JsonSyntaxNode) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, FormatterConfiguration::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();
        match name_text {
            "formatWithErrors" => {
                self.format_with_errors = self.map_to_boolean(&value, name_text)?;
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text)?;
            }
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text)?;
            }
            "indentStyle" => {
                let mut indent_style = PlainIndentStyle::default();
                self.map_to_known_string(&value, name_text, &mut indent_style)?;
                self.indent_style = indent_style;
            }
            "indentSize" => {
                self.indent_size = self.map_to_u8(&value, name_text, u8::MAX)?;
            }
            "lineWidth" => {
                let line_width = self.map_to_u16(&value, name_text, LineWidth::MAX)?;
                self.line_width = LineWidth::try_from(line_width).map_err(|err| {
                    ConfigurationDiagnostic::Deserialization(Deserialization {
                        reason: MessageAndDescription::from(err.to_string()),
                        range: Some(value.range()),
                        deserialization_advice: DeserializationAdvice {
                            hint: Some(
                                markup! {"Maximum value accepted is "{{LineWidth::MAX}}}.to_owned(),
                            ),
                            ..DeserializationAdvice::default()
                        },
                    })
                })?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl VisitConfigurationNode<JsonLanguage> for PlainIndentStyle {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let node = with_only_known_variants(node, PlainIndentStyle::KNOWN_VALUES)?;
        if node.value_token()?.text() == "space" {
            *self = PlainIndentStyle::Space;
        } else {
            *self = PlainIndentStyle::Tab;
        }
        Ok(())
    }
}
