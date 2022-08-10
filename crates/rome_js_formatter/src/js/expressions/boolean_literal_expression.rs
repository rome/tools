use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::JsBooleanLiteralExpressionFields;
use rome_js_syntax::{JsBooleanLiteralExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsBooleanLiteralExpression;

impl FormatNodeRule<JsBooleanLiteralExpression> for FormatJsBooleanLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsBooleanLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBooleanLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsBooleanLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsBooleanLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        false
    }
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
