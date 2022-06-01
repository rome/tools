use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsSuperExpression;
use rome_js_syntax::JsSuperExpressionFields;

impl FormatNodeFields<JsSuperExpression> for FormatNodeRule<JsSuperExpression> {
    fn format_fields(
        node: &JsSuperExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsSuperExpressionFields { super_token } = node.as_fields();

        formatted![formatter, [super_token.format()]]
    }
}
