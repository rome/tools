use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsParenthesizedAssignment;
use rome_js_syntax::{JsParenthesizedAssignmentFields, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsParenthesizedAssignment;

impl FormatNodeRule<JsParenthesizedAssignment> for FormatJsParenthesizedAssignment {
    fn fmt_fields(
        &self,
        node: &JsParenthesizedAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = node.as_fields();

        write![
            f,
            [
                format_removed(&l_paren_token?),
                assignment.format(),
                format_removed(&r_paren_token?),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &JsParenthesizedAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsParenthesizedAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
