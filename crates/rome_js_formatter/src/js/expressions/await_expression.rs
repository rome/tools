use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsAwaitExpression;
use rome_js_syntax::JsAwaitExpressionFields;

impl FormatNodeFields<JsAwaitExpression> for FormatNodeRule<JsAwaitExpression> {
    fn fmt_fields(node: &JsAwaitExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = node.as_fields();

        write![f, [await_token.format(), space_token(), argument.format(),]]
    }
}
