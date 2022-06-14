use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsStaticMemberAssignment;
use rome_js_syntax::JsStaticMemberAssignmentFields;

impl FormatNodeFields<JsStaticMemberAssignment> for FormatNodeRule<JsStaticMemberAssignment> {
    fn fmt_fields(node: &JsStaticMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        write![f, [object.format(), dot_token.format(), member.format(),]]
    }
}
