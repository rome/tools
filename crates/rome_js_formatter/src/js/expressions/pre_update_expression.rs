use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsPreUpdateExpression;
use rome_js_syntax::JsPreUpdateExpressionFields;

impl FormatNodeFields<JsPreUpdateExpression> for FormatNodeRule<JsPreUpdateExpression> {
    fn fmt_fields(node: &JsPreUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = node.as_fields();

        write![f, [operator_token.format(), operand.format(),]]
    }
}
