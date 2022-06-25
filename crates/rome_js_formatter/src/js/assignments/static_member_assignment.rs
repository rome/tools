use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsStaticMemberAssignment;
use rome_js_syntax::JsStaticMemberAssignmentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberAssignment;

impl FormatNodeRule<JsStaticMemberAssignment> for FormatJsStaticMemberAssignment {
    fn fmt_fields(&self, node: &JsStaticMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        write![f, [object.format(), dot_token.format(), member.format(),]]
    }
}
