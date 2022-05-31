use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

impl FormatNodeFields<JsYieldExpression> for FormatNodeRule<JsYieldExpression> {
    fn format_fields(
        node: &JsYieldExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = node.as_fields();

        formatted![formatter, [yield_token.format(), argument.format()]]
    }
}
