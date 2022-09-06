use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{JsSyntaxNode, JsUnknownExpression};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownExpression;

impl FormatNodeRule<JsUnknownExpression> for FormatJsUnknownExpression {
    fn fmt_fields(
        &self,
        node: &JsUnknownExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsUnknownExpression) -> bool {
        item.needs_parentheses()
    }

    fn prints_comments(&self, _item: &JsUnknownExpression) -> bool {
        true
    }
}

impl NeedsParentheses for JsUnknownExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        self.needs_parentheses()
    }
}
