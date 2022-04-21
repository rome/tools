use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsIdentifierBinding;
use rome_js_syntax::JsIdentifierBindingFields;

impl FormatNode for JsIdentifierBinding {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierBindingFields { name_token } = self.as_fields();

        name_token.format(formatter)
    }
}
