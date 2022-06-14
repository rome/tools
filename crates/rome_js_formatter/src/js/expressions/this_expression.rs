use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsThisExpression;
use rome_js_syntax::JsThisExpressionFields;

impl FormatNodeFields<JsThisExpression> for FormatNodeRule<JsThisExpression> {
    fn fmt_fields(node: &JsThisExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsThisExpressionFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }
}
