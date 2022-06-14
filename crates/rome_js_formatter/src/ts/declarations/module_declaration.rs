use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsModuleDeclaration;
use rome_js_syntax::TsModuleDeclarationFields;

impl FormatNodeFields<TsModuleDeclaration> for FormatNodeRule<TsModuleDeclaration> {
    fn fmt_fields(node: &TsModuleDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsModuleDeclarationFields {
            module_or_namespace,
            name,
            body,
        } = node.as_fields();

        write![
            f,
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
