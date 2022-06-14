use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsExpressionSnipped;
use rome_js_syntax::JsExpressionSnippedFields;

impl FormatNodeFields<JsExpressionSnipped> for FormatNodeRule<JsExpressionSnipped> {
    fn fmt_fields(node: &JsExpressionSnipped, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionSnippedFields {
            expression,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                expression.format(),
                format_replaced(&eof_token?, &empty_element()),
            ]
        ]
    }
}
