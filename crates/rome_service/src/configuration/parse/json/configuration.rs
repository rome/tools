use crate::configuration::{
    FilesConfiguration, FormatterConfiguration, JavascriptConfiguration, LinterConfiguration,
};
use crate::Configuration;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;
use std::num::NonZeroU64;
use crate::configuration::organize_imports::OrganizeImports;

impl VisitJsonNode for FilesConfiguration {}

impl VisitNode<JsonLanguage> for FilesConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, FilesConfiguration::KNOWN_KEYS, diagnostics)
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
            "maxSize" => {
                self.max_size =
                    NonZeroU64::new(self.map_to_u64(&value, name_text, u64::MAX, diagnostics)?);
            }
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text, diagnostics);
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitJsonNode for Configuration {}

impl VisitNode<JsonLanguage> for Configuration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Configuration::KNOWN_KEYS, diagnostics)
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
            "$schema" => {
                self.schema = Some(self.map_to_string(&value, name_text, diagnostics)?);
            }
            "files" => {
                let mut files = FilesConfiguration::default();
                self.map_to_object(&value, name_text, &mut files, diagnostics)?;
                self.files = Some(files);
            }
            "formatter" => {
                let mut formatter = FormatterConfiguration::default();
                self.map_to_object(&value, name_text, &mut formatter, diagnostics)?;
                self.formatter = Some(formatter);
            }
            "linter" => {
                let mut linter = LinterConfiguration::default();
                self.map_to_object(&value, name_text, &mut linter, diagnostics)?;
                self.linter = Some(linter);
            }
            "javascript" => {
                let mut javascript = JavascriptConfiguration::default();
                self.map_to_object(&value, name_text, &mut javascript, diagnostics)?;
                self.javascript = Some(javascript);
            }
            "organizeImports" => {
                let mut organize_imports = OrganizeImports::default();
                self.map_to_object(&value, name_text, &mut organize_imports, diagnostics)?;
                self.organize_imports = Some(organize_imports);
            }
            _ => {}
        }

        Some(())
    }
}
