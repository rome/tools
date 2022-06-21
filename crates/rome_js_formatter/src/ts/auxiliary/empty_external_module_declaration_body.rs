use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBody;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBodyFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsEmptyExternalModuleDeclarationBody;

impl FormatNodeRule<TsEmptyExternalModuleDeclarationBody>
    for FormatTsEmptyExternalModuleDeclarationBody
{
    fn fmt_fields(
        node: &TsEmptyExternalModuleDeclarationBody,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = node.as_fields();
        write![f, [semicolon_token.format()]]
    }
}
