use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

impl FormatNodeFields<JsExpressionSnipped> for FormatNodeRule<JsExpressionSnipped> {
    fn format_fields(
        node: &JsExpressionSnipped,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = node.as_fields();

        formatted![formatter, [expression.format(), eof_token.format(),]]
    }
}
