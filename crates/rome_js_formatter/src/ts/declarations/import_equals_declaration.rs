use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::TsImportEqualsDeclaration;
use rome_js_syntax::TsImportEqualsDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportEqualsDeclaration;

impl FormatNodeRule<TsImportEqualsDeclaration> for FormatTsImportEqualsDeclaration {
    fn fmt_fields(
        &self,
        node: &TsImportEqualsDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
                    space(),
                    type_token
                        .format()
                        .with_or_empty(|token, f| write![f, [token, space()]]),
                    id.format(),
                    space(),
                    eq_token.format(),
                    space(),
                    module_reference.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
