use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsIdentifierBinding, TsIdentifierBindingFields};

impl FormatNodeFields<TsIdentifierBinding> for FormatNodeRule<TsIdentifierBinding> {
    fn fmt_fields(node: &TsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
