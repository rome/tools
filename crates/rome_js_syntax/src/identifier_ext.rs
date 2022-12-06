use crate::{JsIdentifierAssignment, JsReferenceIdentifier, JsSyntaxToken, JsxReferenceIdentifier};
use rome_rowan::{declare_node_union, SyntaxResult};

declare_node_union! {
    pub AnyReferenceIdentifier = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

impl AnyReferenceIdentifier {
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyReferenceIdentifier::JsReferenceIdentifier(node) => node.value_token(),
            AnyReferenceIdentifier::JsIdentifierAssignment(node) => node.name_token(),
            AnyReferenceIdentifier::JsxReferenceIdentifier(node) => node.value_token(),
        }
    }
}
