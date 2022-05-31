use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsNewExpression;
use rome_js_syntax::JsNewExpressionFields;

impl FormatNodeFields<JsNewExpression> for FormatNodeRule<JsNewExpression> {
    fn format_fields(
        node: &JsNewExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsNewExpressionFields {
            new_token,
            callee,
            type_arguments,
            arguments,
        } = node.as_fields();

        formatted![
            formatter,
            [
                new_token.format(),
                space_token(),
                callee.format(),
                type_arguments.format(),
                arguments
                    .format()
                    .or_format(|| formatted![formatter, [token("("), token(")")]]),
            ]
        ]
    }
}
