use crate::configuration::parse::json::{has_only_known_keys, VisitConfigurationAsJson};
use crate::configuration::visitor::VisitConfigurationNode;
use crate::configuration::LinterConfiguration;
use crate::ConfigurationDiagnostic;
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;

impl VisitConfigurationAsJson for LinterConfiguration {}

impl VisitConfigurationNode<JsonLanguage> for LinterConfiguration {
    fn visit_member_name(&mut self, node: &JsonSyntaxNode) -> Result<(), ConfigurationDiagnostic> {
        has_only_known_keys(node, LinterConfiguration::KNOWN_KEYS)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
    ) -> Result<(), ConfigurationDiagnostic> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.text();
        match name_text {
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text)?;
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text)?;
            }
            "rules" => {}
            _ => {}
        }

        Ok(())
    }
}
