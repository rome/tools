use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNullLiteralExpression;
use rome_js_syntax::JsNullLiteralExpressionFields;

impl FormatNodeFields<JsNullLiteralExpression> for FormatNodeRule<JsNullLiteralExpression> {
    fn format_fields(
        node: &JsNullLiteralExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        formatted![formatter, [value_token.format()]]
    }
}
