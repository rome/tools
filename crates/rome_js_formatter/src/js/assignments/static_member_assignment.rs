use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsStaticMemberAssignment;
use rome_js_syntax::JsStaticMemberAssignmentFields;

impl FormatNodeFields<JsStaticMemberAssignment> for FormatNodeRule<JsStaticMemberAssignment> {
    fn format_fields(
        node: &JsStaticMemberAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = node.as_fields();

        formatted![
            formatter,
            [object.format(), dot_token.format(), member.format(),]
        ]
    }
}
