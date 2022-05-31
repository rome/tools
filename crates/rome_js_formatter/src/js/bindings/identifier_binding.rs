use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsIdentifierBinding;
use rome_js_syntax::JsIdentifierBindingFields;

impl FormatNodeFields<JsIdentifierBinding> for FormatNodeRule<JsIdentifierBinding> {
    fn format_fields(
        node: &JsIdentifierBinding,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsIdentifierBindingFields { name_token } = node.as_fields();

        formatted![formatter, [name_token.format()]]
    }
}
