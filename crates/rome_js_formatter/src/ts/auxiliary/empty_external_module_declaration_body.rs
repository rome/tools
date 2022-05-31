use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBody;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBodyFields;

impl FormatNodeFields<TsEmptyExternalModuleDeclarationBody>
    for FormatNodeRule<TsEmptyExternalModuleDeclarationBody>
{
    fn format_fields(
        node: &TsEmptyExternalModuleDeclarationBody,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = node.as_fields();
        formatted![formatter, [semicolon_token.format()]]
    }
}
