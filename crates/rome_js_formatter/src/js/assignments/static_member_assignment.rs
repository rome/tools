use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsStaticMemberAssignment;
use rome_js_syntax::JsStaticMemberAssignmentFields;

impl FormatNode for JsStaticMemberAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = self.as_fields();

        formatted![
            formatter,
            object.format(formatter)?,
            dot_token.format(formatter)?,
            member.format(formatter)?,
        ]
    }
}
