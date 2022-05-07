use crate::prelude::*;

use rome_js_syntax::JsAssignmentExpression;
use rome_js_syntax::JsAssignmentExpressionFields;

impl FormatNode for JsAssignmentExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = self.as_fields();

        Ok(group_elements(formatted![
            formatter,
            left.format(formatter)?,
            space_token(),
            operator_token.format(formatter)?,
            group_elements(soft_line_indent_or_space(right.format(formatter)?)),
        ]?))
    }
}
