use crate::prelude::*;

use crate::parentheses::{AssignmentNode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::TsNonNullAssertionAssignmentFields;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsSyntaxNode, TsNonNullAssertionAssignment,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionAssignment;

impl FormatNodeRule<TsNonNullAssertionAssignment> for FormatTsNonNullAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        write![f, [assignment.format(), excl_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNonNullAssertionAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for TsNonNullAssertionAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self))
    }
}

impl NeedsParentheses for TsNonNullAssertionAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
