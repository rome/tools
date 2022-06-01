use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAssignmentExpression;
use rome_js_syntax::JsAssignmentExpressionFields;

impl FormatNodeFields<JsAssignmentExpression> for FormatNodeRule<JsAssignmentExpression> {
    fn format_fields(
        node: &JsAssignmentExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        Ok(group_elements(formatted![
            formatter,
            [
                left.format(),
                space_token(),
                operator_token.format(),
                line_suffix_boundary(),
                group_elements(soft_line_indent_or_space(formatted![
                    formatter,
                    [right.format()]
                ]?)),
            ]
        ]?))
    }
}
