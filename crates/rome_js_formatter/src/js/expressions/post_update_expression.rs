use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

impl FormatNodeFields<JsPostUpdateExpression> for FormatNodeRule<JsPostUpdateExpression> {
    fn format_fields(
        node: &JsPostUpdateExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionFields {
            operand,
            operator_token,
        } = node.as_fields();

        formatted![formatter, [operand.format(), operator_token.format(),]]
    }
}
