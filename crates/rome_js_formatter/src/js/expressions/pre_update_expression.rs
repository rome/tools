use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsPreUpdateExpression;
use rome_js_syntax::JsPreUpdateExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsPreUpdateExpression;

impl FormatNodeRule<JsPreUpdateExpression> for FormatJsPreUpdateExpression {
    fn fmt_fields(&self, node: &JsPreUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = node.as_fields();

        write![f, [operator_token.format(), operand.format(),]]
    }
}
