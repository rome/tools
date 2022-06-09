use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsComputedMemberAssignment;
use rome_js_syntax::JsComputedMemberAssignmentFields;

impl FormatNodeFields<JsComputedMemberAssignment> for FormatNodeRule<JsComputedMemberAssignment> {
    fn fmt_fields(node: &JsComputedMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsComputedMemberAssignmentFields {
            object,
            l_brack_token,
            member,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                object.format(),
                l_brack_token.format(),
                member.format(),
                r_brack_token.format(),
            ]
        )
    }
}
