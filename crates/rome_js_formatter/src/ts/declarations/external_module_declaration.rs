use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::TsExternalModuleDeclaration;
use rome_js_syntax::TsExternalModuleDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExternalModuleDeclaration;

impl FormatNodeRule<TsExternalModuleDeclaration> for FormatTsExternalModuleDeclaration {
    fn fmt_fields(
        &self,
        node: &TsExternalModuleDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = node.as_fields();

        write![
            f,
            [
                module_token.format(),
                space(),
                source.format(),
                space(),
                body.format()
            ]
        ]
    }
}
