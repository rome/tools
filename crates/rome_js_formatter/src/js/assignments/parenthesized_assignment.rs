use crate::parentheses::{AssignmentNode, NeedsParentheses};
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsAnyAssignmentPattern, JsParenthesizedAssignment};
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

impl AssignmentNode for JsParenthesizedAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        let assignment = self
            .assignment()
            .map_or_else(|_| self.clone().into(), |assignment| assignment.into());

        JsAnyAssignmentPattern::JsAnyAssignment(assignment)
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        let assignment = self
            .assignment()
            .map_or_else(|_| self.into(), |assignment| assignment.into());

        JsAnyAssignmentPattern::JsAnyAssignment(assignment)
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
