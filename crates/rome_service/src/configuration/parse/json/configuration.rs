use crate::configuration::parse::json::{has_only_known_keys, VisitConfigurationAsJson};
use crate::configuration::visitor::VisitConfigurationNode;
use crate::configuration::{
    FilesConfiguration, FormatterConfiguration, JavascriptConfiguration, LinterConfiguration,
};
use crate::{Configuration, ConfigurationDiagnostic};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;
use std::num::NonZeroU64;

impl VisitConfigurationAsJson for FilesConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for FilesConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, FilesConfiguration::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();
        match name_text {
            "maxSize" => {
                self.max_size = NonZeroU64::new(self.map_to_u64(&value, name_text, u64::MAX)?);
            }
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl VisitConfigurationAsJson for Configuration {}

impl VisitConfigurationNode<JsonLanguage> for Configuration {
    fn visit_member_name(&mut self, node: &JsonSyntaxNode) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, Configuration::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();
        match name_text {
            "$schema" => {
                self.schema = Some(self.map_to_string(&value, name_text)?);
            }
            "files" => {
                let mut files = FilesConfiguration::default();
                self.map_to_object(&value, name_text, &mut files)?;
                self.files = Some(files);
            }
            "formatter" => {
                let mut formatter = FormatterConfiguration::default();
                self.map_to_object(&value, name_text, &mut formatter)?;
                self.formatter = Some(formatter);
            }
            "linter" => {
                let mut linter = LinterConfiguration::default();
                self.map_to_object(&value, name_text, &mut linter)?;
                self.linter = Some(linter);
            }
            "javascript" => {
                let mut javascript = JavascriptConfiguration::default();
                self.map_to_object(&value, name_text, &mut javascript)?;
                self.javascript = Some(javascript);
            }
            _ => {}
        }

        Ok(())
    }
}
