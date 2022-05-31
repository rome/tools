use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsParenthesizedAssignment;
use rome_js_syntax::JsParenthesizedAssignmentFields;

impl FormatNodeFields<JsParenthesizedAssignment> for FormatNodeRule<JsParenthesizedAssignment> {
    fn format_fields(
        node: &JsParenthesizedAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                l_paren_token.format(),
                assignment.format(),
                r_paren_token.format(),
            ]
        ]
    }
}
