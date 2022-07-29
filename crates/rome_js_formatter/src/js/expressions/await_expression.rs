use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsAwaitExpression;
use rome_js_syntax::JsAwaitExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsAwaitExpression;

impl FormatNodeRule<JsAwaitExpression> for FormatJsAwaitExpression {
    fn fmt_fields(&self, node: &JsAwaitExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = node.as_fields();

        write![f, [await_token.format(), space(), argument.format(),]]
    }
}
