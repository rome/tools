use crate::prelude::*;
use crate::utils::format_string_literal_token;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;

impl FormatNode for JsLiteralExportName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = self.as_fields();

        Ok(format_string_literal_token(value?, formatter))
    }
}
