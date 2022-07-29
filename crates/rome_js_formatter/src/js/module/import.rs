use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsImport;
use rome_js_syntax::JsImportFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsImport;

impl FormatNodeRule<JsImport> for FormatJsImport {
    fn fmt_fields(&self, node: &JsImport, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(import_token.format(), space(), import_clause.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
