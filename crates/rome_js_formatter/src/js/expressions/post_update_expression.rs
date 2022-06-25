use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsPostUpdateExpression;

impl FormatNodeRule<JsPostUpdateExpression> for FormatJsPostUpdateExpression {
    fn fmt_fields(&self, node: &JsPostUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPostUpdateExpressionFields {
            operand,
            operator_token,
        } = node.as_fields();

        write![f, [operand.format(), operator_token.format(),]]
    }
}
