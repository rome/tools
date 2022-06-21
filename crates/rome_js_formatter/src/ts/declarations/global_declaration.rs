use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::TsGlobalDeclaration;
use rome_js_syntax::TsGlobalDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsGlobalDeclaration;

impl FormatNodeRule<TsGlobalDeclaration> for FormatTsGlobalDeclaration {
    fn fmt_fields(node: &TsGlobalDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsGlobalDeclarationFields { global_token, body } = node.as_fields();

        write![f, [global_token.format(), space_token(), body.format()]]
    }
}
