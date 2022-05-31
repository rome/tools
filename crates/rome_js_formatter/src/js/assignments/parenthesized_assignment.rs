use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsParenthesizedAssignment;
use rome_js_syntax::JsParenthesizedAssignmentFields;

impl FormatNodeFields<JsParenthesizedAssignment> for FormatNodeRule<JsParenthesizedAssignment> {
    fn format_fields(node: &JsParenthesizedAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = node.as_fields();

        write![
            f,
            [
                l_paren_token.format(),
                assignment.format(),
                r_paren_token.format(),
            ]
        ]
    }
}
