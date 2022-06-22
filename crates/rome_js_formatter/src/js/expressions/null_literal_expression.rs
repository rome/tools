use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNullLiteralExpression;
use rome_js_syntax::JsNullLiteralExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsNullLiteralExpression;

impl FormatNodeRule<JsNullLiteralExpression> for FormatJsNullLiteralExpression {
    fn fmt_fields(&self, node: &JsNullLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
