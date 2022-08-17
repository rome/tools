use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{
    JsAnyExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsParenthesizedExpression;

impl FormatNodeRule<JsParenthesizedExpression> for FormatJsParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &JsParenthesizedExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let expression = expression?;

        return write!(
            f,
            [
                format_removed(&l_paren_token?),
                expression.format(),
                format_removed(&r_paren_token?)
            ]
        );
    }

    fn needs_parentheses(&self, item: &JsParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsParenthesizedExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for JsParenthesizedExpression {
    fn resolve(&self) -> JsAnyExpression {
        let inner = self.expression();

        inner.unwrap_or_else(|_| self.clone().into())
    }

    fn into_resolved(self) -> JsAnyExpression {
        let inner = self.expression();

        inner.unwrap_or_else(|_| self.into())
    }
}
