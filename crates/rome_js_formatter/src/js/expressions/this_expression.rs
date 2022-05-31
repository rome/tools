use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsThisExpression;
use rome_js_syntax::JsThisExpressionFields;

impl FormatNodeFields<JsThisExpression> for FormatNodeRule<JsThisExpression> {
    fn format_fields(
        node: &JsThisExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsThisExpressionFields { this_token } = node.as_fields();

        formatted![formatter, [this_token.format()]]
    }
}
