use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

impl FormatNodeFields<JsYieldExpression> for FormatNodeRule<JsYieldExpression> {
    fn fmt_fields(node: &JsYieldExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = node.as_fields();

        write![f, [yield_token.format(), argument.format()]]
    }
}
