use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsIdentifierBinding;
use rome_js_syntax::JsIdentifierBindingFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIdentifierBinding;

impl FormatNodeRule<JsIdentifierBinding> for FormatJsIdentifierBinding {
    fn fmt_fields(&self, node: &JsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
