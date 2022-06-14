use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::TsGlobalDeclaration;
use rome_js_syntax::TsGlobalDeclarationFields;

impl FormatNodeFields<TsGlobalDeclaration> for FormatNodeRule<TsGlobalDeclaration> {
    fn fmt_fields(node: &TsGlobalDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsGlobalDeclarationFields { global_token, body } = node.as_fields();

        write![f, [global_token.format(), space_token(), body.format()]]
    }
}
