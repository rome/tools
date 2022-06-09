use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsImportEqualsDeclaration;
use rome_js_syntax::TsImportEqualsDeclarationFields;

impl FormatNodeFields<TsImportEqualsDeclaration> for FormatNodeRule<TsImportEqualsDeclaration> {
    fn fmt_fields(node: &TsImportEqualsDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportEqualsDeclarationFields {
            import_token,
            type_token,
            id,
            eq_token,
            module_reference,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    import_token.format(),
                    space_token(),
                    type_token
                        .format()
                        .with_or_empty(|token, f| write![f, [token, space_token()]]),
                    id.format(),
                    space_token(),
                    eq_token.format(),
                    space_token(),
                    module_reference.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
