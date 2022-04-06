use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;

impl ToFormatElement for JsLiteralExportName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = self.as_fields();

        Ok(format_string_literal_token(value?, formatter))
    }
}
