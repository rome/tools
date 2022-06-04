use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsBooleanLiteralExpression;
use rome_js_syntax::JsBooleanLiteralExpressionFields;

impl FormatNodeFields<JsBooleanLiteralExpression> for FormatNodeRule<JsBooleanLiteralExpression> {
    fn fmt_fields(node: &JsBooleanLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsBooleanLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
