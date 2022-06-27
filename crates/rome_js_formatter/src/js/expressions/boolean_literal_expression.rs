use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsBooleanLiteralExpression;
use rome_js_syntax::JsBooleanLiteralExpressionFields;

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
}
