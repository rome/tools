use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{TsIndexSignatureParameterName, TsIndexSignatureParameterNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsIndexSignatureParameterName;
impl FormatNodeRule<TsIndexSignatureParameterName> for FormatTsIndexSignatureParameterName {
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureParameterName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsIndexSignatureParameterNameFields { ident_token } = node.as_fields();
        write![f, [ident_token.format()]]
    }
}
