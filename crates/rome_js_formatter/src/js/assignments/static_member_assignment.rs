use crate::js::expressions::static_member_expression::JsAnyStaticMemberLike;
use crate::prelude::*;
use rome_js_syntax::JsStaticMemberAssignment;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberAssignment;

impl FormatNodeRule<JsStaticMemberAssignment> for FormatJsStaticMemberAssignment {
    fn fmt_fields(&self, node: &JsStaticMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        JsAnyStaticMemberLike::from(node.clone()).fmt(f)
    }
}
