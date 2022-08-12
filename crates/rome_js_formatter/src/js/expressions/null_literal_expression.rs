use crate::prelude::*;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression, JsNullLiteralExpressionFields};
use rome_js_syntax::{JsNullLiteralExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsNullLiteralExpression;

impl FormatNodeRule<JsNullLiteralExpression> for FormatJsNullLiteralExpression {
    fn fmt_fields(&self, node: &JsNullLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsNullLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsNullLiteralExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for JsNullLiteralExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(self))
    }
}
