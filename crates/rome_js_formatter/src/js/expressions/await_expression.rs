use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAwaitExpression;
use rome_js_syntax::JsAwaitExpressionFields;

impl FormatNodeFields<JsAwaitExpression> for FormatNodeRule<JsAwaitExpression> {
    fn format_fields(
        node: &JsAwaitExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = node.as_fields();

        formatted![
            formatter,
            [await_token.format(), space_token(), argument.format(),]
        ]
    }
}
