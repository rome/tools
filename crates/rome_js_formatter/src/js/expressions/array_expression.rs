use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsArrayExpression;
use rome_js_syntax::JsArrayExpressionFields;

impl FormatNodeFields<JsArrayExpression> for FormatNodeRule<JsArrayExpression> {
    fn format_fields(
        node: &JsArrayExpression,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsArrayExpressionFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            formatted![formatter, [elements.format()]]?,
            &r_brack_token?,
        )
    }
}
