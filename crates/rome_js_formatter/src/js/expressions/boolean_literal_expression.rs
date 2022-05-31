use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsBooleanLiteralExpression;
use rome_js_syntax::JsBooleanLiteralExpressionFields;

impl FormatNodeFields<JsBooleanLiteralExpression> for FormatNodeRule<JsBooleanLiteralExpression> {
    fn format_fields(
        node: &JsBooleanLiteralExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsBooleanLiteralExpressionFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
