use crate::prelude::*;

use crate::js::expressions::computed_member_expression::JsAnyComputedMemberLike;
use rome_js_syntax::JsComputedMemberAssignment;

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
}
