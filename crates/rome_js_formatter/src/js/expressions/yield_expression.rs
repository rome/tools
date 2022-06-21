use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsYieldExpression;
use rome_js_syntax::JsYieldExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsYieldExpression;

impl FormatNodeRule<JsYieldExpression> for FormatJsYieldExpression {
    fn fmt_fields(&self, node: &JsYieldExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = node.as_fields();

        write![f, [yield_token.format(), argument.format()]]
    }
}
