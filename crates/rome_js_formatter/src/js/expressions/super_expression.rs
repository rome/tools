use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsSuperExpression;
use rome_js_syntax::JsSuperExpressionFields;

impl FormatNodeFields<JsSuperExpression> for FormatNodeRule<JsSuperExpression> {
    fn fmt_fields(node: &JsSuperExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSuperExpressionFields { super_token } = node.as_fields();

        write![f, [super_token.format()]]
    }
}
