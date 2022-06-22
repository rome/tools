use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsThisExpression;
use rome_js_syntax::JsThisExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsThisExpression;

impl FormatNodeRule<JsThisExpression> for FormatJsThisExpression {
    fn fmt_fields(&self, node: &JsThisExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsThisExpressionFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }
}
