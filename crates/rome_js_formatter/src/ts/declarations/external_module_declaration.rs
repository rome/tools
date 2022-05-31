use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsExternalModuleDeclaration;
use rome_js_syntax::TsExternalModuleDeclarationFields;

impl FormatNodeFields<TsExternalModuleDeclaration> for FormatNodeRule<TsExternalModuleDeclaration> {
    fn format_fields(
        node: &TsExternalModuleDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = node.as_fields();

        formatted![
            formatter,
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
