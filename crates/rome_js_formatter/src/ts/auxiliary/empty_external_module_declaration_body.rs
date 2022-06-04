use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBody;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBodyFields;

impl FormatNodeFields<TsEmptyExternalModuleDeclarationBody>
    for FormatNodeRule<TsEmptyExternalModuleDeclarationBody>
{
    fn fmt_fields(
        node: &TsEmptyExternalModuleDeclarationBody,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = node.as_fields();
        write![f, [semicolon_token.format()]]
    }
}
