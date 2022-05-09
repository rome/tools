use crate::prelude::*;
use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;

impl FormatNodeFields<JsLiteralExportName> for FormatNodeRule<JsLiteralExportName> {
    fn format_fields(
        node: &JsLiteralExportName,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = node.as_fields();

        Ok(format_string_literal_token(value?, formatter))
    }
}
