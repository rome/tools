use crate::ConfigurationDiagnostic;
use rome_rowan::{Language, SyntaxNode};

/// Generic trait to implement when resolving the configuration from a generic language
pub trait VisitConfigurationNode<L: Language> {
    /// Called when visiting the key of a member
    fn visit_member_name(&mut self, node: &SyntaxNode<L>) -> Result<(), ConfigurationDiagnostic> {
        Err(ConfigurationDiagnostic::unexpected(node.text_trimmed_range()))
    }
    /// Called when visiting the value of a member
    fn visit_member_value(&mut self, node: &SyntaxNode<L>) -> Result<(), ConfigurationDiagnostic> {
        Err(ConfigurationDiagnostic::unexpected(node.text_trimmed_range()))
    }

    /// Called when visiting a list of key-value.
    ///
    /// The implementor should loop through the list and call this function by passing two nodes,
    /// the key/name as first argument, and the value as second argument.
    fn visit_map(
        &mut self,
        key: &SyntaxNode<L>,
        value: &SyntaxNode<L>,
    ) -> Result<(), ConfigurationDiagnostic> {
        Err(ConfigurationDiagnostic::unexpected(
            key.text_trimmed_range().start()..value.text_trimmed_range().end(),
        ))
    }
}
