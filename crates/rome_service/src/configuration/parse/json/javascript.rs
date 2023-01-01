use crate::configuration::javascript::{
    PlainQuoteProperties, PlainQuoteStyle, PlainSemicolons, PlainTrailingComma,
};
use crate::configuration::parse::json::VisitConfigurationAsJson;
use crate::configuration::parse::json::{has_only_known_keys, with_only_known_variants};
use crate::configuration::visitor::VisitConfigurationNode;
use crate::configuration::{JavascriptConfiguration, JavascriptFormatter};
use crate::ConfigurationDiagnostic;
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;

impl VisitConfigurationAsJson for JavascriptConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for JavascriptConfiguration {
    fn visit_member_name(&mut self, node: &JsonSyntaxNode) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, JavascriptConfiguration::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();

        match name_text {
            "formatter" => {
                let mut javascript_formatter = JavascriptFormatter::default();
                self.map_to_object(&value, name_text, &mut javascript_formatter)?;
                self.formatter = Some(javascript_formatter);
            }
            "globals" => {
                self.globals = self.map_to_index_set_string(&value, name_text)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl VisitConfigurationAsJson for JavascriptFormatter {}
impl VisitConfigurationNode<JsonLanguage> for JavascriptFormatter {
    fn visit_member_name(&mut self, node: &JsonSyntaxNode) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, JavascriptFormatter::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();
        match name_text {
            "quoteStyle" => {
                let mut quote_style = PlainQuoteStyle::default();
                self.map_to_known_string(&value, name_text, &mut quote_style)?;
                self.quote_style = quote_style.into();
            }
            "trailingComma" => {
                let mut trailing_comma = PlainTrailingComma::default();
                self.map_to_known_string(&value, name_text, &mut trailing_comma)?;
                self.trailing_comma = trailing_comma.into();
            }
            "quoteProperties" => {
                let mut quote_properties = PlainQuoteProperties::default();
                self.map_to_known_string(&value, name_text, &mut quote_properties)?;
                self.quote_properties = quote_properties.into();
            }
            "semicolons" => {
                let mut semicolons = PlainSemicolons::default();
                self.map_to_known_string(&value, name_text, &mut semicolons)?;
                self.semicolons = semicolons.into();
            }
            _ => {}
        }

        Ok(())
    }
}

impl VisitConfigurationNode<JsonLanguage> for PlainQuoteStyle {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let node = with_only_known_variants(node, PlainQuoteStyle::KNOWN_VALUES)?;
        if node.value_token()?.text() == "single" {
            *self = PlainQuoteStyle::Single;
        } else {
            *self = PlainQuoteStyle::Double;
        }
        Ok(())
    }
}

impl VisitConfigurationNode<JsonLanguage> for PlainQuoteProperties {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let node = with_only_known_variants(node, PlainQuoteProperties::KNOWN_VALUES)?;
        if node.value_token()?.text() == "asNeeded" {
            *self = PlainQuoteProperties::AsNeeded;
        } else {
            *self = PlainQuoteProperties::Preserve;
        }
        Ok(())
    }
}

impl VisitConfigurationNode<JsonLanguage> for PlainTrailingComma {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let node = with_only_known_variants(node, PlainTrailingComma::KNOWN_VALUES)?;
        match node.value_token()?.text() {
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
        Ok(())
    }
}

impl VisitConfigurationNode<JsonLanguage> for PlainSemicolons {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let node = with_only_known_variants(node, PlainSemicolons::KNOWN_VALUES)?;
        if node.value_token()?.text() == "asNeeded" {
            *self = PlainSemicolons::AsNeeded;
        } else {
            *self = PlainSemicolons::Always;
        }
        Ok(())
    }
}
