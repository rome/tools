use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsIdentifierBinding;
use rome_js_syntax::JsIdentifierBindingFields;

impl FormatNodeFields<JsIdentifierBinding> for FormatNodeRule<JsIdentifierBinding> {
    fn fmt_fields(node: &JsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
