use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::TsExternalModuleDeclaration;
use rome_js_syntax::TsExternalModuleDeclarationFields;

impl FormatNodeFields<TsExternalModuleDeclaration> for FormatNodeRule<TsExternalModuleDeclaration> {
    fn fmt_fields(node: &TsExternalModuleDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = node.as_fields();

        write![
            f,
            [
                module_token.format(),
                space_token(),
                source.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
