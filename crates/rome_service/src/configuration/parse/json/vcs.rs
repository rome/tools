use crate::configuration::vcs::{VcsClientKind, VcsConfiguration};
use rome_deserialize::json::{has_only_known_keys, with_only_known_variants, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::JsonLanguage;
use rome_rowan::SyntaxNode;

impl VisitJsonNode for VcsConfiguration {}

impl VisitNode<JsonLanguage> for VcsConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, VcsConfiguration::KNOWN_KEYS, diagnostics)
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
            "clientKind" => {
                let mut client_kind = VcsClientKind::default();
                self.map_to_known_string(&value, name_text, &mut client_kind, diagnostics)?;
                self.client_kind = Some(client_kind);
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics)?;
            }
            "useIgnoreFile" => {
                self.use_ignore_file = self.map_to_boolean(&value, name_text, diagnostics);
            }

            "root" => {
                self.root = self.map_to_string(&value, name_text, diagnostics);
            }
            _ => {}
        }
        Some(())
    }
}

impl VisitJsonNode for VcsClientKind {}

impl VisitNode<JsonLanguage> for VcsClientKind {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, VcsClientKind::KNOWN_VALUES, diagnostics)?;
        if node.inner_string_text().ok()?.text() == "git" {
            *self = VcsClientKind::Git;
        }
        Some(())
    }
}
