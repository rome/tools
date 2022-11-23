use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::TsSatisfiesAssignment;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsSatisfiesAssignmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesAssignment;

impl FormatNodeRule<TsSatisfiesAssignment> for FormatTsSatisfiesAssignment {
    fn fmt_fields(&self, node: &TsSatisfiesAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSatisfiesAssignmentFields {
            assignment,
            satisfies_token,
            ty,
        } = node.as_fields();

        write![
            f,
            [
                assignment.format(),
                space(),
                satisfies_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsSatisfiesAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsSatisfiesAssignment {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_needs_parentheses;
    use rome_js_syntax::TsSatisfiesAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("a satisfies number = 'test'", TsSatisfiesAssignment);
        assert_needs_parentheses!("(a satisfies number)! = 'test'", TsSatisfiesAssignment);
        assert_needs_parentheses!(
            "(<number>(a satisfies number)) = 'test'",
            TsSatisfiesAssignment
        );
        assert_needs_parentheses!("++(a satisfies number)", TsSatisfiesAssignment);
        assert_needs_parentheses!("(a satisfies number)--", TsSatisfiesAssignment);
        assert_needs_parentheses!(
            "({ a: a satisfies number } = { a: 5 })",
            TsSatisfiesAssignment
        );
    }
}
