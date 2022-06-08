use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsNumberLiteralExpression;
use rome_js_syntax::JsNumberLiteralExpressionFields;

impl FormatNodeFields<JsNumberLiteralExpression> for FormatNodeRule<JsNumberLiteralExpression> {
    fn fmt_fields(node: &JsNumberLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNumberLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
