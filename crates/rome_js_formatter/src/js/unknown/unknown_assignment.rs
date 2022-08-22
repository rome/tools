use crate::prelude::*;

use crate::parentheses::{AssignmentNode, NeedsParentheses};
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsSyntaxKind, JsSyntaxNode, JsUnknownAssignment,
};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownAssignment;

impl FormatNodeRule<JsUnknownAssignment> for FormatJsUnknownAssignment {
    fn fmt_fields(
        &self,
        node: &JsUnknownAssignment,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsUnknownAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for JsUnknownAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self))
    }
}

impl NeedsParentheses for JsUnknownAssignment {
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        self.syntax().parent().map_or(false, |parent| {
            parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT
        })
    }
}
