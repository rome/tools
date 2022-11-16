use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatStatementSemicolon;

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
            [
                import_token.format(),
                space(),
                import_clause.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
