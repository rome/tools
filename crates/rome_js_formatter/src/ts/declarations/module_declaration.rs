use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsModuleDeclaration;
use rome_js_syntax::TsModuleDeclarationFields;

impl FormatNodeFields<TsModuleDeclaration> for FormatNodeRule<TsModuleDeclaration> {
    fn format_fields(
        node: &TsModuleDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsModuleDeclarationFields {
            module_or_namespace,
            name,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [
                module_or_namespace.format(),
                space_token(),
                name.format(),
                space_token(),
                body.format(),
            ]
        ]
    }
}
