use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsAssignmentExpression;
use rome_js_syntax::JsAssignmentExpressionFields;

impl FormatNodeFields<JsAssignmentExpression> for FormatNodeRule<JsAssignmentExpression> {
    fn fmt_fields(node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [group_elements(&format_args![
                left.format(),
                space_token(),
                operator_token.format(),
                line_suffix_boundary(),
                group_elements(&soft_line_indent_or_space(&right.format())),
            ])]
        )
    }
}
