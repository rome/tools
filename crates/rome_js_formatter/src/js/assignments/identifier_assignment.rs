use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsIdentifierAssignment;
use rome_js_syntax::JsIdentifierAssignmentFields;

impl FormatNode for JsIdentifierAssignment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierAssignmentFields { name_token } = self.as_fields();

        name_token.format(formatter)
    }
}
