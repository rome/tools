use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNumberLiteralExpression;
use rome_js_syntax::JsNumberLiteralExpressionFields;

impl FormatNodeFields<JsNumberLiteralExpression> for FormatNodeRule<JsNumberLiteralExpression> {
    fn format_fields(
        node: &JsNumberLiteralExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNumberLiteralExpressionFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
