use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsIndexSignatureIdentifierBinding, TsIndexSignatureIdentifierBindingFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsIndexSignatureIdentifierBinding;

impl FormatNodeRule<TsIndexSignatureIdentifierBinding> for FormatTsIndexSignatureIdentifierBinding {
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureIdentifierBinding,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsIndexSignatureIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
