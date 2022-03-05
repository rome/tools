use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsIdentifierAssignment;
use rslint_syntax::JsIdentifierAssignmentFields;

impl ToFormatElement for JsIdentifierAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierAssignmentFields { name_token } = self.as_fields();

        name_token.format(formatter)
    }
}
