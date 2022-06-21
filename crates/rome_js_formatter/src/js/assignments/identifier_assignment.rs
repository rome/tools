use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsIdentifierAssignment;
use rome_js_syntax::JsIdentifierAssignmentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsIdentifierAssignment;

impl FormatNodeRule<JsIdentifierAssignment> for FormatJsIdentifierAssignment {
    fn fmt_fields(&self, node: &JsIdentifierAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierAssignmentFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
