use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsTypeAssertionAssignmentFields};

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::TsTypeAssertionAssignment;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionAssignment;

impl FormatNodeRule<TsTypeAssertionAssignment> for FormatTsTypeAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsTypeAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = node.as_fields();

        write![
            f,
            [
                format_delimited(&l_angle_token?, &ty.format(), &r_angle_token?)
                    .soft_block_indent(),
                assignment.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeAssertionAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeAssertionAssignment {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::TsTypeAssertionAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(<number>a) = 'test'", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>a)! = 'test'", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>(<any>a)) = 'test'", TsTypeAssertionAssignment[0]);
        assert_needs_parentheses!("(<number>(<any>a)) = 'test'", TsTypeAssertionAssignment[1]);
        assert_needs_parentheses!("++(<number>a)", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>a)--", TsTypeAssertionAssignment);
        assert_not_needs_parentheses!("({ a: <number>a } = { a: 5 })", TsTypeAssertionAssignment);
    }
}
