use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsIdentifierAssignment;
use rome_js_syntax::JsIdentifierAssignmentFields;

impl FormatNodeFields<JsIdentifierAssignment> for FormatNodeRule<JsIdentifierAssignment> {
    fn fmt_fields(node: &JsIdentifierAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierAssignmentFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
