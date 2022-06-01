use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsImportEqualsDeclaration;
use rome_js_syntax::TsImportEqualsDeclarationFields;

impl FormatNodeFields<TsImportEqualsDeclaration> for FormatNodeRule<TsImportEqualsDeclaration> {
    fn format_fields(
        node: &TsImportEqualsDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsImportEqualsDeclarationFields {
            import_token,
            type_token,
            id,
            eq_token,
            module_reference,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    import_token.format(),
                    space_token(),
                    type_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [token, space_token(),]]),
                    id.format(),
                    space_token(),
                    eq_token.format(),
                    space_token(),
                    module_reference.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
