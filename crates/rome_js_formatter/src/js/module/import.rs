use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsImport;
use rome_js_syntax::JsImportFields;

impl ToFormatElement for JsImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportFields {
            import_token,
            import_clause,
            semicolon_token,
        } = self.as_fields();

        let import_token = import_token.format(formatter)?;
        let import_clause = import_clause.format(formatter)?;

        format_with_semicolon(
            formatter,
            format_elements![import_token, space_token(), import_clause],
            semicolon_token,
        )
    }
}
