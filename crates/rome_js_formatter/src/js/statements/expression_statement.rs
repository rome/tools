use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

impl FormatNodeFields<JsExpressionStatement> for FormatNodeRule<JsExpressionStatement> {
    fn format_fields(
        node: &JsExpressionStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![formatter, [expression.format()]]?,
            semicolon_token,
        )
    }
}
