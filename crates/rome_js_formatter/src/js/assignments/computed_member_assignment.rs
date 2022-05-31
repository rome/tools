use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsComputedMemberAssignment;
use rome_js_syntax::JsComputedMemberAssignmentFields;

impl FormatNodeFields<JsComputedMemberAssignment> for FormatNodeRule<JsComputedMemberAssignment> {
    fn format_fields(
        node: &JsComputedMemberAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsComputedMemberAssignmentFields {
            object,
            l_brack_token,
            member,
            r_brack_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                object.format(),
                l_brack_token.format(),
                member.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
