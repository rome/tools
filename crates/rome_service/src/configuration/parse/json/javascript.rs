use crate::configuration::javascript::JavascriptOrganizeImports;
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_js_formatter::context::trailing_comma::TrailingComma;
use rome_js_formatter::context::{QuoteProperties, QuoteStyle, Semicolons};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;

impl VisitJsonNode for JavascriptConfiguration {}

impl VisitNode<JsonLanguage> for JavascriptConfiguration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JavascriptConfiguration::KNOWN_KEYS, diagnostics)
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
            "formatter" => {
                let mut javascript_formatter = JavascriptFormatter::default();
                self.map_to_object(&value, name_text, &mut javascript_formatter, diagnostics)?;
                self.formatter = Some(javascript_formatter);
            }
            "globals" => {
                self.globals = self.map_to_index_set_string(&value, name_text, diagnostics);
            }
            "organizeImports" => {
                let mut javascript_organize_imports = JavascriptOrganizeImports::default();
                self.map_to_object(
                    &value,
                    name_text,
                    &mut javascript_organize_imports,
                    diagnostics,
                )?;
                self.organize_imports = Some(javascript_organize_imports);
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for JavascriptFormatter {}
impl VisitNode<JsonLanguage> for JavascriptFormatter {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, JavascriptFormatter::KNOWN_KEYS, diagnostics)
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
            "quoteStyle" => {
                let mut quote_style = QuoteStyle::default();
                self.map_to_known_string(&value, name_text, &mut quote_style, diagnostics)?;
                self.quote_style = quote_style;
            }
            "trailingComma" => {
                let mut trailing_comma = TrailingComma::default();
                self.map_to_known_string(&value, name_text, &mut trailing_comma, diagnostics)?;
                self.trailing_comma = trailing_comma;
            }
            "quoteProperties" => {
                let mut quote_properties = QuoteProperties::default();
                self.map_to_known_string(&value, name_text, &mut quote_properties, diagnostics)?;
                self.quote_properties = quote_properties;
            }
            "semicolons" => {
                let mut semicolons = Semicolons::default();
                self.map_to_known_string(&value, name_text, &mut semicolons, diagnostics)?;
                self.semicolons = semicolons;
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitJsonNode for JavascriptOrganizeImports {}
impl VisitNode<JsonLanguage> for JavascriptOrganizeImports {
    fn visit_map(
        &mut self,
        _key: &JsonSyntaxNode,
        _value: &JsonSyntaxNode,
        _diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }
}
