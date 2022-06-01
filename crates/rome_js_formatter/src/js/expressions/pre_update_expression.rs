use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsPreUpdateExpression;
use rome_js_syntax::JsPreUpdateExpressionFields;

impl FormatNodeFields<JsPreUpdateExpression> for FormatNodeRule<JsPreUpdateExpression> {
    fn format_fields(
        node: &JsPreUpdateExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = node.as_fields();

        formatted![formatter, [operator_token.format(), operand.format(),]]
    }
}
