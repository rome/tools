use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsComputedMemberAssignment;
use rome_js_syntax::JsComputedMemberAssignmentFields;

impl FormatNode for JsComputedMemberAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsComputedMemberAssignmentFields {
            object,
            l_brack_token,
            member,
            r_brack_token,
        } = self.as_fields();

        formatted![
            formatter,
            object.format(formatter)?,
            l_brack_token.format(formatter)?,
            member.format(formatter)?,
            r_brack_token.format(formatter)?,
        ]
    }
}
