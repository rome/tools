use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsUnknownAssignment};
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

impl NeedsParentheses for JsUnknownAssignment {
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        self.syntax().parent().map_or(false, |parent| {
            parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT
        })
    }
}
