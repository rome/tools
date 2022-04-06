use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_line_indent_or_space, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsAssignmentExpression;
use rome_js_syntax::JsAssignmentExpressionFields;

impl ToFormatElement for JsAssignmentExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = self.as_fields();

        Ok(group_elements(format_elements![
            left.format(formatter)?,
            space_token(),
            operator_token.format(formatter)?,
            group_elements(soft_line_indent_or_space(right.format(formatter)?)),
        ]))
    }
}
