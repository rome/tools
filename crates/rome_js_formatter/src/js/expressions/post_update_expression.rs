use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

impl FormatNodeFields<JsPostUpdateExpression> for FormatNodeRule<JsPostUpdateExpression> {
    fn fmt_fields(node: &JsPostUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPostUpdateExpressionFields {
            operand,
            operator_token,
        } = node.as_fields();

        write![f, [operand.format(), operator_token.format(),]]
    }
}
