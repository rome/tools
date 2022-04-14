use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_formatter::{if_group_breaks, if_group_fits_on_single_line, soft_line_indent_or_space};
use rome_js_syntax::{JsAnyExpression, JsAssignmentExpression, JsAssignmentExpressionFields};

impl ToFormatElement for JsAssignmentExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = self.as_fields();

        let right = right?;

        let right = if !matches!(right, JsAnyExpression::JsAssignmentExpression(_)) {
            format_elements![space_token(), right.format(formatter)?]
        } else {
            group_elements(soft_line_indent_or_space(right.format(formatter)?))
        };

        Ok(group_elements(format_elements![
            left.format(formatter)?,
            space_token(),
            operator_token.format(formatter)?,
            right,
        ]))
    }
}
