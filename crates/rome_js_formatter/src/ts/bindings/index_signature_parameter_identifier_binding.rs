use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{
    TsIndexSignatureParameterIdentifierBinding, TsIndexSignatureParameterIdentifierBindingFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsIndexSignatureParameterIdentifierBinding;
impl FormatNodeRule<TsIndexSignatureParameterIdentifierBinding>
    for FormatTsIndexSignatureParameterIdentifierBinding
{
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureParameterIdentifierBinding,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsIndexSignatureParameterIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
