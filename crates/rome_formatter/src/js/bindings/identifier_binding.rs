use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsIdentifierBinding;
use rslint_syntax::JsIdentifierBindingFields;

impl ToFormatElement for JsIdentifierBinding {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierBindingFields { name_token } = self.as_fields();

        name_token.format(formatter)
    }
}
