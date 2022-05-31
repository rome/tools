use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsIdentifierAssignment;
use rome_js_syntax::JsIdentifierAssignmentFields;

impl FormatNodeFields<JsIdentifierAssignment> for FormatNodeRule<JsIdentifierAssignment> {
    fn format_fields(
        node: &JsIdentifierAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsIdentifierAssignmentFields { name_token } = node.as_fields();

        formatted![formatter, [name_token.format()]]
    }
}
