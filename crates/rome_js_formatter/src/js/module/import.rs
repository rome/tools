use crate::utils::format_with_semicolon;
use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsImport;
use rome_js_syntax::JsImportFields;

impl FormatNode for JsImport {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = self.as_fields();

        let import_token = import_token.format(formatter)?;
        let import_clause = import_clause.format(formatter)?;

        format_with_semicolon(
            formatter,
            formatted![formatter, import_token, space_token(), import_clause]?,
            semicolon_token,
        )
    }
}
