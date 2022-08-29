use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::parentheses::NeedsParentheses;
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

        let l_paren_token = l_paren_token?;
        let expression = expression?;

        let should_hug = !(expression.syntax().has_comments_direct()
            || l_paren_token.has_trailing_comments())
            && (matches!(
                expression,
                JsAnyExpression::JsObjectExpression(_) | JsAnyExpression::JsArrayExpression(_)
            ));

        if should_hug {
            write!(
                f,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&expression.format()),
                    r_paren_token.format()
                ])]
            )
        }
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
