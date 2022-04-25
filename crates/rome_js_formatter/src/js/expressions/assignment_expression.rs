use crate::{
    format_elements, group_elements, soft_line_indent_or_space, space_token, Format, FormatElement,
    FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsAssignmentExpression;
use rome_js_syntax::JsAssignmentExpressionFields;

impl FormatNode for JsAssignmentExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = self.as_fields();

        // EXAMPLE: Remove feature flag example before merging
        if rome_flags::unstable().new_linebreaking() {
            Ok(group_elements(format_elements![
                left.format(formatter)?,
                space_token(),
                operator_token.format(formatter)?,
                space_token(),
                right.format(formatter)?
            ]))
        } else {
            Ok(group_elements(format_elements![
                left.format(formatter)?,
                space_token(),
                operator_token.format(formatter)?,
                group_elements(soft_line_indent_or_space(right.format(formatter)?))
            ]))
        }
    }
}
