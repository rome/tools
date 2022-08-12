use crate::prelude::*;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{
    JsAnyExpression, JsParenthesizedExpression, JsSyntaxNode, JsUnknownExpression,
};
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
}

impl NeedsParentheses for JsUnknownExpression {
    fn needs_parentheses(&self) -> bool {
        // Keep parens if it is parenthesized.
        self.syntax().parent().map_or(false, |parent| {
            JsParenthesizedExpression::can_cast(parent.kind())
        })
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        self.needs_parentheses()
    }
}

impl ExpressionNode for JsUnknownExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}
