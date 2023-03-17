use crate::configuration::organize_imports::OrganizeImports;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxNode;

impl VisitJsonNode for OrganizeImports {}

impl VisitNode<JsonLanguage> for OrganizeImports {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["enabled"], diagnostics)
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
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics)?;
            }
            "ignore" => {
                self.ignore = self.map_to_index_set_string(&value, name_text, diagnostics);
            }
            _ => {}
        }

        Some(())
    }
}
