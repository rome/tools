use crate::js::expressions::static_member_expression::JsAnyStaticMemberLike;
use crate::parentheses::{AssignmentNode, NeedsParentheses};
use crate::prelude::*;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsStaticMemberAssignment, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberAssignment;

impl FormatNodeRule<JsStaticMemberAssignment> for FormatJsStaticMemberAssignment {
    fn fmt_fields(&self, node: &JsStaticMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        JsAnyStaticMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsStaticMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl AssignmentNode for JsStaticMemberAssignment {
    #[inline]
    fn resolve(&self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self.clone()))
    }

    #[inline]
    fn into_resolved(self) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsAnyAssignment(JsAnyAssignment::from(self))
    }
}

impl NeedsParentheses for JsStaticMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
