use crate::configuration::javascript::{
    JavascriptOrganizeImports, PlainQuoteProperties, PlainQuoteStyle, PlainSemicolons,
    PlainTrailingComma,
};
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use rome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
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
                let mut quote_style = PlainQuoteStyle::default();
                self.map_to_known_string(&value, name_text, &mut quote_style, diagnostics)?;
                self.quote_style = quote_style.into();
            }
            "trailingComma" => {
                let mut trailing_comma = PlainTrailingComma::default();
                self.map_to_known_string(&value, name_text, &mut trailing_comma, diagnostics)?;
                self.trailing_comma = trailing_comma.into();
            }
            "quoteProperties" => {
                let mut quote_properties = PlainQuoteProperties::default();
                self.map_to_known_string(&value, name_text, &mut quote_properties, diagnostics)?;
                self.quote_properties = quote_properties.into();
            }
            "semicolons" => {
                let mut semicolons = PlainSemicolons::default();
                self.map_to_known_string(&value, name_text, &mut semicolons, diagnostics)?;
                self.semicolons = semicolons.into();
            }
            _ => {}
        }

        Some(())
    }
}

impl VisitNode<JsonLanguage> for PlainQuoteStyle {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, PlainQuoteStyle::KNOWN_VALUES, diagnostics)?;
        if node.inner_string_text().ok()?.text() == "single" {
            *self = PlainQuoteStyle::Single;
        } else {
            *self = PlainQuoteStyle::Double;
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for PlainQuoteProperties {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, PlainQuoteProperties::KNOWN_VALUES, diagnostics)?;
        if node.inner_string_text().ok()?.text() == "asNeeded" {
            *self = PlainQuoteProperties::AsNeeded;
        } else {
            *self = PlainQuoteProperties::Preserve;
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for PlainTrailingComma {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, PlainTrailingComma::KNOWN_VALUES, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "all" => {
                *self = PlainTrailingComma::All;
            }
            "es5" => {
                *self = PlainTrailingComma::ES5;
            }
            "none" => {
                *self = PlainTrailingComma::None;
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitNode<JsonLanguage> for PlainSemicolons {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, PlainSemicolons::KNOWN_VALUES, diagnostics)?;
        if node.inner_string_text().ok()?.text() == "asNeeded" {
            *self = PlainSemicolons::AsNeeded;
        } else {
            *self = PlainSemicolons::Always;
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
