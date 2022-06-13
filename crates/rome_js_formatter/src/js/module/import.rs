use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsImport;
use rome_js_syntax::JsImportFields;

impl FormatNodeFields<JsImport> for FormatNodeRule<JsImport> {
    fn fmt_fields(node: &JsImport, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(import_token.format(), space_token(), import_clause.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
