use crate::prelude::*;

use crate::parentheses::{AssignmentNode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{JsAnyAssignment, JsAnyAssignmentPattern, TsAsAssignment};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsAsAssignmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsAssignment;

impl FormatNodeRule<TsAsAssignment> for FormatTsAsAssignment {
    fn fmt_fields(&self, node: &TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = node.as_fields();

        write![
            f,
            [
                assignment.format(),
                space(),
                as_token.format(),
                space(),
                ty.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsAsAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for TsAsAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self))
    }
}

impl NeedsParentheses for TsAsAssignment {
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
    use rome_js_syntax::TsAsAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("a as number = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(a as number)! = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(<number>(a as number)) = 'test'", TsAsAssignment);
        assert_needs_parentheses!("++(a as number)", TsAsAssignment);
        assert_needs_parentheses!("(a as number)--", TsAsAssignment);
        assert_needs_parentheses!("({ a: a as number } = { a: 5 })", TsAsAssignment);
    }
}
