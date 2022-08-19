use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{
    JsParenthesizedExpression, JsParenthesizedExpressionFields, JsSyntaxNode,
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

        write!(
            f,
            [
                l_paren_token.format(),
                expression.format(),
                r_paren_token.format()
            ]
        )
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
