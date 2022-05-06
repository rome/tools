use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsLiteralMemberNameFields;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

impl FormatNode for JsLiteralMemberName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralMemberNameFields { value } = self.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => Ok(format_string_literal_token(value, formatter)),
            _ => value.format(formatter),
        }
    }
}
