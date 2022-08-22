use crate::prelude::*;

use crate::js::expressions::computed_member_expression::JsAnyComputedMemberLike;
use crate::parentheses::{AssignmentNode, NeedsParentheses};
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsComputedMemberAssignment, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsComputedMemberAssignment;

impl FormatNodeRule<JsComputedMemberAssignment> for FormatJsComputedMemberAssignment {
    fn fmt_fields(
        &self,
        node: &JsComputedMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        JsAnyComputedMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsComputedMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for JsComputedMemberAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self))
    }
}

impl NeedsParentheses for JsComputedMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
