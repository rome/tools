use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsImport;
use rome_js_syntax::JsImportFields;

impl FormatNodeFields<JsImport> for FormatNodeRule<JsImport> {
    fn format_fields(node: &JsImport, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = node.as_fields();

        let import_token = import_token.format();
        let import_clause = import_clause.format();

        format_with_semicolon(
            formatter,
            formatted![formatter, [import_token, space_token(), import_clause]]?,
            semicolon_token,
        )
    }
}
