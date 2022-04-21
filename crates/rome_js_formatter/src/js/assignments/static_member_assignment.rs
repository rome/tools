use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsStaticMemberAssignment;
use rome_js_syntax::JsStaticMemberAssignmentFields;

impl FormatNode for JsStaticMemberAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = self.as_fields();

        Ok(format_elements![
            object.format(formatter)?,
            dot_token.format(formatter)?,
            member.format(formatter)?,
        ])
    }
}
