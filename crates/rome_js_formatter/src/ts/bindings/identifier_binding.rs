use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsIdentifierBinding, TsIdentifierBindingFields};

impl FormatNodeFields<TsIdentifierBinding> for FormatNodeRule<TsIdentifierBinding> {
    fn format_fields(
        node: &TsIdentifierBinding,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIdentifierBindingFields { name_token } = node.as_fields();

        formatted![formatter, [name_token.format()]]
    }
}
